"""Pytest file for testing `src/ars/utils/adjacency_int.py`."""

from ars.base.run import Run
from ars.utils.adjacency_int import (
    is_adjacent_left_int,
    is_adjacent_right_int,
    runs_are_adjacent_int,
)


def test_adjacent_left_int() -> None:
    """Test is_adjacent_left_int function."""
    r = Run([10])
    assert is_adjacent_left_int(9, r) is True
    assert is_adjacent_left_int(11, r) is False


def test_adjacent_right_int() -> None:
    """Test is_adjacent_right_int function."""
    r = Run([10])
    assert is_adjacent_right_int(11, r) is True
    assert is_adjacent_right_int(9, r) is False


def test_runs_are_adjacent_int() -> None:
    """Test runs_are_adjacent_int function."""
    r1 = Run([1, 2])
    r2 = Run([3, 4])
    assert runs_are_adjacent_int(r1, r2) is True

    r3 = Run([5, 6])  # 4 + 1 == 5 → adjacent
    assert runs_are_adjacent_int(r2, r3) is True

    r4 = Run([10, 11])  # 6 + 1 != 10 → NOT adjacent
    assert runs_are_adjacent_int(r3, r4) is False
