# cimoxide on ReliCapGrid

Runs cimoxide against ENTSO-E's [ReliCapGrid](https://github.com/entsoe/relicapgrid), the
dataset of the 2026 SV-IOP (Network Code Profiles 2.5), with
[triplets](https://pypi.org/project/triplets/) as the reference: ReliCapGrid's own CI validates
with it. File groups and the triplets validation are imported from ReliCapGrid's
`buildScripts/validate_instances.py`, so both tools see the same files under the same policies.

## Setup

ReliCapGrid is cimoxide's `relicapgrid` submodule, pinned to the commit the results here are for.
It is not checked out by `git submodule update --init --recursive`; fetch it explicitly:

```bash
git submodule update --init --checkout relicapgrid    # entsoe/relicapgrid, cgmes-3.0_ncp-2.5_tc-2.0
cd scripts/relicapgrid
python3 -m venv .venv
.venv/bin/pip install 'triplets[polars,oxigraph,validation]==0.2.0' 'pandas!=2.3.3' pytest maturin
(cd ../../cimoxide-py && VIRTUAL_ENV=$PWD/../scripts/relicapgrid/.venv \
    PATH=$PWD/../scripts/relicapgrid/.venv/bin:$PATH maturin develop --release)
```

`RELICAPGRID_DIR` points the harness at another checkout instead. `KNOWN`, `TRIAGE`, the edits and
mutations name mRIDs and files of the pinned commit; moving the submodule means reviewing them.

The APL is cimoxide's `application-profiles-library` submodule (override with
`CIMOXIDE_APL_DIR`); it carries the CGMES 3.0 and NCP 2.5 that ReliCapGrid's CI uses.

## Run

```bash
.venv/bin/python -m pytest -q                 # all tests; skipped without a ReliCapGrid checkout
.venv/bin/python harness.py --report out/     # import_diff.csv, validation_diff.csv, summary.md
```

triplets results are cached per dataset commit in `.cache/`.

| Test | Checks |
|---|---|
| `test_import.py` | every file decoded by cimoxide (`entries()` and `foreign()`) equals triplets' triples |
| `test_export.py` | each CGMES and NC file's profile export (`to_xml_for_profile`), and each file's `to_xml()`, is the same RDF graph as the source |
| `test_validation.py` | every finding the tools disagree on has a cause in `harness.TRIAGE` |
| `test_use_cases.py` | the report's 22 use cases (Annex 1), on what cimoxide decodes, each also on broken data |
| `test_edits.py` | CO_UC1's and RA_UC4's edits through cimoxide-py, exported and validated in place of ReliCapGrid's files |
| `test_mutations.py` | broken copies (report Table 2, §5.2, §13.1): which rules each tool newly reports |

A use case's data or documentation problems are listed in `test_use_cases.KNOWN`; a validation
disagreement's cause in `harness.TRIAGE`. A new entry in either is a new difference to look at.

## Policies taken from ReliCapGrid

- Boundary files declare no application profile: their `dcat:Dataset` header's
  `dcterms:conformsTo` names vocabularies only (`CIM100#`, `nc/2.4#`). ReliCapGrid's CI
  validates them as `CoreEquipment-EU/3.0`; the harness gives cimoxide copies whose header
  adds that profile to `conformsTo` (and `EquipmentBoundary-EU/3.0` for the boundary group).
- `Instance/Jotunheim/GridSituation/` duplicates `Jotunheim/NetworkCode/` and is skipped.
- Findings are compared as `(object, rule local name)`; two shapes sharing a local name
  (e.g. `eq:` and `equ:ACDCConverter.PccTerminal-valueType`) compare as one.
