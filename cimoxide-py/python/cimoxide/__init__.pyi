from __future__ import annotations

from typing import Any, Iterator

CimObject = dict[str, Any]
"""
A parsed CIM element as a Python dict.

Always contains ``"_type": str`` (the CIM class name) plus one key per
populated CIM attribute (snake_case).  Numeric/bool fields are ``float``,
``int``, or ``bool``; reference fields (MridRef) are plain MRID ``str``
values; string fields are ``str``.
"""

class PyCimDatasetIter:
    def __iter__(self) -> PyCimDatasetIter: ...
    def __next__(self) -> str: ...

class CimDataset:
    """A parsed CGMES dataset keyed by MRID."""

    @staticmethod
    def decode_file(path: str) -> CimDataset:
        """Parse a single CGMES RDF/XML file."""
        ...

    @staticmethod
    def decode_files(paths: list[str]) -> CimDataset:
        """Parse multiple CGMES RDF/XML files, merging them into one dataset."""
        ...

    @staticmethod
    def decode_str(content: str) -> CimDataset:
        """Parse CGMES RDF/XML from a string."""
        ...

    def merge(self, other: CimDataset) -> None:
        """Merge another dataset into this one (other becomes empty).

        Scalar fields: last-wins.  ResourceList fields: union.
        Do not pass the same object as both self and other.
        """
        ...

    def drop_blocks(self) -> None:
        """Release internal RdfBlock memory after the final merge."""
        ...

    def __len__(self) -> int: ...
    def __contains__(self, mrid: str) -> bool: ...
    def __getitem__(self, mrid: str) -> CimObject: ...
    def __iter__(self) -> PyCimDatasetIter: ...

    def get(self, mrid: str) -> CimObject | None:
        """Return the element dict for the given MRID, or None if not found."""
        ...

    def mrids(self) -> list[str]:
        """Return all MRIDs as a list."""
        ...

    def by_type(self) -> dict[str, list[str]]:
        """Return a type-name → MRID-list index (no deserialization)."""
        ...

    def get_type(self, type_name: str) -> list[CimObject]:
        """Return all element dicts for the given CIM type name.

        Example::

            lines = ds.get_type("ACLineSegment")
            # [{"_type": "ACLineSegment", "r": 0.12, ...}, ...]
        """
        ...

    def entries(self) -> dict[str, CimObject]:
        """Return all entries as ``{mrid: element_dict}``.

        Deserializes every element — prefer ``get_type`` or ``__getitem__`` for
        partial access on large datasets.
        """
        ...


def decode_file(path: str) -> CimDataset: ...
def decode_files(paths: list[str]) -> CimDataset: ...
def decode_str(content: str) -> CimDataset: ...
