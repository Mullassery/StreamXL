from typing import Iterator, List, Any, Union, Dict
from .core import read_rows


def read(path: str, sheet: str = None, as_dict: bool = False) -> Iterator[Union[List[Any], Dict[str, Any]]]:
    """
    Stream rows from an XLSX file.

    Args:
        path: Path to the .xlsx file.
        sheet: Sheet name to read (defaults to first sheet).
        as_dict: Return rows as dicts with header row as keys (default: False).

    Yields:
        List of cell values per row, or dict with header keys if as_dict=True.
    """
    yield from read_rows(path, sheet=sheet, as_dict=as_dict)


def stream(path: str, sheet: str = None, as_dict: bool = False) -> Iterator[Union[List[Any], Dict[str, Any]]]:
    """Alias for read(). Stream rows from an XLSX file."""
    yield from read(path, sheet=sheet, as_dict=as_dict)
