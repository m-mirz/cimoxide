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

class Violation:
    """A single SHACL or custom validation finding."""

    object_id:   str
    """MRID of the offending element."""
    rule_id:     str
    """Unique rule identifier (e.g. ``"Rule-EQ-1"``)."""
    name:        str
    """Short rule name."""
    class_:      str
    """CIM class of the offending element."""
    property:    str
    """CIM property that triggered the violation (empty string if not applicable)."""
    message:     str
    """Human-readable violation description."""
    severity:    str
    """``"Violation"`` or ``"Warning"``."""
    description: str
    """Longer rule description."""

    def __repr__(self) -> str: ...

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

    def __setitem__(self, mrid: str, value: CimObject) -> None:
        """Insert or replace the element at ``mrid``.

        ``value`` must be a dict shaped like the ones returned by
        ``__getitem__`` (a ``"_type"`` key naming a known CIM class, plus
        attribute keys). Raises ``ValueError`` if ``"_type"`` is missing or
        not a recognized CIM type.
        """
        ...

    def __delitem__(self, mrid: str) -> None:
        """Remove the element at ``mrid``. Raises ``KeyError`` if not found."""
        ...

    def get(self, mrid: str) -> CimObject | None:
        """Return the element dict for the given MRID, or None if not found."""
        ...

    def mrids(self) -> list[str]:
        """Return all MRIDs as a list."""
        ...

    def by_type(self) -> dict[str, list[str]]:
        """Return a type-name → MRID-list index (no deserialization).

        Nothing is deserialized, but the whole index is copied out: one ``str``
        per MRID in the dataset, across every type. To count a single type, use
        ``count_type`` instead — it copies nothing.
        """
        ...

    def count_type(self, type_name: str) -> int:
        """Return the number of elements of the given CIM type (0 if unknown).

        An O(1) index lookup that copies nothing — prefer this over
        ``len(ds.by_type()[name])``, which materialises the entire index.
        """
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

    def query(self, sparql: str) -> list[dict[str, str]] | bool | list[tuple[str, str, str]]:
        """Run a SPARQL 1.1 query over this dataset.

        The dataset is materialised into an in-memory RDF graph on every call,
        so hold on to the results rather than querying in a tight loop. The
        CGMES namespaces (``cim:``, ``eu:``, ``md:``, ``dm:``, ``rdf:``) and
        ``xsd:`` are pre-bound, so no prologue is needed.

        Returns a list of dicts for ``SELECT``, a bool for ``ASK``, and a list
        of ``(subject, predicate, object)`` string triples for ``CONSTRUCT`` and
        ``DESCRIBE``.
        """
        ...

    def to_xml_for_profile(self, profile: str) -> str:
        """Encode this dataset as a single CGMES profile's RDF/XML text.

        Only elements/fields whose CIM schema origin includes ``profile``
        (e.g. ``"EQ"``, ``"SSH"``) are emitted. If the dataset contains a
        decoded ``FullModel`` header for this profile, it is reused verbatim;
        otherwise a minimal synthetic header is generated.
        """
        ...

    def write_xml_files(self, dir: str, profiles: list[str]) -> None:
        """Encode and write one RDF/XML file per profile into ``dir``.

        Creates ``dir`` (and parents) if it doesn't exist, then writes
        ``dir/{profile}.xml`` for each entry in ``profiles``, e.g.
        ``["EQ", "SSH"]`` -> ``dir/EQ.xml``, ``dir/SSH.xml``.
        """
        ...


def decode_file(path: str) -> CimDataset: ...
def decode_files(paths: list[str]) -> CimDataset: ...
def decode_str(content: str) -> CimDataset: ...

def validate_files(
    paths: list[str],
    profiles: list[str] | None = None,
    solved: bool | None = None,
    common: bool = False,
    quality: bool = False,
    silence: list[str] | None = None,
) -> list[Violation]:
    """Validate a set of CGMES profile files using two-phase validation.

    Phase 1 runs per-profile (local) SHACL and SPARQL rules against each
    file's individual dataset before merging. Phase 2 runs cross-profile
    rules on the merged dataset. This is the recommended entry point for
    validation.

    Parameters
    ----------
    paths:
        Paths to the CGMES RDF/XML files to validate.
    profiles:
        Profile short names to check, e.g. ``["EQ", "SSH"]``.
        ``None`` (default) uses auto-detected profiles.
    solved:
        ``True`` forces solved-case checks; ``False`` forces
        not-solved checks; ``None`` (default) auto-detects.
    common:
        Enable cross-profile common checks (default ``False``).
    quality:
        Enable CIMdesk modeling quality checks (default ``False``).
    silence:
        Rule IDs to suppress, e.g. ``["Rule-EQ-1"]``.
    """
    ...
