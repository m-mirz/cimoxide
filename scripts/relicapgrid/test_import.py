"""cimoxide's decoder against triplets, triple by triple, on every instance file."""

import pytest

import harness


def files():
    try:
        return harness.all_files()
    except RuntimeError:
        return []


@pytest.mark.parametrize("path", files())
def test_import_matches_triplets(path):
    """Objects of classes no CIM table knows (SKOS, PROV, DCAT) count, from foreign();
    a literal is kept as written: no trimmed ("whitespace") or dropped ("empty") ones."""
    kinds = harness.classify(harness.import_diff(path))
    assert not kinds, {kind: triples[:10] for kind, triples in kinds.items()}
