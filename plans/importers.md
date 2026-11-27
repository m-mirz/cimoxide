# Importers: other grid formats into CGMES

Plan, 2026-11-22. Converters that turn MATPOWER, pandapower and PowerFactory DGS files into
CGMES, so that cimlens, the cimoxide CLI, cimmcp and gridoxide (through its CGMES importer) can
all open them. MATPOWER goes first.

## Why here, not in gridoxide

- cimlens works on the CIM model. The graph, the diagrams, edit mode, difference models and
  saving all run on `cimmodel` elements. gridoxide's importers (`src/matpower.rs`, `iidm/`,
  `ucte.rs`, `pgm/`) build its solver's bus-branch `Network`, which none of these can use.
- gridoxide depends on `cimoxide-model`, so a converter that outputs CIM belongs below it. That
  also means it can't call gridoxide. The MATPOWER parser (`gridoxide::matpower::parse`, the
  part that only reads the file) is copied over, not shared.
- Every cimoxide front end gets the converters at once: `cimox`, cimmcp, the VS Code extension
  and cimlens-wasm.

## Shape

A new workspace crate, `cimimport/`, package `cimoxide-import`, lib `cimimport`, re-exported
by the `cimoxide` facade.

```
cimimport/src/
  lib.rs        Converted { files: Vec<(String, String)>, notes: Vec<Note> }; detect(name, text)
  build.rs      shared builder: mRIDs, BaseVoltage, Substation/VoltageLevel, bus, terminal,
                limits, regions; fills a CimDataset and writes it per profile
  matpower/
    parse.rs    from gridoxide's matpower::parse (MatpowerCase), plus bus_name/genfuel if present
    map.rs      MatpowerCase → build calls
```

- It takes text and returns texts, with no filesystem access, so it builds for wasm.
- Output: one CimDataset written by `dataset_to_xml_for_profile` as `EQ`, `SSH`, `TP` and
  `SV`, each with its FullModel header and the `dependentOn` chain
  SV → TP → SSH → EQ.
- `notes`: what the conversion invented, assumed or dropped, such as "buses 3 and 7 grouped into
  one substation by transformer" or "gencost not mapped". Front ends show them.
- mRIDs are deterministic: UUID v5 over a namespace, the case name and the source key (`bus:14`,
  `gen:3`, `branch:4-7:2`, the second parallel branch between buses 4 and 7). Converting the same
  case again gives the same ids, so saved DL layouts, difference models and edits still match.
  MATPOWER branches have no ids, so reordering the branch rows changes their mRIDs. That
  limitation goes in the docs.

## MATPOWER → CGMES 3.0 mapping

