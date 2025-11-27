"""Pytest file for testing `src/ars/base/base_sorter.py`."""

from __future__ import annotations

from typing import override

from ars.base.base_sorter import BaseSorter
from ars.base.run import Run


class MockSorter(BaseSorter[int]):
    """Minimal sorter for testing BaseSorter functionality."""

    @override
    def _process_value(self, value: int) -> None:
        # Just store runs as singletons for testing
        self.runs.append(Run([value]))

    @override
    def _get_output(self) -> list[int]:
        # Flat concatenate
        result: list[int] = []
        for run in self.runs:
            result.extend(run.to_list())
        return result


def test_base_sorter_flow() -> None:
    """Test BaseSorter Logic."""
    sorter = MockSorter()
    result = sorter.sort([1, 2, 3])
    assert result == [1, 2, 3]
    assert len(sorter.runs) == 3
