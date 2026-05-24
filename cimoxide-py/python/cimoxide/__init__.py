from .cimoxide import PyCimDataset as CimDataset, PyCimDatasetIter

__all__ = ["CimDataset", "decode_file", "decode_files", "decode_str"]


def decode_file(path: str) -> CimDataset:
    """Parse a single CGMES RDF/XML file."""
    return CimDataset.decode_file(path)


def decode_files(paths: list) -> CimDataset:
    """Parse multiple CGMES RDF/XML files, merging them into one dataset."""
    return CimDataset.decode_files(paths)


def decode_str(content: str) -> CimDataset:
    """Parse CGMES RDF/XML from a string."""
    return CimDataset.decode_str(content)
