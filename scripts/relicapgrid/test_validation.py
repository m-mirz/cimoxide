"""cimoxide's validation against triplets, group by group as ReliCapGrid's CI forms them."""

import pytest

import harness


def groups():
    try:
        return [g.name for g in harness.all_groups()]
    except RuntimeError:
        return []


@pytest.mark.parametrize("name", groups())
def test_every_disagreement_is_triaged(name):
    """Each (object, rule) the tools disagree on has a cause in harness.TRIAGE.
    A new rule here means a new difference to look at."""
    untriaged = sorted({(d.side, d.finding.rule) for d in harness.validation_diff(harness.group(name))
                        if d.kind == harness.DISAGREE and d.finding.rule not in harness.TRIAGE})
    assert not untriaged


def test_tools_agree_somewhere():
    """Guards the comparison itself: ids and rule names line up on both sides."""
    agree = sum(d.kind == harness.AGREE for g in harness.all_groups() for d in harness.validation_diff(g))
    assert agree > 1100
