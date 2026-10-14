from __future__ import annotations

from typing import Any, Iterator

CimObject = dict[str, Any]
"""
A parsed CIM element as a Python dict.

Always contains ``"_type": str`` (the CIM class name) and ``"id": str`` (the
mRID), plus one key per attribute the XML carried, named ``"Class.attr"``
(e.g. ``"IdentifiedObject.name"``, ``"ACLineSegment.r"``). Every value is the
``str`` the XML wrote — numbers are not parsed — and a reference is the
referenced mRID; an attribute written more than once is a ``list[str]``.
When a text value carried an ``xml:lang``, ``"_lang"`` maps its key to the
tags, in value order (``{"IdentifiedObject.description": ["en"]}``).
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
        """Parse a single CGMES RDF/XML file. Releases the GIL while parsing."""
        ...

    @staticmethod
    def decode_files(paths: list[str]) -> CimDataset:
        """Parse multiple CGMES RDF/XML files, merging them into one dataset.

        The files are parsed in parallel, one thread each, without holding the
        GIL, then merged in the order given. ``by_type()`` lists each class's
        elements in document order, the first file's first.
        """
        ...

    @staticmethod
    def decode_str(content: str) -> CimDataset:
        """Parse CGMES RDF/XML from a string. Releases the GIL while parsing."""
        ...

    def merge(self, other: CimDataset) -> None:
        """Merge another dataset into this one (other becomes empty).

        Scalar fields: last-wins.  ResourceList fields: union.
        Do not pass the same object as both self and other.
        """
        ...

    def drop_sparql_store(self) -> None:
        """Release the RDF graph cached by ``query()``.

        The graph roughly doubles the dataset's resident memory and is otherwise
        held until the dataset is dropped. The next ``query()`` rebuilds it.
        A no-op if no query has run.
        """
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
        not a recognized CIM type. Whether a string is a reference or text
        follows how the dataset holds that attribute on objects of the same
        class (a DCAT header's ``conformsTo`` is a reference, its
        ``dcat:version`` text), else the class table; one neither knows is text.
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

    def defined_mrids(self) -> list[str]:
        """The MRIDs of the objects this dataset defines (``rdf:ID``), as opposed
        to only describing objects another dataset defines (``rdf:about``).
        After a merge, an object is defined if any merged file defined it."""
        ...

    def types(self, mrid: str) -> list[str]:
        """Every type the files wrote ``mrid`` under: its class first, then any
        other a merged file used (an SSH file's ``Equipment`` for an EQ
        ``ACLineSegment``). Raises ``KeyError`` if not found."""
        ...

    def type_conflicts(self) -> list[tuple[str, list[str]]]:
        """The objects merged files typed with unrelated classes (neither an
        ancestor of the other), as ``[(mrid, [types])]``, sorted."""
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

    def foreign(self) -> dict[str, CimObject]:
        """Objects of classes no CIM class table knows — reference data's SKOS
        concepts, PROV agents, DCAT catalogues — keyed by identifier as written
        (a full IRI, or ``_:`` and a blank node's label); ``"_type"`` is the
        class IRI. They are not in ``entries()`` and not validated."""
        ...

    def query(self, sparql: str) -> list[dict[str, str]] | bool | list[tuple[str, str, str]]:
        """Run a SPARQL 1.1 query over this dataset.

        The first call materialises the dataset into an in-memory RDF graph and
        caches it; later calls reuse it, so the first query is much slower than
        the rest (~1.1 s vs ~4 ms on a 150k-element dataset). The cache is
        dropped on any mutation (``__setitem__``, ``__delitem__``, ``merge``)
        and can be released with ``drop_sparql_store()`` — it roughly doubles
        resident memory.

        The CGMES namespaces (``cim:``, ``eu:``, ``md:``, ``dm:``, ``rdf:``) and
        ``xsd:`` are pre-bound, so no prologue is needed.

        Returns a list of dicts for ``SELECT``, a bool for ``ASK``, and a list
        of ``(subject, predicate, object)`` string triples for ``CONSTRUCT`` and
        ``DESCRIBE``.
        """
        ...

    def to_xml(self) -> str:
        """Encode the whole dataset as RDF/XML: CGMES elements, then NC
        elements and ``foreign()`` objects as they were read."""
        ...

    def to_xml_for_profile(self, profile: str) -> str:
        """Encode this dataset as one profile's RDF/XML text: a CGMES profile
        (``"EQ"``, ``"SSH"``, …) or an NC one (``"CO"``, ``"AE"``, …).

        Only elements/fields the profile exchanges are emitted, with the
        dataset's own header for the profile when it has one (``md:FullModel``
        for CGMES, ``dcat:Dataset`` for NC), else a synthesized one. An unknown
        profile code raises ``RuntimeError``.
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
