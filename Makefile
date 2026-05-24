SHACL_GLOB := application-profiles-library/CGMES/CurrentRelease/SHACL/TTL/*.ttl

.PHONY: all generate build test clean python-dev python-build

all: generate build test

generate:
	mkdir -p cimstructs/src cimvalidation/src cimoxide-py/python/cimoxide
	touch cimstructs/src/lib.rs cimvalidation/src/lib.rs
	cargo run -p cimgen -- \
		--output cimstructs/src \
		--shacl "$(SHACL_GLOB)" \
		--shacl-output cimvalidation/src \
		--python-stubs-output cimoxide-py/python/cimoxide

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
	rm -rf cimstructs/src cimvalidation/src
