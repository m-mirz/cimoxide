SHACL_GLOB := application-profiles-library/CGMES/SHACL/*.ttl

.PHONY: all generate build test clean python-dev python-build

all: generate build test

generate:
	mkdir -p cimmodel/src/generated cimvalidation/src cimoxide-py/python/cimoxide
	cargo run -p cimoxide-gen

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
	rm -rf cimmodel/src/generated
	rm -f cimvalidation/src/cgmes_shapes.rs cimvalidation/src/cgmes_profiles.rs cimvalidation/src/nc_shapes.rs cimvalidation/src/nc_profiles.rs
