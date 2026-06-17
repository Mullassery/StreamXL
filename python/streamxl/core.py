from streamxl._core import read as _read_all
from streamxl._core import write as _write_all
from streamxl._core import PyXlsxWriter as XlsxWriter


def read_rows(path: str):
    """Low-level row iterator. Yields lists of Python values."""
    yield from _read_all(path)


def write_rows(path: str, rows):
    """Low-level write — passes rows directly to the Rust engine."""
    _write_all(path, rows)
