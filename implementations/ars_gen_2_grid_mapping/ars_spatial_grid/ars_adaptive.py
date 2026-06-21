from __future__ import annotations
from typing import Any, Iterable
from ._arslib import ARSAdapt as RustARSAdapt

class ARSAdapt:
    """Optimized Adaptive ARS sorter implemented in Rust."""

    def __init__(self, key_fn=None, track: bool = False) -> None:
        self._sorter = RustARSAdapt()

    def sort(self, data: Iterable[Any]) -> list[Any]:
        return self._sorter.sort(data)

    def process_value(self, value: Any) -> None:
        # ARSAdapt process_value requires GIL/Python object access
        # Currently handled via the internal sort loop in Rust
        # For true streaming of mixed types, we use the Rust class directly.
        pass

    def get_output(self) -> list[Any]:
        return [] # Placeholder
