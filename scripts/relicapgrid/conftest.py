import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent))

import harness  # noqa: E402


def pytest_collection_modifyitems(config, items):
    try:
        harness.relicap_dir()
    except RuntimeError as missing:
        skip = pytest.mark.skip(reason=str(missing))
        for item in items:
            item.add_marker(skip)