| MATPOWER | CGMES | Notes |
|---|---|---|
| `baseKV` of each bus | `BaseVoltage` | one per distinct value |
| buses joined by transformers, and by name (see [Substation grouping](#substation-grouping)) | `Substation`, one `VoltageLevel` per baseKV in it | gives cimlens a substation view; noted |
| bus `area` | `SubGeographicalRegion` (one `GeographicalRegion`) | substations go under it |
| bus | `BusbarSection` + `ConnectivityNode` (EQ), `TopologicalNode` (TP) | name from `bus_name`, else `Bus {i}` |
| each branch end | a `Breaker` between the busbar's `ConnectivityNode` and one of its own, which the branch's terminal connects to | see [Breakers](#breakers); both nodes in the bus's `TopologicalNode` |
| bus type 3 | `SynchronousMachine.referencePriority = 1` on its machine | if it has no online generator: first PV bus, as `bustypes` does |
| bus type 4 (isolated) | the bus is kept; its equipment's terminals get `connected = false` | visible in cimlens, not solved |
| branch, tap 0, shift 0, same kV | `ACLineSegment` | r, x, b converted from pu to Ω/S on its baseKV; b is `bch`, the full charging |
| branch with tap ≠ 0, shift ≠ 0 or different kV | `PowerTransformer` + 2 `PowerTransformerEnd` | end 1 `ratedU = tap·baseKV_from`, end 2 `ratedU = baseKV_to` with r, x, b in Ω/S on end 2, which is MATPOWER's model: an ideal tap at the from side, then the series impedance |
| `shift` | `PhaseTapChangerLinear`, fixed at one step giving the angle | |
| `rateA` / `rateB` / `rateC` | `OperationalLimitSet` per terminal: `CurrentLimit` PATL from rateA, TATL from rateB/C | MVA → A at baseKV; 0 means unlimited, so no limit |
| `BR_STATUS = 0` | both end breakers `Switch.open = true` (SSH); terminals stay connected | kept, shown as out, closed again in cimlens with a click |
| gen | `SynchronousMachine` + `ThermalGeneratingUnit` (from `genfuel` where given: hydro, wind, solar, nuclear) + `RegulatingControl` (voltage, target `Vg·baseKV` at its bus) | SSH `p = −Pg`, `q = −Qg`; minQ/maxQ, `GeneratingUnit.minOperatingP/maxOperatingP` from Pmin/Pmax |
| `GEN_STATUS ≤ 0` | `Equipment.inService = false` and terminal disconnected | |
| bus `Pd`, `Qd` | `EnergyConsumer` | only if non-zero |
| bus `Gs`, `Bs` | `LinearShuntCompensator`, one section | `bPerSection = Bs / baseKV²` (MVAr at 1 pu → S) |
| bus `Vm`, `Va` | `SvVoltage` per `TopologicalNode` (SV) | always written, so the case's voltages are not lost; see [SV](#sv) |
| branch `PF`, `QF`, `PT`, `QT` (columns 14–17, in solved cases) | `SvPowerFlow` on the branch's two terminals | only if the columns are there |
| gen `Pg`, `Qg` | `SvPowerFlow` on the machine's terminal | |
| breakers, branches, machines | `SvStatus.inService` | from the status columns |
| `gencost`, `dcline`, OPF limits | not mapped (noted) | `dcline` maybe later as `EquivalentInjection` pairs |

### Substation grouping

A union-find over the buses, in two passes:

1. **By transformer:** two buses are joined by any branch with tap ≠ 0, shift ≠ 0 or different
   baseKV.
2. **By name**, only when the case has `bus_name`: two buses whose names are equal once a trailing
   section number is stripped (`MASON CITY 3` → `MASON CITY`, `Olive V2` → `Olive`) are joined,
   but only if a branch also connects them. The branch check stops unrelated buses that happen to
   share a name from merging. Only the last token is stripped, so `PEORIA 8` and `PEORIA 9` stay
   two stations.

The example is `case_ACTIVSg200`. Mason City's buses 17 to 21 are all at 115 kV, and buses 18 to
21 each hang off bus 17 through a short line (x ≈ 0.003–0.004 pu, about 0.5 Ω). Bus 17's two
lines out of the station have x ≈ 0.07 pu. By transformer alone, Mason City is five substations;
by name, one. Over the whole case that is 134 substations by transformer alone and 111 with names.
Lovington, Bethany and Peoria 8/9 have the same shape. `DECATUR 3` is joined by both passes: a
230/115 kV transformer, plus a short line to a third bus. In `case118` (`Olive V1`/`Olive V2`) the
names add nothing, because the transformer pass already joins those buses.

What this changes in cimlens: the short lines become lines inside a substation. They disappear
from the grid view and show only inside the substation. Each such line is listed in the notes
("4 lines inside MASON CITY"). Grouping by name is on by default, with an option to turn it off.

Of the cases in benchmark-grids, only `case14`, `case118` and `case_ACTIVSg200` have bus names.
The RTE and PEGASE cases don't, so they are grouped by transformer alone.

### Breakers

Every branch end gets a `Breaker` in its bus's voltage level, on by default (`--no-breakers` turns
them off). The busbar's `ConnectivityNode` sits on one side, the branch terminal's own on the
other, and both belong to the bus's `TopologicalNode`, so the bus-branch TP doesn't change. This
gives:

- cimlens's substation view a bay per line and transformer to draw;
- N-1 and what-if something to switch;
- a switch to show an out-of-service branch as open (`BR_STATUS = 0`) instead of
  disconnecting its terminals;
- gridoxide's `NodeBreakerFromEq` something to rebuild buses from, checked in phase 2.

Names and mRIDs follow the branch: `<branch name> CB <bus>`, mRID from `breaker:<branch key>:<bus>`.
Generators, loads and shunts connect straight to the busbar: MATPOWER has no status for loads and
shunts, and a generator's status is `inService`. They could be added later the same way if
switching them turns out to be useful.

### SV

The SV is always written. It holds `SvVoltage` from each bus's `Vm`/`Va`, `SvPowerFlow` from the
branch flow columns where a case has them and from each generator's `Pg`/`Qg`, and `SvStatus`.
This keeps what the case says about its state; without it, cimlens would show the case unsolved
until it is solved again.

A case's values are a solved state only if it was saved after a power flow. Many are, but some
hold flat starts (`Vm = 1`, `Va = 0`). The converter notes a flat start ("SV is a flat start, not
a solved state") when every `Vm` is 1 and every `Va` is 0. It doesn't solve anything: cimlens and
gridoxide do that.

To check in phase 1, against gridoxide's CGMES importer (`src/cgmes.rs`):

- A MATPOWER transformer's charging is split b/2 on each side (pi model), while a CGMES end
  carries one b. Is putting it all on end 2 close enough? If not, use an `EquivalentBranch`.
- Phase shifter sign conventions.
- Whether the slack bus survives gridoxide's choice of reference (`referencePriority`).

## Phases

One PR each.

### 1. The crate and the converter (cimoxide)

- `cimimport` with `build.rs` and `matpower/`. The parser is copied from gridoxide with its
  tests.
- Test data: [benchmark-grids](https://github.com/m-mirz/benchmark-grids) as a submodule at
  `benchmark-grids/`, next to `CGMES-Test-Configurations/`. It is the same repo, pinned the same
  way, that gridoxide has at `tests/data/benchmark-grids`, so both sides of the cross-check read
  the same files. It is 29 MB and checked out by default, like the other test submodules. No case
  files are copied: MATPOWER's licence covers its code but not its case files (see benchmark-grids'
  `PROVENANCE.md`).
  - Unit tests: `matpower/case14.m`, `case118.m` and `case300.m`; `pglib/pglib_opf_case30_ieee.m`
    (adds rateA limits and a richer gen table); and two radial feeders from `matpower-plain/`
    (`case33bw.m`, `case69.m`). The plain variants matter because the parser does not run the
    MATLAB code that converts units in the originals.
  - Grouping: `case_ACTIVSg200.m`. Expect 111 substations, Mason City as one substation holding
    buses 17 to 21, and Peoria 8 and Peoria 9 kept apart.
  - Size checks: `case2869pegase.m` and `case6515rte.m`, for speed (no bus names, so they are
    grouped by transformer only).
- Tests:
  - the output decodes again;
  - `cimvalidation` reports no errors on EQ/SSH/TP;
  - converting twice gives identical files (stable ids and order);
  - element counts match the case;
  - pu → Ω unit checks on case14 by hand;
  - every branch end has a closed breaker (open where `BR_STATUS = 0`), and the TP still has one
    `TopologicalNode` per bus;
  - the SV matches the case's `Vm`/`Va` exactly.

### 2. The cross-check (gridoxide)

- `tests/matpower_via_cgmes.rs`, with a git dev-dependency on `cimoxide-import`, pinned the way
  cimlens pins gridoxide. Every transmission case in `tests/data/benchmark-grids/matpower/` (14 to
  9,241 buses) and the `matpower-plain/` feeders are solved twice: directly through
  `matpower::read`, and converted and then read through
  `cgmes_to_network_with(.., BusBranchFromTp)` and `NodeBreakerFromEq`. Through the breakers,
  the second rebuilds the buses that the first takes from the TP. The largest cases can sit behind `#[ignore]` if
  they slow CI down.
- The converted SV against gridoxide's solve: equal for cases saved solved, and a measure of how
  far from solved the others are (reported, not failed).
- Pass: max |ΔV| < 1e-6 pu and max |Δθ| < 1e-6 rad per bus, and the same convergence.
- This is where the mapping table's open points get settled. A deviation found here is fixed in
  the converter, not in gridoxide.

### 3. CLI and MCP (cimoxide)

- `cimox convert case14.m -o out/` writes `case14_EQ.xml` and the other profiles; `--zip`,
  `--no-breakers`, `--no-name-grouping`.
- cimmcp: a `convert` tool, or its model-set loader accepting `.m` directly. Decide in the PR.

### 4. cimlens

- DropZone accepts `.m` (`web/src/views/DropZone.svelte`, `web/src/zip.ts`). cimlens-wasm
  converts it, and the profiles enter the model set as `case14.m/EQ.xml` etc., the way zipped
  files appear.
- A banner on the model: converted from MATPOWER, with the notes.
- Saving writes CGMES. Layout comes from the automatic grid layout, since MATPOWER has no
  coordinates.
- The demo gets a MATPOWER case next to ReliCapGrid's.

### 5. Later formats, each its own plan when started

- **pandapower** JSON: serialised DataFrames, read without Python. It has switches (bus-bus and
  bus-element), so the output is closer to node-breaker. `bus_geodata` maps to GL and
  `line_geodata` to line routes.
- **PowerFactory DGS** (text/CSV variant first): ElmSubstat, ElmTerm, StaCubic, ElmCoup and
  ElmLne/ElmTr2 with their types, close to CGMES node-breaker. `IntGrfnet`/`IntGrf` graphics map
  to a DL diagram. The most work, and PowerFactory exports CGMES itself, so it only goes ahead if
  users ask for it.
- PGM JSON and UCTE could follow on the same `build.rs`, if wanted.

## Decisions

Decided 2026-11-22:

- Substations are grouped by transformer, and also by name where a branch connects the buses
  (see [Substation grouping](#substation-grouping)).

Decided 2026-11-27:

- Breakers at every branch end, on by default (see [Breakers](#breakers)).
- SV is always written from the case's voltages (see [SV](#sv)); without it, that information
  in the case would be lost.

## Status

Phase 1 built, 2026-11-27, on cimoxide's branch `matpower-import` (not yet committed):
`cimimport/` (package `cimoxide-import`, re-exported by the facade as `cimoxide::import`), the
`benchmark-grids/` submodule at gridoxide's commit, and tests in `cimimport/tests/matpower.rs`.

The crate is named `cimoxide-import`, not `cimoxide-convert` as first planned: a
`cimoxide-convert` was published in 0.3.x (the RDF/XML and JSON conversion, since merged into
`cimmodel`), and a different API under that crates.io name would mislead its users.

What phase 1 found and settled, beyond the mapping table above:

- **No base voltage.** `case14` has `baseKV` 0 throughout. Such buses get 1 kV, which keeps their
  per-unit values, and the notes say so.
- **A ratio of 1 is no ratio.** RTE's cases write `TAP` 1 on every line. Read as transformers,
  they chained the whole grid into a few substations.
- **CGMES rules that changed the model.**
  - A transformer's end 1 is its winding of the highest `ratedU`. Where that is MATPOWER's from
    side, the transformer is built mirrored: the impedance and charging go on end 1, scaled by
    tap² (which is the same two-port), and the shift sits on end 1.
  - Lines without charging that have a negative x become `SeriesCompensator`s, and those with a
    negative r become `EquivalentBranch`es (30 in `case9241pegase`).
  - Loads with a negative P or Q become `EquivalentInjection`s.
  - Generators get the kind their Pmin/Pmax make them: generator, motor, both, or condenser.
  - `normalPF` is 1 on the reference bus's machines and 0 elsewhere.
  - Each `Line` container holds one segment.
  - A phase shifter is a `PhaseTapChangerLinear` of steps −1..1 at ±1. That makes its largest
    angle its own, and gridoxide scales the reactance between `xMin` and `xMax` by that angle.
- **Kept as the cases give them, and noted.** These are the only validation findings left: a
  transformer's positive b (MATPOWER's charging), and non-positive x or negative r on
  transformers (star legs in RTE and PEGASE).
- **Speed.** `CimDataset::set` looks for the id among every object of its type, which is
  quadratic. The builder inserts directly instead, and `case9241pegase` converts in 3.7 s
  (58 s before). Other code that inserts many objects through `set` pays the same cost.

### A first cross-check, ahead of phase 2

A scratch program solved each case directly (`matpower::read`) and converted
(`BusBranchFromTp`), both from a flat start. Every case where both converge agrees to 1e-12 pu or
better: `case14`, `118`, `300`, `1354pegase`, `2869pegase`, `9241pegase`, `ACTIVSg200`, the 13
`matpower-plain` feeders and pglib's `case5`, `case30`. Six cases don't converge from a flat start
by either path (`case1888rte`, `case6495rte`, `case6515rte`, `case141`, pglib `case3_lmbd` and
`case300`); phase 2 should start those from the case's voltages.

Two of gridoxide's CGMES readings differ from the converter's output. Phase 2 has to settle them:

1. **The slack's magnitude comes from `SvVoltage.v`**, ahead of the regulating control's target.
   In a case that wasn't saved solved, the slack's `Vm` is only a starting value, while MATPOWER
   holds the bus at the generator's Vg. This explains `case4_dist`, `case18` and `case3120sp` (up
   to 0.065 pu). Writing Vg into the SV at the slack makes them agree to 1e-13.
2. **`RotatingMachine.q` is read without the load-sign negation** that loads get (a choice
   gridoxide made from RealGrid). The converter writes CGMES's load sign, q = −Qg. A generator
   on a PQ bus is then injected with the wrong sign: `case2848rte` is off by 7.7e-3 pu, and
   agrees to 3e-13 with q = +Qg.

Recommendation: fix both in gridoxide, since the converter follows CGMES on each.
1. Take the slack's magnitude from an enabled voltage control's target where there is one.
2. Look at the machine-Q sign again against a source that has a machine with q ≠ 0.

If gridoxide should stay as it is, the converter could write Vg into the SV at regulated buses
instead, but that loses the case's own Vm there.
