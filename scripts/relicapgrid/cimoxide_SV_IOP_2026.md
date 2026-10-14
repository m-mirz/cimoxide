# cimoxide on the 2026 SV-IOP test cases

What cimoxide does for each test case of the [2026 SV-IOP report](SV_IOP_Report_2026.md)
(Network Code Profiles 2.5, ReliCapGrid), and with what result. The section numbers follow the
report's. Every statement here is backed by a test in this directory; the test names are given so
each result can be reproduced.

## Versions

| | |
|---|---|
| cimoxide | 0.5.1, commit `c426108` (branch `relicapgrid`), Python bindings `cimoxide-py` |
| Dataset | [entsoe/relicapgrid](https://github.com/entsoe/relicapgrid) `cgmes-3.0_ncp-2.5_tc-2.0` @ `f09bb34` (2026-09-11) |
| Profiles | ENTSO-E application-profiles-library @ `d8b2d21` (CGMES 3.0, NCP 2.5), cimoxide's submodule |
| Reference tool | [triplets](https://pypi.org/project/triplets/) 0.2.0, as ReliCapGrid's CI uses it |

## How cimoxide was tested

cimoxide is a library, a CLI (`cimcli`), a language server and an MCP server; it has no user
interface to click through and no power-flow solver. It was tested through its Python bindings by a
harness ([README](README.md)) that runs every step the report asks for and checks the result, so
that each Annex 1 entry below is the outcome of an automated test rather than an inspection.

Four kinds of check run on every file or file group, independent of the use cases:

| Check | What cimoxide does | Compared with | Test |
|---|---|---|---|
| Import | `CimDataset.decode_file` on each file; every decoded object, value and reference (`entries()` and `foreign()`) | triplets' triples of the same file | `test_import.py` |
| Validation | `cimoxide.validate_files(files, common=True, quality=True)` on the groups ReliCapGrid's CI forms | triplets' SHACL run on the same groups | `test_validation.py` |
| Export, per profile | `to_xml_for_profile(code)` on each CGMES and NC file, for the profile it declares | the source file: NC as an RDF graph, CGMES as cimoxide decodes it | `test_export.py` |
| Export, whole | `to_xml()` on each file, CGMES, NC and reference data alike | the source file, as an RDF graph | `test_export.py` |

The groups, and the policies for building them, come from ReliCapGrid's own
`buildScripts/validate_instances.py`, which the harness imports: the same files are validated
together as in ReliCapGrid's CI, for both tools.

Each use case then has a check of what the report asks (the objects and links its ReliCapGrid
description names), run on what cimoxide decoded (`test_use_cases.py`). Each such check is also run
on a copy of the data broken where the use case matters, and must fail there, so that a passing
check is known to look at something. Problems the check finds in the data or in the ReliCapGrid
description are listed per use case.

## Overall results

| | Files or cases | Result |
|---|---|---|
| Import | 135 files | identical to triplets, value for value |
| Validation | 19 groups | 1,151 findings both tools report; every disagreement explained (see [Validation](#validation-compared-with-triplets)) |
| Export per profile, CGMES | 47 files | the same objects and values back, as cimoxide decodes them (an SSH object typed with a class SSH does not have is written as `cim:Equipment`, as the ENTSO-E test configurations write it) |
| Export per profile, NC | 72 files | the same RDF graph, less two `IdentifiedObject.mRID` the SAR profile does not allow; `Svedala_FAP_PV.xml` (Provenance 1.0) is refused, cimoxide has no Provenance profile |
| Export whole | 127 files | the same RDF graph, every file |
| Use cases | 36 checks | all pass; data and description problems listed below |
| Edits | CO_UC1, RA_UC4 | replayed, exported and validated (`test_edits.py`) |

---

## 1. Power Flow on ReliCapGrid

### 1.1. PF_UC1: Power Flow ReliCapGrid model

**Report.** Run power flow on each IGM, then on growing combinations of IGMs, then on the whole
CGM (§1.1, Annex 2).

**What cimoxide does.** cimoxide has no power-flow solver, so it runs no power flow. It checks
what a power flow needs and what a solved case must contain, per IGM (`PF_UC1[<area>]`, nine areas:
Belgovia, Britheim, DC-Espheim-Svedala, DC-Nordheim-Galia, Espheim, Galia, Nordheim, Portheim,
Svedala):

1. Decodes `<area>/Grid/cimxml/*.xml` (`CimDataset.decode_files`).
2. Checks that the IGM has EQ, SSH, TP and SV datasets (`md:Model.profile` of its headers).
3. Checks that every TopologicalNode of a TopologicalIsland has an SvVoltage, and that there is a
   TopologicalIsland.
4. Validates the IGM with the boundary (group `cgmes-<area>`), which includes the SV
   `*SolvedMAS` rules on the merged model.

**Result.** import, validate, export. All nine IGMs carry a complete, solved power-flow case.

**Notes.**
- In Espheim and Svedala, TopologicalNodes outside every TopologicalIsland have no SvVoltage; they
  are de-energised, so this is correct.
- The SV shape `sv:SvStatus.ConductingEquipment-valueType` lists only the two converter classes;
  triplets, running it on the merged group, reports it 7,684 times. It is written for one SV file,
  where EQ equipment has no type; cimoxide runs it so and the cross-profile shape
  (`sh:class ConductingEquipment`) checks the type on the merged model.
- The SV rules `CsConverter.alpha/gamma-valueRangeTypical` (rectifier alpha and inverter gamma
  outside their typical range) compare SV values with the SSH operating mode; cimoxide runs them on
  the merged model and agrees with triplets on DC-Espheim-Svedala.

## 2. Boundary, Common and Reference data

### 2.9.1. BDS_UC1: Boundary Data Set Importing

**Report.** Load the boundary dataset, merge it with the IGMs, check that matching nodes and
terminals connect, with no duplication and no topology gap at the borders.

**What cimoxide does.**
1. Merges ReliCapGrid's CGM with `CimDataset.decode_files`: the files of its group
   `cgmes-CGM-Jotunheim` (the six boundary files, every IGM's EQ and updated SSH, Jotunheim's CGM
   topology and state, the common data), 31,412 objects.
2. Tags each Terminal with the IGM whose EQ defines it, and each object with the datasets that
   define it (`defined_mrids()`: `rdf:ID`, not `rdf:about`).
3. For every BoundaryPoint, follows `CommonResponsibilityPoint.ConnectivityNode` and checks that
   - Terminals of two IGMs connect to that node;
   - the node and all its Terminals sit on one TopologicalNode of the CGM's topology, which has an
     SvVoltage.
4. Checks that no object is defined by two datasets, and that no merged object was typed with
   unrelated classes by different files (`type_conflicts()`; an SSH file's `cim:Equipment` for an
   EQ class is a superclass, not a conflict).
5. Validates the boundary on its own (group `cgmes-boundary`, EQ and EquipmentBoundary shapes) and
   the CGM (group `cgmes-CGM-Jotunheim`).

**Result.** import, validate. No type conflicts. Topology gaps and duplicated definitions at the
borders:

| Boundary point | Problem |
|---|---|
| Madgar1-Lysaborg1, Madgar2-Lysaborg2, Madgar3-Lysaborg3 (`Boundary_Border-Espheim-Portheim.xml`) | reached from Espheim only: no Portheim terminal |
| Madgar4-Leyva1 (`Boundary_Border-Svedala-Espheim.xml`) | reached from DC-Espheim-Svedala only |
| BP Portheim-Espheim | Espheim's two terminals are on a TopologicalNode, Portheim's nine are on none |

The CGM's topology leaves Portheim out: 1 of its 121 terminals is on a TopologicalNode and none has
a voltage, while its EQ and SSH are part of the CGM. Every other IGM's terminals are all in the
topology (Espheim: 3,245 of 3,281).

Objects defined by more than one dataset, identically in each:

| Object | Defined by |
|---|---|
| VoltageLevel `0c4f41f8…`, Substation `42b7f03a…`, SubGeographicalRegion `f69de45f…` | Espheim EQ, Portheim EQ, `Boundary_Border-Espheim-Portheim.xml` |
| VoltageLevel `e61f8ecf…`, Substation `c39c3eb8…`, SubGeographicalRegion `77467625…`, ConnectivityNode `892ef502…` | Nordheim EQ, `Boundary_Border-Galia-Nordheim.xml` |
| GeographicalRegion `9356077b…` | Nordheim EQ, `Grid_CommonData_CGM-CD.xml` |

**Notes.**
- The boundary files' `dcat:Dataset` header names vocabularies only (`dcterms:conformsTo`
  `CIM100#` and `nc/2.4#`), no application profile, so no tool knows which shapes apply.
  ReliCapGrid's CI validates them as `CoreEquipment-EU/3.0`; the harness gives cimoxide copies
  whose `conformsTo` adds that profile (and `EquipmentBoundary-EU/3.0` for the boundary group).
  cimoxide reads CGMES profiles from `conformsTo` as well as from `md:Model.profile`.
- cimoxide's merge keeps one copy of an object several files define (a later value wins) and every
  type it was written under; it does not report duplicates by itself. This check does, from the
  datasets' definitions.

### 2.9.2. CD_UC1: Common Data Importing and exporting

**Report.** Import and export the common data; the twelve classes it lists are present.

**What cimoxide does.**
1. Decodes `commonData/Grid/cimxml/*.xml` and `commonData/NetworkCode/cimxml/*.xml`.
2. Checks that each listed class has instances: BiddingZone, BiddingZoneBorder,
   CapacityCalculationRegion, CoordinatedCapacityCalculator, Organisation,
   TransmissionSystemOperator, SecurityCoordinator, LoadFrequencyControlArea, SynchronousArea,
   SchedulingArea, LoadFrequencyControlBlock, TieCorridor.
3. Validates the NC common data (group `nc-commonData`).
4. Exports each file: the NC files (`Org-NineRealms_CD.xml` for ER, `Org-NineRealms_CD_OR.xml`
   for OR) per profile and whole; the CGMES common data whole.

**Result.** import, validate, export. All twelve classes present; every export is the same RDF
graph.

**Notes.** The report's "CapacityCalculationCalculator" is the class
CoordinatedCapacityCalculator.

### 2.9.3. RD_UC1: Reference Data Importing and exporting

**Report.** Load the reference data and verify that the other datasets use it consistently.

**What cimoxide does.**
1. Decodes `referenceData/NetworkCode/cimxml/*.xml` and every NC dataset. Reference data is SKOS,
   PROV, DCAT, QUDT and GeoSPARQL, which no CIM class table describes: cimoxide keeps those
   objects apart from the CIM objects (`CimDataset.foreign()`), keyed by their full IRI, blank
   nodes (`rdf:nodeID`) included.
2. For every NC dataset header, looks up each of `publisher`, `isVersionOf`, `spatial`,
   `wasGeneratedBy`, `license` and `accessRights` among the reference data's objects.
3. Exports each reference data file whole (`to_xml()`).

**Result.** import, export. The import equals triplets' and every export is the same RDF graph.
Of 390 URIs the headers name, 105 resolve:

| Header property | Not in the reference data |
|---|---|
| accessRights | 0 of 71 |
| publisher | 37 of 71 (`test/party/RCC-Jotunheim`, `test/party/Britheim`, `https://www.entsoe.eu/`) |
| license | 71 of 71 (all `creativecommons.org`; the reference data defines `…/License/…`) |
| spatial | 41 of 41 (`Test/Frame/X-Power-Transmission-System`; it defines `test/frame/X-Transmission`) |
| wasGeneratedBy | 65 of 65 (`Activity/CGM-IAM`; it defines `activity/AC-IAM`) |
| isVersionOf | 71 of 71 (no entry of the model collection matches) |

**Notes.** Reference data is not validated: cimoxide validates CIM and NC objects against their
shapes, and the reference data files declare no profile.

## 3. Contingencies

### 3.8.1. CO_UC1: Importing, executing, updating and exporting

**Report.** Import the contingencies, resolve them against the network model, execute them, update
them, export the updated CO dataset in compliance with the profile. ReliCapGrid's description
lists eleven contingencies (its CO_UC1 to CO_UC11) with the equipment each names.

**What cimoxide does.**
1. Decodes every NC dataset and every IGM.
2. For each of the twelve contingency → equipment pairs (CO_UC11 is an N-2 with two), checks that
   the contingency is an Ordinary, OutOfRange or Exceptional contingency, that one of its
   ContingencyEquipment names the equipment, and that the equipment has the class the description
   gives (ACLineSegment, PowerTransformer, LinearShuntCompensator, NonlinearShuntCompensator,
   CsConverter, DCLineSegment, SynchronousMachine, ConformLoad).
3. Validates each area's NC datasets with its IGM, common data and boundary (groups `nc-<area>`).
   The contingencies' equipment is in the IGM's EQ file, so the value-type checks of
   `ContingencyEquipment.Equipment` read its type there (314 findings on which both tools agree,
   see notes).
4. **Update** (`test_edits.py`): through cimoxide-py, contingency `7e31c67d…` (loss of the
   Belgovia–Galia tie-line) is renamed and given a second ContingencyEquipment for the
   Belgovia–Svedala tie-line `ed0c5d75…`, copied from its first: the N-1 becomes an N-2.
5. **Export**: the updated dataset is written for the CO profile (`to_xml_for_profile("CO")`),
   read back, and both tools validate it in place of `Belgovia_CO.xml`.

**Result.** import, validate, edit, export. All twelve pairs resolve with the stated classes. The
update survives export and re-import, the rest of the dataset is unchanged, and neither tool
reports anything on the new equipment that the original equipment did not have. A broken update
(no `ContingencyEquipment.contingentStatus`) is reported by both.

**Notes.**
- Execution (contingency analysis) is not in cimoxide.
- Both tools report `ContingencyEquipment.Equipment-valueType` on the contingencies because the
  SSH files type the same equipment `cim:Equipment` beside its EQ class; the value type lists
  concrete classes only.
- `Svedala_ER.xml` types a machine `cim:SynchronousMachine` in the NC namespace, which the value
  type of `ContingencyEquipment.Equipment` omits (it lists the CIM16 and CIM17 machines); cimoxide
  reports it, triplets, which matches classes by local name, does not.
- `Espheim_CO.xml`'s dataset header reuses the UUID of remedial action `5898c268…` in
  `Espheim_RA.xml`.

## 4. Equipment Reliability

### 4.5. ER_UC1: Import/Export Equipment Reliability (ER) Profile

**Report.** Import and export the ER profile, validate it, check consistency with the other
datasets.

**What cimoxide does.** Per area with an ER dataset (`ER_UC1[<area>]`: Belgovia, Britheim, Espheim,
Galia, Svedala):
1. Decodes the area's grid and NC datasets, the common data and the boundary.
2. Follows every association of every object of the area's `*_ER.xml` and checks that its target
   is in the decoded data.
3. Validates (groups `nc-<area>`), and exports the ER file per profile and whole.

**Result.** import, validate, export. Every association of every ER object resolves; exports are
the same RDF graph.

**Notes.**
- `Espheim_ER.xml` types a tap changer `cim:PhaseTapChangerLinear` in the NC namespace, a class
  the NC vocabulary does not define; cimoxide reads it as the CGMES class and writes it back in the
  namespace it was read in.
- NCP's `sh:class` shapes require NC-namespace classes (`cim:Terminal`,
  `cim:OperationalLimitSet`, …) where ReliCapGrid's targets are the CGMES 3.0 objects in CIM100,
  while its `sh:in` value types list both. cimoxide checks the IRIs as written and reports 25 such
  findings (PinTerminal.Terminal, Circuit.IdentifyingTerminal, …); an inconsistency of the APL.

## 5. Remedial Action

All RA checks decode the area's grid and NC datasets and the common data (Svedala for RA_UC1, 2, 4
and 5; Espheim for RA_UC3; Belgovia for RA_UC6 and 7), and validate them in the groups
`nc-<area>`. Every NC file is exported per profile and whole.

### 5.9. RA_UC1: Topology Remedial Action with One Topology Action

**What cimoxide does.** Checks that GridStateAlterationRemedialAction `6d2c8901…` has exactly one
TopologyAction, that it operates switch `176d262c…`, and that the switch is closed in SSH
(`Switch.open` false).

**Result.** import, validate, export. As described.

### 5.10. RA_UC2: Dependent Topology Remedial Actions with Multiple Topology Actions

**What cimoxide does.** Checks that dependencies `598ab84e…` and `13c29cc3…` are exclusive, that
each names a GridStateAlterationRemedialAction with several TopologyActions, and that both act on
the same four switches.

**Result.** import, validate, export. The data differs from the description: it describes two
remedial actions on the same four switches of Substation A; the data's are
"RA SubstationB_2N" and "RA SubstationA_quarterbar1B", on four different switches each.

### 5.11. RA_UC3: Modification Prior to RAO of a Topology Remedial Action

**What cimoxide does.** Checks that GridStateAlterationRemedialAction `d856a2a2…` has
TopologyAction `4d3757fd…`, that GridStateAlterationSchedule `f5205bd4…` schedules it, and that
the schedule's time points both enable and disable it.

**Result.** import, validate, export. As described.

### 5.12. RA_UC4: Proposing Modifications of a Topology Remedial Action After RAO

**Report.** Refuse the RAO's remedial action schedule as a whole, create a complete new schedule
with new identifiers (new EventSchedule included) and a RemedialActionScheduleDependency of kind
none from the new to the refused one.

**What cimoxide does.**
1. Checks ReliCapGrid's own answer: RemedialActionScheduleResponse `96f93b71…` refuses schedule
   `17527637…`; dependency `f7231aa5…` replaces it with `51361821…`, whose EventSchedule is not
   the refused one's.
2. **Edit** (`test_edits.py`): builds the same answer through cimoxide-py from the RAO proposal
   (`Svedala_RAS_Jotunheim_proposed.xml`): a response refusing `17527637…`; a copy of the schedule
   under a new identifier, with a new EventSchedule and every time point copied under new
   identifiers; the dependency; and a new `dcat:Dataset` header.
3. **Export**: writes it for the RAS profile, reads it back, and has both tools validate it in
   place of ReliCapGrid's `Svedala_RAS_acceptance_proposal.xml`.

**Result.** import, validate, edit, export. Both tools report nothing on the new objects that
ReliCapGrid's counterparts did not have. A broken variant (the dependency replacing the response
instead of the schedule) is reported by cimoxide (`ReplacedRemedialActionSchedule-valueType`);
triplets misses it.

