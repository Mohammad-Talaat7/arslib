"""Value-to-run adjacency checks for ARS."""

from arslib.base.run import Run
from arslib.utils.logger import setup_logger

logger = setup_logger("AdjFloat", "ars_adjacency_float.log")


def is_adjacent_left_float(value: float, run: Run[float], tolerance: float) -> bool:
    """Check if value is adjacent to the left boundary of the run."""
    result = abs(value - run.start) <= tolerance
    logger.debug(f"is_adjacent_left_float({value}, {run.start}) -> {result}")
    return result


def is_adjacent_right_float(value: float, run: Run[float], tolerance: float) -> bool:
    """Check if value is adjacent to the right boundary of the run."""
    result = abs(value - run.end) <= tolerance
    logger.debug(f"is_adjacent_right_float({value}, {run.end}) -> {result}")
    return result


def runs_are_adjacent_float(
    left: Run[float], right: Run[float], tolerance: float
) -> bool:
    """Check if two runs are adjacent."""
    result = abs(left.end - right.start) <= tolerance
    logger.debug(f"runs_are_adjacent_float({left.end}, {right.start}) -> {result}")
    return result
