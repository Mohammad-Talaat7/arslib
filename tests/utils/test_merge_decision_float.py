"""Pytest file for testing `src/ars/utils/merge_decision_float.py`."""

from ars.base.run import Run
from ars.utils.merge_decision_float import merge_decision_float


def test_merge_left_decision_float() -> None:
    """Test Left Result."""
    left = Run([10.0])
    assert merge_decision_float(9.99, left, None, tolerance=0.02) == "left"


def test_merge_right_decision_float() -> None:
    """Test Right Result."""
    right = Run([10.0])
    assert merge_decision_float(10.01, None, right, tolerance=0.02) == "right"


def test_merge_both_decision_float() -> None:
    """Test Both Result."""
    left = Run([10.05])
    right = Run([9.95])
    assert merge_decision_float(10.0, left, right, tolerance=0.1) == "both"


def test_merge_none_decision_float() -> None:
    """Test None Result."""
    left = Run([10.0])
    assert merge_decision_float(7.0, left, None, tolerance=0.5) == "none"
