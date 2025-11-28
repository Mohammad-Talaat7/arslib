"""BaseSorter: Abstract foundation for all ARS sorter variants.

This class defines:
- the public API (`sort`)
- lifecycle hooks (on_start, on_run_create, etc.)
- shared state (list of runs)
- type-safe abstract methods for variant-specific logic
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import Iterable
from typing import Generic, TypeVar

from arslib.base.run import Run
from arslib.utils.logger import setup_logger

T = TypeVar("T")

logger = setup_logger("BaseSorter", "ars_base_sorter.log")


class BaseSorter(ABC, Generic[T]):
    """Abstract base class for all ARS sorter variants.

    Variants must implement:
    - _process_value(value: T)
    - _get_output() -> List[T]
    """

    def __init__(self) -> None:
        # List of current runs, updated as sorting progresses
        self.runs: list[Run[T]] = []

    # --------------------------------------------------------
    # Public API
    # --------------------------------------------------------
    def sort(self, data: Iterable[T]) -> list[T]:
        """Sort input data using the ARS algorithm."""
        lst = list(data)
        logger.debug(f"sort() called with {len(lst)} items")

        self.on_start(lst)

        for value in lst:
            self.on_value_insert(value)
            self._process_value(value)

        result = self._get_output()
        self.on_finish(result)

        logger.debug(f"sort() finished, output size={len(result)}")
        return result

    # --------------------------------------------------------
    # Hooks (can be overridden by subclasses)
    # --------------------------------------------------------
    def on_start(self, data: list[T]) -> None:
        """Start Sort Hook."""
        logger.debug(f"Sorting started (size={len(data)})")

    def on_value_insert(self, value: T) -> None:
        """Insert Value Hook."""
        logger.debug(f"Inserting value: {value}")

    def on_run_create(self, run: Run[T]) -> None:
        """Create Run Hook."""
        logger.debug(f"Run created: {run}")

    def on_run_merge(self, left: Run[T], right: Run[T], result: Run[T]) -> None:
        """Merge Runs Hook."""
        logger.debug(f"Merged runs: left={left}, right={right} -> result={result}")

    def on_finish(self, sorted_output: list[T]) -> None:
        """Finish Sort Hook."""
        logger.debug(f"Sorting completed (size={len(sorted_output)})")

    # --------------------------------------------------------
    # Required abstract methods
    # --------------------------------------------------------
    @abstractmethod
    def _process_value(self, value: T) -> None:
        """Insert a value into the sorter.

        Must update `self.runs`.
        Must perform merges when necessary.
        """
        raise NotImplementedError

    @abstractmethod
    def _get_output(self) -> list[T]:
        """Return final sorted output from `self.runs`."""
        raise NotImplementedError

    # --------------------------------------------------------
    # Helper methods for subclasses
    # --------------------------------------------------------
    def _create_run(self, value: T) -> Run[T]:
        """Create a new Run and call hook."""
        run = Run([value])
        self.on_run_create(run)
        return run

    def _replace_runs(self, new_runs: list[Run[T]]) -> None:
        """Replace current run list (useful after merging)."""
        self.runs = new_runs

    def _merge_two_runs(self, left: Run[T], right: Run[T]) -> Run[T]:
        """Merge two runs by appending right into left.

        Safe for all variants.
        """
        left.merge_right_run(right)
        self.on_run_merge(left, right, left)
        return left
