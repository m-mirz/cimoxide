import os
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent))


def pytest_collection_modifyitems(config, items):
    if os.environ.get("RELICAPGRID_DIR"):
        return
    skip = pytest.mark.skip(reason="set RELICAPGRID_DIR to a relicapgrid checkout")
    for item in items:
        item.add_marker(skip)
