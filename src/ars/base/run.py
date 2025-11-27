"""Run representation for the ARS (Adjacency Run Sort) algorithm.

Implementation notes
--------------------
This Run stores elements in fixed-size "blocks" (chunked lists).

Blocks keep insertion at both ends O(1) amortized and make insertion at an arbitrary index
fast (bounded by BLOCK_SIZE).

Blocks split when they grow too large.

This gives predictable, tunable performance.
"""

from __future__ import annotations

from collections.abc import Iterable
from dataclasses import dataclass, field
from typing import Generic, TypeVar, override

from ars.utils.logger import setup_logger

logger = setup_logger("Run", "ars_run.log")

T = TypeVar("T")


@dataclass
class Run(Generic[T]):
    """Run implemented as a list of blocks (chunked array).

    Parameters
    ----------
    values : Iterable[T]
        Initial elements for the run. Must be non-empty.
    block_size : int, optional
        Target block size (tunable). Default is 64.
    lower_bound_bsize : int, optional
        Minimum allowed block size. Default is 8.

    Attributes
    ----------
    values : Iterable[T]
        Initial iterable used to construct the run.
    block_size : int
        Computed or provided block size (after applying lower bound).
    lower_bound_bsize : int
        Minimum allowable block size.
    blocks : list[list[T]]
        Internal list of blocks storing the run's values.
    start : T
        First element of the run.
    end : T
        Last element of the run.

    Notes
    -----
    The run is internally represented as a list of blocks of at-most
    ``block_size`` elements each. This structure provides efficient
    appends at both ends and reasonably fast random inserts.

    """

    values: Iterable[T]
    block_size: int = 64
    lower_bound_bsize: int = 8

    blocks: list[list[T]] = field(init=False, default_factory=list)
    _size: int = field(init=False, default=0)

    start: T = field(init=False)
    end: T = field(init=False)

    def __post_init__(self) -> None:
        vals = list(self.values)
        if not vals:
            raise ValueError("Run cannot be initialized with empty values.")
        self._size = 0
        self.blocks = []
        self.block_size = max(
            self.lower_bound_bsize, int(self.block_size)
        )  # sanity lower bound

        # Chunk Initial Values
        for i in range(0, len(vals), self.block_size):
            block = vals[i : i + self.block_size]
            self.blocks.append(block)
            self._size += len(block)

        self._refresh_bounds()
        logger.debug(
            f"Created Run(start={self.start}, end={self.end}, size={self.size}, blocks={len(self.blocks)}, block_size={self.block_size})"
        )

    # -------------------------
    # Basic properties
    # -------------------------
    @property
    def size(self) -> int:
        """Return size of the run."""
        return self._size

    # -------------------------
    # Internal helpers
    # -------------------------
    def _refresh_bounds(self) -> None:
        """Update cached start/end from blocks."""
        if not self.blocks or self._size == 0:
            raise RuntimeError("Run became empty unexpectedly.")
        self.start = self.blocks[0][0]
        self.end = self.blocks[-1][-1]

    def _ensure_block_for_left(self) -> None:
        """Ensure there is a block at left we can append to."""
        if not self.blocks or len(self.blocks[0]) >= self.block_size:
            self.blocks.insert(0, [])
        # else: there's room in first block

    def _ensure_block_for_right(self) -> None:
        """Ensure there is a block at right we can append to."""
        if not self.blocks or len(self.blocks[-1]) >= self.block_size:
            self.blocks.append([])
        # else: there's room in last block

    def _locate(self, index: int) -> tuple[int, int]:
        """Locate the block index and inner offset for a global index.

        Raises IndexError if out of range.
        """
        if index < 0 or index >= self._size:
            raise IndexError("index out of range")
        # scan blocks; number of blocks ≈ size/block_size (small)
        offset = 0
        for b_idx, block in enumerate(self.blocks):
            if offset + len(block) > index:
                inner = index - offset
                return b_idx, inner
            offset += len(block)
        # should not reach here
        raise IndexError("index out of range after scanning blocks")

    def _maybe_split_block(self, b_idx: int) -> None:
        """If block at b_idx is too large, split it into two roughly equal blocks."""
        block = self.blocks[b_idx]
        if len(block) <= 2 * self.block_size:
            return
        mid = len(block) // 2
        left = block[:mid]
        right = block[mid:]
        self.blocks[b_idx] = left
        self.blocks.insert(b_idx + 1, right)
        logger.debug(f"Split block {b_idx} into sizes {len(left)},{len(right)}")

    # -------------------------
    # Mutating operations
    # -------------------------
    def append_right(self, value: T) -> None:
        """Append a single value to the right/end in O(1) amortized."""
        self._ensure_block_for_right()
        self.blocks[-1].append(value)
        self._size += 1
        self.end = value
        logger.debug(f"append_right: added {value}; new size={self._size}")

        # optionally split if block grown too large
        self._maybe_split_block(len(self.blocks) - 1)

    def append_left(self, value: T) -> None:
        """Append a single value to the left/start in O(1) amortized."""
        self._ensure_block_for_left()
        self.blocks[0].insert(0, value)
        self._size += 1
        self.start = value
        logger.debug(f"append_left: added {value}; new size={self._size}")

        self._maybe_split_block(0)

    def insert_at(self, index: int, value: T) -> None:
        """Insert value at global index.

        Complexity: O(block_size + #blocks scan). In practice this is tuned by
        block_size (default 64) to be very fast for typical workloads.
        """
        if index < 0 or index > self._size:
            raise IndexError("index out of range for insert")

        if index == self._size:
            return self.append_right(value)
        if index == 0:
            return self.append_left(value)

        b_idx, inner = self._locate(index)
        self.blocks[b_idx].insert(inner, value)
        self._size += 1
        logger.debug(
            f"insert_at: index={index} -> block={b_idx}, inner={inner}, value={value}"
        )
        # update bounds
        self._refresh_bounds()
        # keep blocks balanced
        self._maybe_split_block(b_idx)

    # -------------------------
    # Merging operations
    # -------------------------
    def merge_right_run(self, other: Run[T]) -> None:
        """Merge another run to the right by concatenating blocks (O(#blocks_other))."""
        # shallow-copy other's blocks — safe because we'll not mutate other's blocks after merge
        for block in other.blocks:
            self.blocks.append(list(block))
            self._size += len(block)
        self._refresh_bounds()
        logger.debug(
            f"merge_right_run: merged run of size {other.size}; new size={self._size}"
        )
        # optionally split the last block if required
        self._maybe_split_block(len(self.blocks) - 1)

    def merge_left_run(self, other: Run[T]) -> None:
        """Merge another run to the left."""
        # insert other's blocks at front
        for block in reversed(other.blocks):
            self.blocks.insert(0, list(block))
            self._size += len(block)
        self._refresh_bounds()
        logger.debug(
            f"merge_left_run: merged run of size {other.size} to left; new size={self._size}"
        )
        self._maybe_split_block(0)

    # -------------------------
    # Utilities
    # -------------------------
    def to_list(self) -> list[T]:
        """Return flattened list of run values."""
        if not self.blocks:
            return []
        out: list[T] = []
        for block in self.blocks:
            out.extend(block)
        return out

    @override
    def __repr__(self) -> str:
        return f"Run(start={self.start!r}, end={self.end!r}, size={self.size}, blocks={len(self.blocks)})"
