"""Shared Variables across all Variants."""

from __future__ import annotations

from typing import Protocol, TypeVar, runtime_checkable


# ============================================================
# Ordering Protocol
# ============================================================
@runtime_checkable
class SupportsTotalOrder(Protocol):
    """Protocol for types that support full total ordering.

    Any type used as a key in the RBTree must implement <, <=, >, >=.
    """

    def __lt__(self: KT, other: KT, /) -> bool: ...
    def __le__(self: KT, other: KT, /) -> bool: ...
    def __gt__(self: KT, other: KT, /) -> bool: ...
    def __ge__(self: KT, other: KT, /) -> bool: ...


T = TypeVar("T", bound="SupportsTotalOrder")  # Data type inside Runs
KT = TypeVar("KT", bound="SupportsTotalOrder")  # Key type for RB-tree ordering
V = TypeVar("V")  # value type for RBTree (NO ordering needed)