**Notes.** The description's step "set RemedialActionScheduleDependencyKind to none" contradicts
the APL: `rasc:RemedialActionScheduleDependency.kind-applicability` forbids a kind on a dependency
outside a RemedialActionScheduleGroup, and both tools report it when the step is followed.
ReliCapGrid's data, which gives no kind, follows the APL.

### 5.13. RA_UC5: Activation After RAO of a Topology Remedial Action

**What cimoxide does.** Checks that remedial actions `7e422768…` and `fa3e3533…` are exclusive
within group `6a9b5221…`, that EventSchedules `1c1e39cf…` and `023512a8…` belong to schedules
`279bfb00…` and `87e90c47…`, and that at no time point both are active.

**Result.** import, validate, export. The data differs from the description: although exclusive,
both schedules are active at 2024-11-20T00:30Z, 01:30Z and 02:30Z.

### 5.14. RA_UC6: Countertrade Remedial Action

**What cimoxide does.** Checks that CountertradeRemedialAction `30f6377d…` has PowerSchedule
`27940387…` for GeneratingUnit `413bbc8a…`, and PowerBidSchedule `ae6a5ddd…`.

**Result.** import, validate, export. As described.

**Notes.** The PowerSchedule names a plain GeneratingUnit where the value type requires one of its
subclasses; both tools report it.

