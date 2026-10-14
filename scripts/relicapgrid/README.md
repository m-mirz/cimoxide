# cimoxide on ReliCapGrid

Runs cimoxide against ENTSO-E's [ReliCapGrid](https://github.com/entsoe/relicapgrid), the
dataset of the 2026 SV-IOP (Network Code Profiles 2.5), with
[triplets](https://pypi.org/project/triplets/) as the reference: ReliCapGrid's own CI validates
with it. File groups and the triplets validation are imported from ReliCapGrid's
`buildScripts/validate_instances.py`, so both tools see the same files under the same policies.

## Setup

```bash
git clone https://github.com/entsoe/relicapgrid.git ../relicapgrid   # default branch: cgmes-3.0_ncp-2.5_tc-2.0
cd scripts/relicapgrid
python3 -m venv .venv
.venv/bin/pip install 'triplets[polars,oxigraph,validation]==0.2.0' 'pandas!=2.3.3' pytest maturin
(cd ../../cimoxide-py && VIRTUAL_ENV=$PWD/../scripts/relicapgrid/.venv \
    PATH=$PWD/../scripts/relicapgrid/.venv/bin:$PATH maturin develop --release)
export RELICAPGRID_DIR=$PWD/../../../relicapgrid
```

The APL is cimoxide's `application-profiles-library` submodule (override with
`CIMOXIDE_APL_DIR`); it carries the CGMES 3.0 and NCP 2.5 that ReliCapGrid's CI uses.

## Run

```bash
.venv/bin/python -m pytest -q                 # all tests; skipped without RELICAPGRID_DIR
.venv/bin/python harness.py --report out/     # import_diff.csv, validation_diff.csv, summary.md
```

triplets results are cached per dataset commit in `.cache/`.

| Test | Checks |
|---|---|
| `test_import.py` | every file decoded by cimoxide equals triplets' triples, up to known differences |
| `test_export.py` | each CGMES file round-trips through `to_xml_for_profile`; NC export is `xfail` |
| `test_validation.py` | every finding the tools disagree on has a cause in `harness.TRIAGE` |
| `test_use_cases.py` | the report's 22 use cases (Annex 1), on what cimoxide decodes, each also on broken data |
| `test_mutations.py` | broken copies (report Table 2, §5.2, §13.1): which rules each tool newly reports |

A use case's data or documentation problems are listed in `test_use_cases.KNOWN`; a validation
disagreement's cause in `harness.TRIAGE`. A new entry in either is a new difference to look at.

## Policies taken from ReliCapGrid

- Boundary files declare no application profile; ReliCapGrid's CI validates them as
  `CoreEquipment-EU/3.0`. cimoxide reads CGMES profiles from `md:Model.profile` only, so the
  harness validates copies with an `md:FullModel` header declaring it (and
  `EquipmentBoundary-EU/3.0` for the boundary group).
- `Instance/Jotunheim/GridSituation/` duplicates `Jotunheim/NetworkCode/` and is skipped.
- Findings are compared as `(object, rule local name)`; two shapes sharing a local name
  (e.g. `eq:` and `equ:ACDCConverter.PccTerminal-valueType`) compare as one.
