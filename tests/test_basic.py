import pytest
import streamxl


def test_read_returns_rows(tmp_xlsx):
    rows = list(streamxl.read(tmp_xlsx))
    assert len(rows) > 0


def test_row_is_list(tmp_xlsx):
    rows = list(streamxl.read(tmp_xlsx))
    assert isinstance(rows[0], list)


def test_empty_file_returns_empty(empty_xlsx):
    rows = list(streamxl.read(empty_xlsx))
    assert rows == []