### 5.15. RA_UC7: Redispatch Remedial Action

**What cimoxide does.** Checks that variant 1's RedispatchRemedialAction `fa73cfee…` has
PowerBidSchedule `6c0ba4c5…` with time point `250e37a5…`; that PowerShiftKeySchedule `3b5748de…`
is on machine `3a3b27be…`'s unit and distribution `7ab20445…` belongs to it; and that the
distribution leads to variant 2's `9d72d12a…`.

**Result.** import, validate, export. The data differs from the description: variant 2's
`9d72d12a…` is linked to no power shift key object; distribution `7ab20445…` leads through
PowerBidSchedule `6c0ba4c5…` to variant 1's `fa73cfee…`.

## 6. Availability Schedule

### 6.2. AS_UC1: Availability Remedial Action

**What cimoxide does.** Decodes Svedala's grid, NC and common data; checks that
AvailabilityRemedialAction `14b2b671…` has AvailabilitySchedule `d394295f…` and that the schedule
is cancelled (`AvailabilitySchedule.cancelledDateTime`).

**Result.** import, validate, export. As described.

## 7. Assessed Element

All AE checks decode Belgovia's grid, NC and common data (AE_UC5 every area's NC data), validate in
group `nc-Belgovia`, and export each file per profile and whole.

| Use case | What cimoxide checks | Result |
|---|---|---|
| 7.1. AE_UC1 Secured | AssessedElement `992c2de6…` is in the base case and secured for region `56d81c91…` | import, validate, export |
| 7.2. AE_UC2 Scanned | `1eb2eb03…` is not in the base case, scanned and secured, with an exclusion reason; `d463cbba…` is scanned only | import, validate, export |
| 7.3. AE_UC3 Disable | AssessedElementSchedule `7221ae31…` on `992c2de6…`, whose time point `a26e3ae0…` disables it | import, validate, export |
| 7.4. AE_UC4 Exclude | `1eb2eb03…` is excluded through ScannedForRegion | import, validate, export |
| 7.5. AE_UC5 With contingency | `d17943b5…` is in the base case and combinable; AssessedElementWithContingency `1f38d403…` excludes and `deb4c65f…` includes a contingency that exists; `663c6c3c…` is not combinable | import, validate, export |
| 7.6. AE_UC6 With remedial action | AssessedElementWithRemedialAction `66f786ce…` links `13d17257…` to GridStateAlterationRemedialAction `70b696ac…` | import, validate, export |

