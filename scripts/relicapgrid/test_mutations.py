"""Broken copies of ReliCapGrid files, run through both tools: which rules each
newly reports. The cases follow the 2026 SV-IOP report: Table 2's missing
header properties, §5.2's RAS value types, §13.1's deprecated properties."""

import re

import pytest

import harness

AE = "Instance/Belgovia/NetworkCode/cimxml/Belgovia_AE.xml"
RAS = "Instance/Belgovia/NetworkCode/cimxml/Belgovia_RAS.xml"
REQUIRED = "urn:uuid:7528537a-551c-45bf-9e4f-1854287d5cdc"

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
}


@pytest.mark.parametrize("name", CASES)
def test_mutation(name):
    path, transform, triplets_rules, cimoxide_rules = CASES[name]
    triplets_new, cimoxide_new = harness.mutated(path, transform)
    assert {rule for _, rule in triplets_new} == triplets_rules
    assert {rule for _, rule in cimoxide_new} == cimoxide_rules
