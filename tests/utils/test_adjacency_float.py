"""Pytest file for testing `src/ars/utils/adjacency_float.py`."""

from arslib.base.run import Run
from arslib.utils.adjacency_float import (
    is_adjacent_left_float,
    is_adjacent_right_float,
    runs_are_adjacent_float,
)


def test_adjacent_left_float() -> None:
    """Test is_adjacent_left_float function."""
    r = Run([10.0])
    assert is_adjacent_left_float(9.999, r, tolerance=0.01) is True
    assert is_adjacent_left_float(9.9, r, tolerance=0.01) is False


def test_adjacent_right_float() -> None:
    """Test is_adjacent_right_float function."""
    r = Run([10.0])
    assert is_adjacent_right_float(10.001, r, tolerance=0.01) is True
    assert is_adjacent_right_float(10.5, r, tolerance=0.01) is False


def test_runs_are_adjacent_float() -> None:
    """Test runs_are_adjacent_float function."""
    r1 = Run([1.0, 2.0])
    r2 = Run([3.0])

    assert runs_are_adjacent_float(r1, r2, tolerance=1.0) is True
    assert runs_are_adjacent_float(r1, r2, tolerance=0.1) is False