**Notes.** The AEs' `ConductingEquipment` are in the IGMs' EQ files; both tools check their type
there and agree on 316 findings, raised by the SSH files typing the same equipment `cim:Equipment`.

## 8. Object Registry (OR)

### 8.1.1. OR_UC1

**Report.** Import Espheim's Object Registry, validate it, check consistency with the other
datasets.

**What cimoxide does.** Decodes Espheim's grid and NC data and the common data; checks that there
are Designations, DesignationTypes and DesignationAuthorities and that each of their associations
resolves; validates (groups `nc-Espheim`, `nc-commonData`) and exports `Espheim_OR.xml` and
`Org-NineRealms_CD_OR.xml` per profile and whole.

**Result.** import, validate, export.

## 9. Security Analysis Results (SAR)

### 9.1.1. SAR_UC1

**Report.** Export a SAR: the tool produces security analysis results and exports them.

**What cimoxide does.** cimoxide runs no security analysis and cannot produce a SAR. For the SAR
ReliCapGrid supplies (`Jotunheim/NetworkCode/2D_SAR_1030Z_Jotunheim.xml`), it decodes it with all
grid, NC and common data, checks that its BaseCasePowerFlowResults and ContingencyPowerFlowResults
exist and that every association resolves, validates it (group `nc-Jotunheim`), and re-exports it.

