"""Value-to-run adjacency checks for ARS."""

from ars.base.run import Run
from ars.utils.logger import setup_logger

logger = setup_logger("AdjInt", "ars_adjacency_int.log")


def is_adjacent_left_int(value: int, run: Run[int]) -> bool:
    """Check if value is adjacent to the left boundary of the run."""
    result = value == run.start - 1
    logger.debug(f"is_adjacent_left_int({value}, {run.start}) -> {result}")
    return result


def is_adjacent_right_int(value: int, run: Run[int]) -> bool:
    """Check if value is adjacent to the right boundary of the run."""
    result = value == run.end + 1
    logger.debug(f"is_adjacent_right_int({value}, {run.end}) -> {result}")
    return result


def runs_are_adjacent_int(left: Run[int], right: Run[int]) -> bool:
    """Check if two runs are adjacent."""
    result = left.end + 1 == right.start
    logger.debug(f"runs_are_adjacent_int({left.end}, {right.start}) -> {result}")
    return result
