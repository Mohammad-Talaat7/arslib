from __future__ import annotations
from typing import Iterable
from ._arslib import ARSHash as RustARSHash

class ARSHash:
    """Ultra-optimized ARS-Hash for integer sorting - implemented in Rust."""

    def __init__(self, order: int = 64, track: bool = False) -> None:
        self._sorter = RustARSHash()

    def sort(self, data: Iterable[int]) -> list[int]:
        if not isinstance(data, list):
            data = list(data)
        return self._sorter.sort(data)

    def sort_parallel(self, data: Iterable[int]) -> list[int]:
        if not isinstance(data, list):
            data = list(data)
        return self._sorter.sort_parallel(data)

    def process_value(self, value: int) -> None:
        self._sorter.process_value(value)

    def get_output(self) -> list[int]:
        return self._sorter.get_output()