**Result.** import, validate. The SAR's associations resolve. The per-profile export leaves out two
`IdentifiedObject.mRID` on ContingencyPowerFlowResults, which the SAR profile does not exchange
(both tools report them as `ContingencyPowerFlowResult-AllowedProperties`).

## Other ReliCapGrid use cases

The five SIPS descriptions in ReliCapGrid (gate logic, complex SIPS logic, PATL/TATL conditions,
last-line disconnection, anti-swing protection, overvoltage protection) are not in the report. They
need contingency analysis; cimoxide checks their structure only (`SIPS[Belgovia]`,
`SIPS[Svedala]`): the SIPS classes (SchemeRemedialAction, Stage, StageTrigger, Gate, GateInputPin,
PinTerminal, PinEquipmentTripping, RemedialActionScheme, TriggerCondition) exist and every
association resolves. "Correction of generation production…" has no data.

## Validation compared with triplets

| | Findings |
|---|---|
| Both tools | 1,151 |
| triplets only, explained | 9,532 |
| cimoxide only, explained | 27 |
| triplets only, a rule cimoxide does not have | 49 (`Model.profile-allowedValues` 48, `SynchronousMachine-AllowedProperties` 1) |
| cimoxide only, a rule triplets does not run | 962 (`All-DanglingReferences` 820, cimoxide's modelling-quality and header checks) |
| triplets only, triplets' own check | 9,652 (`triplets:lexicalForm`: e.g. `500` for an `xsd:float`) |

Every disagreement has a recorded cause (`harness.TRIAGE`; `test_validation.py` fails on a new one).
The largest:

| Rule | Reported by | Findings | Cause |
|---|---|---|---|
| SvStatus.ConductingEquipment-valueType | triplets | 7,684 | a per-file SV shape run on the merged group (see PF_UC1) |
| IdentifiedObject.mRID-cardinality | triplets | 1,364 | SSH describes equipment (`rdf:about`) without an mRID, which the EQ file defines; cimoxide requires the mRID where an object is defined |
| BoundaryPoint.*-cardinality | triplets | 140 | the data has `nc:BoundaryPoint`, the CGMES shapes target `cim:BoundaryPoint`; triplets matches classes by local name |
| description / versionNotes langTag-presence | triplets | 142 | triplets drops language tags |
| identifier-datatype | triplets | 72 | triplets reads a UUID literal as a reference |
| conformsTo-datatype | triplets | 47 | `conformsTo` IRIs ending in `#` |
| RemedialActionSchedule-proposingEntity, Circuit-associations | triplets | 29 | triplets 0.2.0 does not evaluate `sh:alternativePath` |
| *-containment | triplets | 22 | containers defined in several datasets, counted once per definition |
| NC `sh:class` value types | cimoxide | 25 | APL inconsistency, see ER_UC1 |

### Negative tests (report Table 2, §5.2, §13.1)

Copies of ReliCapGrid files broken one way each, run through both tools (`test_mutations.py`):

| Case | triplets reports | cimoxide reports |
|---|---|---|
| Table 2: no `prov:wasGeneratedBy` | wasGeneratedBy-cardinality | wasGeneratedBy-cardinality |
| Table 2: no `dcat:startDate` | startDate-cardinality | startDate-cardinality |
| no `xml:lang` on `dcterms:description` | – (drops tags) | description.langTag-presence |
| §13.1: deprecated `prov:wasInfluencedBy` | – | – (no shape closes the header) |
| `dcterms:requires` a missing dataset | – | All-DanglingReferences |
| §5.2: RAS value not a number | GenericValueTimePoint.value-datatype | GenericValueTimePoint.value-datatype |
| §5.2: RAS boolean written as text | GenericValueTimePoint.value-datatype | GenericValueTimePoint.value-datatype |
| rectifier minAlpha below 10 (EQ limit, SSH mode) | CsConverter.minAlpha-valueRangeTypical | CsConverter.minAlpha-valueRangeTypical |
| inverter minGamma below 17 (EQ limit, SSH mode) | CsConverter.minGamma-valueRangeTypical | CsConverter.minGamma-valueRangeTypical |
| topology action on a line (target in EQ) | TopologyAction.Switch-valueType | TopologyAction.Switch-valueType |

## Implemented improvements

Changes made to cimoxide while testing with ReliCapGrid:

- **Validation across a model's files.** NC value types read a referenced object's type from the
  other files validated with it (an NC dataset's equipment is in the IGM's EQ). Hand-written rules
  of one profile that read another's data (CsConverter typical ranges, the DY mbase equation, the
  OP measurement terminal, …) run on the merged model.
