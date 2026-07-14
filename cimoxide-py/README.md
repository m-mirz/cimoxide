# cimoxide

Python bindings for [cimoxide](https://github.com/m-mirz/cimoxide), a Rust toolkit for
ENTSO-E CGMES (Common Grid Model Exchange Standard) power system data. This package wraps
the Rust decoder and SHACL/SPARQL validator (via [PyO3](https://pyo3.rs)) to give you fast
RDF/XML parsing and CGMES conformance validation from Python, with no Rust toolchain
required at install time.

## Install

```bash
pip install cimoxide
```

Prebuilt wheels are published for common platforms; if none match your environment, `pip`
will need a Rust toolchain and [`maturin`](https://www.maturin.rs) to build from source.

## Quick start

```python
import cimoxide

# Parse one or more CGMES RDF/XML files into a single merged dataset.
ds = cimoxide.decode_files([
    "RealGrid_EQ.xml",
    "RealGrid_SSH.xml",
    "RealGrid_TP.xml",
    "RealGrid_SV.xml",
])

len(ds)                       # total number of elements
ds.by_type()                  # {"ACLineSegment": [mrid, ...], ...} — no deserialization
ds.get_type("ACLineSegment")  # [{"_type": "ACLineSegment", "r": 0.12, ...}, ...]

for mrid in ds:
    obj = ds[mrid]             # dict, e.g. {"_type": "BusbarSection", "name": "...", ...}
```

Each element is a plain Python `dict` with a `"_type"` key (the CIM class name) plus one
key per populated attribute, snake_case, matching the JSON serialization of the underlying
Rust structs. Reference fields (MRID associations) are plain MRID strings.

### Modify and re-encode

`CimDataset` supports dict-style assignment and deletion, so you can edit elements in place
and write the result back out as CGMES profile XML:

```python
# Edit an existing element (read, mutate the dict, assign it back).
line = ds["ACLineSegment.1"]
line["r"] = 0.15
ds["ACLineSegment.1"] = line

# Add a brand-new element the same way — the "_type" key selects the CIM class.
ds["BaseVoltage.NEW"] = {"_type": "BaseVoltage", "id": "BaseVoltage.NEW", "nominal_voltage": 110.0}

# Remove one.
del ds["ACLineSegment.2"]

# Encode a single profile as an RDF/XML string.
eq_xml = ds.to_xml_for_profile("EQ")

# Or write a full profile set straight to a directory: dir/EQ.xml, dir/SSH.xml, ...
ds.write_xml_files("out/", ["EQ", "SSH", "TP", "SV"])
```

`to_xml_for_profile`/`write_xml_files` only emit elements and fields whose CIM schema
origin includes the requested profile. If the dataset still has the decoded `FullModel`
header for that profile (from the original source file), it's reused verbatim
(`scenarioTime`, `modelingAuthoritySet`, `version`, `DependentOn`, ...); otherwise a minimal
header is synthesized.

### Validation

```python
violations = cimoxide.validate_files(
    ["RealGrid_EQ.xml", "RealGrid_SSH.xml"],
    profiles=["EQ", "SSH"],   # optional; auto-detected if omitted
)

for v in violations:
    print(v.severity, v.rule_id, v.message, v.object_id)
```

`validate_files` runs two-phase validation: per-profile SHACL/SPARQL checks against each
file individually, then cross-profile checks on the merged dataset. See the
[`validate_files` docstring](python/cimoxide/types.pyi) for the full parameter list
(`solved`, `common`, `quality`, `silence`).

## API surface

| Function / method | Description |
|---|---|
| `cimoxide.decode_file(path)` | Parse a single RDF/XML file. |
| `cimoxide.decode_files(paths)` | Parse and merge multiple RDF/XML files. |
| `cimoxide.decode_str(content)` | Parse RDF/XML from a string. |
| `cimoxide.validate_files(paths, ...)` | Two-phase SHACL/SPARQL validation, returns `list[Violation]`. |
| `CimDataset.merge(other)` | Merge another dataset into this one (`other` becomes empty). |
| `CimDataset.drop_blocks()` | Free internal parse buffers after the final merge. |
| `CimDataset[mrid]` / `.get(mrid)` | Fetch one element as a dict (`KeyError` / `None` if missing). |
| `CimDataset[mrid] = {...}` | Insert or replace the element at `mrid`. |
| `del CimDataset[mrid]` | Remove the element at `mrid` (`KeyError` if missing). |
| `CimDataset.mrids()` / `iter(ds)` / `len(ds)` | Enumerate or count MRIDs. |
| `CimDataset.by_type()` | `dict[str, list[mrid]]` type index, no deserialization. |
| `CimDataset.get_type(name)` | All element dicts for one CIM class. |
| `CimDataset.entries()` | All entries as `dict[mrid, dict]` (deserializes everything). |
| `CimDataset.to_xml_for_profile(profile)` | Encode one CGMES profile (e.g. `"EQ"`) as an RDF/XML string. |
| `CimDataset.write_xml_files(dir, profiles)` | Write one RDF/XML file per profile into `dir`. |

Full type stubs with per-method docstrings are in
[`python/cimoxide/__init__.pyi`](python/cimoxide/__init__.pyi) and
[`python/cimoxide/types.pyi`](python/cimoxide/types.pyi) (generated `TypedDict` per CIM
class, for editor autocomplete on the returned dicts).

## Examples

[`examples/example_counts.py`](examples/example_counts.py) decodes the RealGrid test
configuration and prints an element count per CIM type:

```bash
python examples/example_counts.py
```

[`examples/example_encode.py`](examples/example_encode.py) decodes RealGrid, encodes it
back to `EQ`/`SSH`/`TP`/`SV` profile files (in a temp directory by default, or the directory
given as an argument), then re-decodes the output to confirm the round-trip is lossless:

```bash
python examples/example_encode.py [output-dir]
```

Both require the `CGMES-Test-Configurations` submodule checked out at the repo root — see
"Development" below.

## Benchmark

[`examples/benchmark_realgrid.py`](examples/benchmark_realgrid.py) times `decode_files`,
`write_xml_files`, and `validate_files` against the full RealGrid dataset and reports
best/mean wall time plus MB/s throughput for each:

```bash
python examples/benchmark_realgrid.py [iterations]   # default: 3
```

Also requires the `CGMES-Test-Configurations` submodule.

## Tests

The test suite decodes and validates the CGMES fixture files checked into the parent
repository's `testdata/` directory:

```bash
pip install pytest
pytest tests/
```

- `tests/test_decode.py` — round-trip decode tests (`decode_file`/`decode_str`/`decode_files`,
  indexing, iteration).
- `tests/test_api.py` — dataset API contract tests (`merge`, `drop_blocks`, mutation via
  `__setitem__`/`__delitem__`, error handling).
- `tests/test_encode.py` — `to_xml_for_profile`/`write_xml_files` behavior, including
  `FullModel` header reuse against a real CGMES fixture.
- `tests/test_validate.py` — `validate_files` behavior (profile filtering, `silence`,
  `quality`/`common` flags, `Violation` fields).

## Development

This package is built from the [`cimoxide`](https://github.com/m-mirz/cimoxide) monorepo,
where `cimoxide-py` lives alongside the Rust crates it binds (`cimdecoder`, `cimstructs`,
`cimvalidation`, `cimconvert`). To build it from source:

```bash
# for ubuntu
git clone --recurse-submodules https://github.com/m-mirz/cimoxide.git
cd cimoxide
python3 -m venv .venv
source .venv/bin/activate
pip3 install maturin
cd cimoxide-py
maturin develop --release   # editable install into the active virtualenv
pip3 install pytest
pytest tests/
```

See the [repository README](https://github.com/m-mirz/cimoxide#readme) for the full
project layout, the code generator, and the Rust CLI.

## License

Apache-2.0
