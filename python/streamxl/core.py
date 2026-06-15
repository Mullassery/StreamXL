from streamxl._core import read as _read_all


def read_rows(path: str, sheet: str = None, as_dict: bool = False):
    """Low-level row iterator. Yields lists or dicts of Python values."""
    rows = _read_all(path, sheet)

    if as_dict:
        rows_iter = iter(rows)
        try:
            header = next(rows_iter)
        except StopIteration:
            return

        for row in rows_iter:
            yield dict(zip(header, row))
    else:
        yield from rows
