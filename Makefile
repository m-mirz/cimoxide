SHACL_GLOB := application-profiles-library/CGMES/CurrentRelease/SHACL/TTL/*.ttl

.PHONY: all generate build test clean python-dev python-build

all: generate build test

generate:
	mkdir -p cimstructs/src cimvalidation/src cimoxide-py/python/cimoxide
	cargo run -p cimgen

build:
	cargo build --workspace

test:
	cargo test --workspace

python-dev:
	cd cimoxide-py && maturin develop --release

python-build:
	cd cimoxide-py && maturin build --release

clean:
	cargo clean
	find cimstructs/src -name '*.rs' ! -name 'base.rs' -delete
	rm -f cimvalidation/src/generated_*.rs cimvalidation/src/generated_lib.rs
