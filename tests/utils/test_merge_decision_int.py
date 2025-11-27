"""Pytest file for testing `src/ars/utils/merge_decision_int.py`."""

from ars.base.run import Run
from ars.utils.merge_decision_int import merge_decision_int


def test_merge_left_decision_int() -> None:
    """Test Left Result."""
    left = Run([10])
    assert merge_decision_int(9, left_run=left, right_run=None) == "left"


def test_merge_right_decision_int() -> None:
    """Test Right Result."""
    right = Run([10])
    assert merge_decision_int(11, left_run=None, right_run=right) == "right"


def test_merge_both_decision_int() -> None:
    """Test Both Result."""
    left = Run([12])
    right = Run([10])
    assert merge_decision_int(11, left, right) == "both"


def test_merge_none_decision_int() -> None:
    """Test None Result."""
    left = Run([10])
    assert merge_decision_int(7, left, None) == "none"
