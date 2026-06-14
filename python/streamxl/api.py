from typing import Iterator, List, Any
from .core import read_rows


def read(path: str, sheet: str = None) -> Iterator[List[Any]]:
    """
    Stream rows from an XLSX file.

    Args:
        path: Path to the .xlsx file.
        sheet: Sheet name (currently ignored — defaults to first sheet).

    Yields:
        List of cell values per row.
    """
    yield from read_rows(path)


def stream(path: str) -> Iterator[List[Any]]:
    """Alias for read()."""
    yield from read(path)
