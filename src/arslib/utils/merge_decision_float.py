"""Merge decision logic for ARS-Bucket (float variant)."""

from __future__ import annotations

from arslib.base.run import Run
from arslib.utils.adjacency_float import is_adjacent_left_float, is_adjacent_right_float
from arslib.utils.logger import setup_logger

logger = setup_logger("MergeFloat", "ars_merge_decision_float.log")


def merge_decision_float(
    value: float,
    left_run: Run[float] | None,
    right_run: Run[float] | None,
    tolerance: float,
) -> str:
    """Determine whether a float `value` should be merged with the left run, right run, both, or none, based on adjacency rules.

    Parameters
    ----------
    value : float
        The value to test for adjacency.
    left_run : Run[float] | None
        The run on the left side, if any.
    right_run : Run[float] | None
        The run on the right side, if any.
    tolerance : float
        Maximum allowed gap for two values to be considered adjacent.

    Returns
    -------
    str
        One of ``"none"``, ``"left"``, ``"right"``, or ``"both"`` indicating
        the merge direction.

    """
    left_adj = is_adjacent_left_float(value, left_run, tolerance) if left_run else False
    right_adj = (
        is_adjacent_right_float(value, right_run, tolerance) if right_run else False
    )

    if left_adj and right_adj:
        result = "both"
    elif left_adj:
        result = "left"
    elif right_adj:
        result = "right"
    else:
        result = "none"

    logger.debug(
        f"merge_decision_float(value={value}, left_adj={left_adj}, right_adj={right_adj}, tol={tolerance}) -> {result}"
    )
    return result
