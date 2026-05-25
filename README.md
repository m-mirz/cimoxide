# cimoxide

Rust tooling for CGMES CIM data: struct generation, SHACL validation, decoding, and protobuf conversion.

## Repository layout

| Crate | Description |
|---|---|
| `cimgen` | Code generator — reads ENTSO-E RDF/SHACL schemas and emits Rust source |
| `cimstructs` | **Generated.** Typed Rust structs for every CIM class |
| `cimvalidation` | **Generated.** One `Check*` function per SHACL constraint |
| `cimdecoder` | Deserialises CGMES RDF/XML into `cimstructs` types |

`cimstructs/` and `cimvalidation/` are fully generated — do not hand-edit them.

## Setup

Clone with submodules (the ENTSO-E RDF schema and SHACL files are required):

```bash
git clone --recurse-submodules <url>
# or after a plain clone:
git submodule update --init --recursive
```

## Commands

```bash
# Build everything
cargo build -v

# Run all tests (requires submodules)
cargo test

# Regenerate cimstructs and cimvalidation from schema files
cargo run -p cimgen
```

## Benchmarks

```bash
# Run all benchmarks
cargo bench -p cimdecoder

# Run a specific group (e.g. the import benchmark)
cargo bench -p cimdecoder --bench real_grid -- import
```

Benchmarks are in `cimdecoder/benches/real_grid.rs` and use the RealGrid test dataset
from the `CGMES-Test-Configurations` submodule (must be initialised). The `import`
group measures `decode_files_parallel` with real file paths, matching what
`cimoxide-cli import` does. Add debug symbols to get useful flamegraphs:

```toml
# Cargo.toml
[profile.bench]
debug = true
```

## Codegen stability tests

`cimgen/tests/codegen.rs` contains two hash-based tests that detect unintended drift in
the generated output — the same pattern used in `cimgo`:

- `cimstructs_codegen_stable` — runs the RDF struct generator and hashes `cimstructs/src/`
- `cimvalidation_codegen_stable` — runs the SHACL generator and hashes `cimvalidation/src/`

Each test regenerates into a temporary directory under `target/` and compares the
directory hash against a stored expected value. A mismatch means either the generator
logic or the schema files changed and the checked-in generated code needs to be updated.

### Bootstrapping the hashes (first run)

After the initial generation, or after any intentional schema/generator change, update
the stored hashes:

1. Run the tests and let them fail — the actual hash is printed in the failure message:
   ```bash
   cargo test -p cimgen --test codegen -- --nocapture
   ```
2. Copy the printed hash into the corresponding `assert_eq!` in `cimgen/tests/codegen.rs`.
3. Regenerate the checked-in files so they match:
   ```bash
   cargo run -p cimgen
   ```
4. Commit both the updated test hashes and the regenerated source files together.

### CI

`cargo test -p cimgen --test codegen` is the check to add to CI. It requires the
submodule to be checked out (schema files are needed to regenerate) and will fail if
the generator output has changed relative to the stored hash.
