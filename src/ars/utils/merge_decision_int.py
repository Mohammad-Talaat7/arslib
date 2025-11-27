"""Merge decision logic for ARS-Hash (integer variant)."""

from __future__ import annotations

from ars.base.run import Run
from ars.utils.adjacency_int import is_adjacent_left_int, is_adjacent_right_int
from ars.utils.logger import setup_logger

logger = setup_logger("MergeInt", "ars_merge_decision_int.log")


def merge_decision_int(
    value: int,
    left_run: Run[int] | None,
    right_run: Run[int] | None,
) -> str:
    """Determine whether an integer ``value`` should be merged with the left run, right run, both, or none, based on adjacency rules.

    Parameters
    ----------
    value : int
        The value to test for adjacency.
    left_run : Run[int] | None
        The run on the left side, if any.
    right_run : Run[int] | None
        The run on the right side, if any.

    Returns
    -------
    str
        One of ``"none"``, ``"left"``, ``"right"``, or ``"both"`` indicating
        the merge direction.

    """
    left_adj = is_adjacent_left_int(value, left_run) if left_run else False
    right_adj = is_adjacent_right_int(value, right_run) if right_run else False

    if left_adj and right_adj:
        result = "both"
    elif left_adj:
        result = "left"
    elif right_adj:
        result = "right"
    else:
        result = "none"

    logger.debug(
        f"merge_decision_int(value={value}, left_adj={left_adj}, right_adj={right_adj}) -> {result}"
    )
    return result
