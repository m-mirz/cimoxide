SHACL_GLOB := application-profiles-library/CGMES/SHACL/*.ttl

.PHONY: all generate build test clean python-dev python-build

all: generate build test

generate:
	mkdir -p cimmodel/src cimvalidation/src cimoxide-py/python/cimoxide
	touch cimmodel/src/lib.rs
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
	find cimmodel/src -name '*.rs' ! -name 'base.rs' ! -name 'schema_source.rs' ! -name 'decode.rs' ! -name 'convert.rs' -delete
	rm -f cimvalidation/src/cgmes_shapes.rs cimvalidation/src/nc_shapes.rs cimvalidation/src/nc_profiles.rs