- **CGMES profiles from DCAT headers.** A CGMES profile named by `dcterms:conformsTo` is detected,
  as one named by `md:Model.profile`.
- **Literals as written.** Whitespace is kept and empty elements are empty values, not dropped.
- **NC export.** `to_xml_for_profile` writes NC profiles (DCAT header, elements, fields, identifiers
  as read); an unknown profile code is an error instead of an empty document.
- **Reference data.** Objects of classes no CIM table knows (SKOS, PROV, DCAT, …) and blank nodes
  are kept (`foreign()`); `to_xml()` writes the whole dataset as read.
- **Editing.** Objects set from Python keep whether a value is a reference and its language tags,
  following how the dataset holds the field.

## Annex 1: Test Use Cases Results

| Use case | cimoxide 0.5.1 |
|---|---|
| **PF_UC1** | import, validate, export (no power flow) |
| **BDS_UC1** | import, validate |
| **CD_UC1** | import, validate, export |
| **RD_UC1** | import, export |
| **CO_UC1** | import, validate, edit, export (no contingency analysis) |
| **ER_UC1** | import, validate, export |
| **RA_UC1** | import, validate, export |
| **RA_UC2** | import, validate, export |
| **RA_UC3** | import, validate, export |
| **RA_UC4** | import, validate, edit, export |
| **RA_UC5** | import, validate, export |
| **RA_UC6** | import, validate, export |
| **RA_UC7** | import, validate, export |
| **AS_UC1** | import, validate, export |
| **AE_UC1** | import, validate, export |
| **AE_UC2** | import, validate, export |
| **AE_UC3** | import, validate, export |
| **AE_UC4** | import, validate, export |
| **AE_UC5** | import, validate, export |
| **AE_UC6** | import, validate, export |
| **OR_UC1** | import, validate, export |
| **SAR_UC1** | import, validate (the supplied SAR; cimoxide cannot produce one) |

