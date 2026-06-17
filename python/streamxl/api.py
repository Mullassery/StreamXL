from typing import Any, Iterable, Iterator, List
from .core import read_rows, write_rows, XlsxWriter as _XlsxWriter


def read(path: str, sheet: str = None) -> Iterator[List[Any]]:
    """
    Stream rows from an Excel (.xlsx) file.

    Args:
        path:  Path to the .xlsx file.
        sheet: Sheet name (currently ignored — defaults to first sheet).

    Yields:
        List of cell values per row (str, float, bool, or None).
    """
    yield from read_rows(path)


def stream(path: str) -> Iterator[List[Any]]:
    """Alias for read()."""
    yield from read(path)


def write(path: str, rows: Iterable[Iterable[Any]]) -> None:
    """
    Write rows to an Excel (.xlsx) file.

    All rows are passed to the Rust engine in one call; the file is
    written and closed when the function returns.

    Args:
        path: Destination path for the .xlsx file.
        rows: Iterable of iterables of cell values.
              Supported types: str, int, float, bool, None.

    Example::

        streamxl.write("report.xlsx", [
            ["Name", "Age", "Score"],
            ["Alice", 30, 95.5],
            ["Bob",   25, 88.0],
        ])
    """
    write_rows(path, rows)


def writer(path: str) -> _XlsxWriter:
    """
    Return a streaming writer for row-by-row XLSX writing.

    Use as a context manager so the file is finalised on exit::

        with streamxl.writer("report.xlsx") as w:
            w.write_row(["Name", "Age", "Score"])
            for name, age, score in data:
                w.write_row([name, age, score])

    You can also call .close() manually instead of using ``with``.
    """
    return _XlsxWriter(path)
