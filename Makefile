SHACL_GLOB := application-profiles-library/CGMES/CurrentRelease/SHACL/TTL/*.ttl

.PHONY: all generate build test clean

all: generate build test

generate:
	mkdir -p cimstructs/src cimvalidation/src
	touch cimstructs/src/lib.rs cimvalidation/src/lib.rs
	cargo run -p cimgen -- \
		--output cimstructs/src \
		--shacl "$(SHACL_GLOB)" \
		--shacl-output cimvalidation/src

build:
	cargo build --workspace

test:
	cargo test --workspace

clean:
	cargo clean
	rm -rf cimstructs/src cimvalidation/src
