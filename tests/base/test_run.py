"""Pytest file for testing `src/ars/base/run.py`."""

from ars.base.run import Run


def test_run_creation() -> None:
    """Test Creating Runs."""
    r = Run([1, 2, 3], block_size=4)
    assert r.size == 3
    assert r.start == 1
    assert r.end == 3
    assert len(r.blocks) == 1


def test_append_right() -> None:
    """Test append_right method."""
    r = Run([1], block_size=4)
    r.append_right(2)
    assert r.size == 2
    assert r.end == 2
    assert r.to_list() == [1, 2]


def test_append_left() -> None:
    """Test append_left method."""
    r = Run([2], block_size=4)
    r.append_left(1)
    assert r.size == 2
    assert r.start == 1
    assert r.to_list() == [1, 2]


def test_insert_at_middle() -> None:
    """Test insert_at method."""
    r = Run([1, 3], block_size=4)
    r.insert_at(1, 2)
    assert r.to_list() == [1, 2, 3]


def test_insert_at_triggers_split() -> None:
    """Test insert_at method."""
    r = Run([1, 2, 3, 4], block_size=2, lower_bound_bsize=1)
    r.insert_at(2, 99)

    assert r.size == 5
    assert len(r.blocks) == 2
    assert r.blocks == [[1, 2], [99, 3, 4]]


def test_merge_right_run() -> None:
    """Test merge_right_run method."""
    r1 = Run([1, 2])
    r2 = Run([3, 4])
    r1.merge_right_run(r2)
    assert r1.to_list() == [1, 2, 3, 4]
    assert r1.start == 1
    assert r1.end == 4


def test_merge_left_run() -> None:
    """Test merge_left_run method."""
    r1 = Run([3, 4])
    r2 = Run([1, 2])
    r1.merge_left_run(r2)
    assert r1.to_list() == [1, 2, 3, 4]
    assert r1.start == 1
    assert r1.end == 4
