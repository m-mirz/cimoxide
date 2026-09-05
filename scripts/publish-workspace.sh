#!/usr/bin/env bash
# Publish every workspace crate to crates.io, tolerating the registry's rate limits.
#
# crates.io meters publishes per user account: a burst of 5 brand-new crate names and
# then 1 per 10 minutes, a burst of 30 new versions of existing crates and then 1 per
# minute (https://crates.io/docs/rate-limits). This workspace has 8 crates, so the
# first publish of a new name set always stalls part-way through and `cargo publish
# --workspace` aborts with whatever already landed left on the registry.
#
# Each attempt below asks crates.io which name@version pairs already exist, excludes
# them, and publishes only the remainder. A 429 is not a failure: the error carries the
# time the bucket refills, so we sleep until then and try what is left again.
set -euo pipefail

dry_run=false
[ "${1:-}" = "--dry-run" ] && dry_run=true

api=${CRATES_API:-https://crates.io/api/v1/crates}
user_agent=${PUBLISH_USER_AGENT:-cimoxide-release (+https://github.com/m-mirz/cimoxide)}
max_attempts=${PUBLISH_MAX_ATTEMPTS:-12}
fallback_wait=${PUBLISH_FALLBACK_WAIT:-620}
log=$(mktemp)
trap 'rm -f "$log"' EXIT

# name<TAB>version for every publishable workspace member.
mapfile -t members < <(
  cargo metadata --no-deps --format-version 1 \
    | jq -r '.packages[] | select(.publish != []) | "\(.name)\t\(.version)"' \
    | sort
)

is_published() { # name version -> 0 when that exact version is on crates.io
  curl -sS -m 30 -H "User-Agent: $user_agent" "$api/$1/$2" \
    | jq -e 'has("version")' >/dev/null 2>&1
}

# Seconds to wait after a 429, taken from the "try again after <RFC3339>" the registry
# puts in the error body; falls back to one refill interval when it cannot be parsed.
wait_seconds() {
  local stamp until now
  stamp=$(grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]+)?(Z|[+-][0-9:]+)?' "$log" | tail -1)
  if [ -n "$stamp" ] && until=$(date -u -d "$stamp" +%s 2>/dev/null); then
    now=$(date -u +%s)
    if [ "$until" -gt "$now" ]; then
      echo $((until - now + 15))
      return
    fi
    echo 30
    return
  fi
  echo "$fallback_wait"
}

# Excluding what is already on crates.io also keeps `--dry-run` usable: cargo fails a
# dry run outright once a version exists on the registry (rust-lang/cargo#14789).
survey() {
  excludes=()
  remaining=()
  for member in "${members[@]}"; do
    name=${member%%$'\t'*}
    version=${member##*$'\t'}
    if is_published "$name" "$version"; then
      excludes+=(--exclude "$name")
    else
      remaining+=("$name@$version")
    fi
  done
}

if [ "$dry_run" = true ]; then
  survey
  if [ ${#remaining[@]} -eq 0 ]; then
    echo "All ${#members[@]} workspace crates are already on crates.io; nothing to dry-run."
    exit 0
  fi
  echo "Dry-running: ${remaining[*]}"
  exec cargo publish --workspace --dry-run --allow-dirty "${excludes[@]}"
fi

for ((attempt = 1; attempt <= max_attempts; attempt++)); do
  survey

  if [ ${#remaining[@]} -eq 0 ]; then
    echo "All ${#members[@]} workspace crates are on crates.io."
    exit 0
  fi

  echo "Attempt $attempt/$max_attempts — publishing: ${remaining[*]}"
  if cargo publish --workspace --allow-dirty "${excludes[@]}" 2>&1 | tee "$log"; then
    continue  # re-check on the next pass so the loop exits via the "all published" arm
  fi

  if ! grep -qiE 'too many (new )?crates|429' "$log"; then
    echo "Publish failed for a reason other than the crates.io rate limit." >&2
    exit 1
  fi

  sleep_for=$(wait_seconds)
  echo "Rate limited by crates.io; sleeping ${sleep_for}s before retrying the rest."
  sleep "$sleep_for"
done

echo "Gave up after $max_attempts attempts; still unpublished: ${remaining[*]}" >&2
exit 1
