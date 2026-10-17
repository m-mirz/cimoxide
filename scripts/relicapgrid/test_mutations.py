"""Broken copies of ReliCapGrid files, run through both tools: which rules each
newly reports. The cases follow the 2026 SV-IOP report: Table 2's missing
header properties, §5.2's RAS value types, §13.1's deprecated properties. The
last cases break a rule whose data spans files: a value checked against
another profile's, a reference whose target another file types."""

import re

import pytest

import harness

AE = "Instance/Belgovia/NetworkCode/cimxml/Belgovia_AE.xml"
RAS = "Instance/Belgovia/NetworkCode/cimxml/Belgovia_RAS.xml"
REQUIRED = "urn:uuid:7528537a-551c-45bf-9e4f-1854287d5cdc"
HVDC_EQ = "Instance/DC-Espheim-Svedala/Grid/cimxml/20220615T2230Z__HVDC-Espheim-Svedala_EQ_1.xml"
RA = "Instance/Belgovia/NetworkCode/cimxml/Belgovia_RA.xml"
SWITCH = "#_0a84038e-1952-4d9d-9909-3b49c364a1ac"
LINE = "#_2d6d13d9-fb6e-4da2-8ddf-31ae98dcff8f"      # an ACLineSegment in Belgovia's EQ
RECTIFIER = "_038ce404-30dc-4289-b9db-7076cb870b8e"  # SSH: operatingMode rectifier
INVERTER = "_e468e6ca-8776-4c22-8f9b-390df610c2fd"   # SSH: operatingMode inverter


def in_element(mrid, old, new):
    """Replace *old* by *new* inside the CsConverter *mrid* defines."""
    def transform(text):
        start = text.index(f'rdf:ID="{mrid}"')
        end = text.index("</cim:CsConverter>", start)
        return text[:start] + text[start:end].replace(old, new, 1) + text[end:]
    return transform


# name: (file, transform, rules triplets newly reports, rules cimoxide newly reports)
CASES = {
    "Table 2: no prov:wasGeneratedBy": (
        AE, lambda t: re.sub(r"\s*<prov:wasGeneratedBy [^>]*/>", "", t, count=1),
        {"wasGeneratedBy-cardinality"}, {"wasGeneratedBy-cardinality"}),
    "Table 2: no dcat:startDate": (
        AE, lambda t: re.sub(r"\s*<dcat:startDate>[^<]*</dcat:startDate>", "", t, count=1),
        {"startDate-cardinality"}, {"startDate-cardinality"}),
    # triplets reports the missing tag on every description already (it drops xml:lang)
    "no xml:lang on dcterms:description": (
        AE, lambda t: t.replace('<dcterms:description xml:lang="en">', "<dcterms:description>", 1),
        set(), {"description.langTag-presence"}),
    # §13.1: removed from the schema, but no shape closes the header
    "deprecated prov:wasInfluencedBy": (
        AE, lambda t: t.replace("</dcat:Dataset>",
                                f'<prov:wasInfluencedBy rdf:resource="{REQUIRED}"/></dcat:Dataset>', 1),
        set(), set()),
    # ReliCapGrid's NC groups run no dangling-reference check
    "dcterms:requires to a missing dataset": (
        AE, lambda t: t.replace(REQUIRED, "urn:uuid:00000000-0000-4000-8000-000000000000", 1),
        set(), {"All-DanglingReferences"}),
    "§5.2: RAS value not a number": (
        RAS, lambda t: t.replace("<nc:GenericValueTimePoint.value>1.0<",
                                 "<nc:GenericValueTimePoint.value>yes<", 1),
        {"GenericValueTimePoint.value-datatype"}, {"GenericValueTimePoint.value-datatype"}),
    "§5.2: RAS boolean written as text": (
        RAS, lambda t: t.replace("<nc:GenericValueTimePoint.value>1.0<",
                                 "<nc:GenericValueTimePoint.value>true<", 1),
        {"GenericValueTimePoint.value-datatype"}, {"GenericValueTimePoint.value-datatype"}),
    # The SSH rule compares SSH's operating mode with EQ's angle limits.
    "rectifier minAlpha below 10 (EQ limit, SSH mode)": (
        HVDC_EQ, in_element(RECTIFIER, "<cim:CsConverter.minAlpha>10<", "<cim:CsConverter.minAlpha>5<"),
        {"CsConverter.minAlpha-valueRangeTypical"}, {"CsConverter.minAlpha-valueRangeTypical"}),
    "inverter minGamma below 17 (EQ limit, SSH mode)": (
        HVDC_EQ, in_element(INVERTER, "<cim:CsConverter.minGamma>17<", "<cim:CsConverter.minGamma>12<"),
        {"CsConverter.minGamma-valueRangeTypical"}, {"CsConverter.minGamma-valueRangeTypical"}),
    # The action's switch is in the EQ file, not the RA file.
    "topology action on a line (target typed in EQ)": (
        RA, lambda t: t.replace(f'<nc:TopologyAction.Switch rdf:resource="{SWITCH}" />',
                                f'<nc:TopologyAction.Switch rdf:resource="{LINE}" />', 1),
        {"TopologyAction.Switch-valueType"}, {"TopologyAction.Switch-valueType"}),
}


@pytest.mark.parametrize("name", CASES)
def test_mutation(name):
    path, transform, triplets_rules, cimoxide_rules = CASES[name]
    triplets_new, cimoxide_new = harness.mutated(path, transform)
    assert {rule for _, rule in triplets_new} == triplets_rules
    assert {rule for _, rule in cimoxide_new} == cimoxide_rules
