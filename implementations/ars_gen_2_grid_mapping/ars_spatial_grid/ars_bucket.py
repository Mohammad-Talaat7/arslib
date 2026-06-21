from __future__ import annotations
from typing import Iterable
from ._arslib import ARSBucket as RustARSBucket

class ARSBucket:
    """Ultra-optimized bucket-based ARS sorter for floats - implemented in Rust."""

    def __init__(self, tolerance: float = 1e-6, track: bool = False) -> None:
        self._sorter = RustARSBucket(tolerance)

    def sort(self, data: Iterable[float]) -> list[float]:
        if not isinstance(data, list):
            data = list(data)
        return self._sorter.sort(data)

    def sort_parallel(self, data: Iterable[float]) -> list[float]:
        if not isinstance(data, list):
            data = list(data)
        return self._sorter.sort_parallel(data)

    def process_value(self, value: float) -> None:
        self._sorter.process_value(value)

    def get_output(self) -> list[float]:
        return self._sorter.get_output()
