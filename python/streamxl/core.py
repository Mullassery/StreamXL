from streamxl._core import read as _read_all


def read_rows(path: str):
    """Low-level row iterator. Yields lists of Python values."""
    yield from _read_all(path)
