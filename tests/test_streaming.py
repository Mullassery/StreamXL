import streamxl


def test_is_iterator(tmp_xlsx):
    result = streamxl.read(tmp_xlsx)
    assert hasattr(result, "__iter__")
    assert hasattr(result, "__next__")


def test_memory_constant(tmp_large_xlsx):
    """Rows should be yielded one at a time — no full-file load."""
    import tracemalloc
    tracemalloc.start()
    snapshot_before = tracemalloc.take_snapshot()

    count = 0
    for _ in streamxl.read(tmp_large_xlsx):
        count += 1

    snapshot_after = tracemalloc.take_snapshot()
    tracemalloc.stop()

    stats = snapshot_after.compare_to(snapshot_before, "lineno")
    total_mb = sum(s.size_diff for s in stats) / 1024 / 1024
    assert total_mb < 50, f"Memory grew by {total_mb:.1f} MB — streaming may be broken"