## Annex 2: Issues found in the data, the descriptions, the APL and triplets

| Where | Issue |
|---|---|
| ReliCapGrid data | Boundary files declare no application profile. |
| ReliCapGrid data | Madgar1-3/Lysaborg1-3 have no Portheim terminal; Madgar4-Leyva1 only the HVDC IGM's. |
| ReliCapGrid data | The CGM's topology and state leave Portheim out, so BP Portheim-Espheim is joined on one side only. |
| ReliCapGrid data | Eight boundary and region objects are defined again by IGMs or the common data (VoltageLevel `0c4f41f8…` by three datasets). |
| ReliCapGrid data | `Espheim_CO.xml`'s header reuses remedial action `5898c268…`'s UUID. |
| ReliCapGrid data | 285 of 390 header URIs name reference data the reference data files do not define. |
| ReliCapGrid data | A plain GeneratingUnit or RegulatingControl where a subclass is required (RA_UC6, PowerShiftKeySchedule, RegulatingControlAction). |
| ReliCapGrid data | Britheim's PCC terminal is on an ACLineSegment. |
| ReliCapGrid descriptions | RA_UC2, RA_UC5 and RA_UC7 data differs from the description (see each). |
| ReliCapGrid descriptions | RA_UC4's kind none contradicts the APL. |
| APL | `sv:SvStatus.ConductingEquipment-valueType` and `dl:DiagramObject.IdentifiedObject-valueType` list only their own profile's classes. |
| APL | NCP `sh:class` shapes name NC-namespace classes where `sh:in` value types also accept CIM100. |
| triplets 0.2.0 | Runs per-profile CGMES shapes on merged groups; no `sh:alternativePath`; drops language tags; matches classes by local name; misses a value-type violation within one file (RA_UC4 broken variant). |
