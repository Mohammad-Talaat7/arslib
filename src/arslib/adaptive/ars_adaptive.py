"""Adaptive ARS using Red-Black Trees."""

from __future__ import annotations

import logging
from collections.abc import Callable
from typing import Generic, cast, override

from arslib.adaptive.rb_tree import RBTree
from arslib.base.base_sorter import BaseSorter
from arslib.base.run import Run
from arslib.utils.shared_defaults import KT, T

logger = logging.getLogger(__name__)


class ARSAdapt(BaseSorter[T], Generic[KT, T]):
    """Adaptive ARS sorter backed by a Red-Black Tree of runs.

    Algorithm:
      - Maintain an RBTree keyed by key_fn(run.start) -> Run[T]
      - For each incoming value `x`:
          1. Find floor/ceiling (closest runs by start key).
          2. If any run contains `x` (run.start <= x <= run.end) -> insert_sorted there.
             * If run.start changed due to insertion at left -> replace key in tree.
          3. Else, check strict bridging:
             - If pred and succ exist and pred.end < x < succ.start -> merge (pred <- x <- succ)
          4. Else if adjacent to left (pred exists and pred.end < x) -> append_right(pred)
             else if adjacent to right (succ exists and x < succ.start) -> append_left(succ)
          5. Else create new singleton run and insert into tree.

    Notes:
      - This implementation preserves duplicates by inserting them into the containing run.
      - key_fn defaults to identity; it must produce an orderable KT.

    """

    def __init__(self, key_fn: Callable[[T], KT] | None = None) -> None:
        super().__init__()
        self.tree: RBTree[KT, Run[T]] = RBTree()
        # default key_fn is identity (assume T and KT are the same/comparable)
        if key_fn is not None:
            self.key_fn: Callable[[T], KT] = key_fn
        else:
            self.key_fn = cast(
                Callable[[T], KT],
                lambda x: x,  # pyright:ignore[reportUnknownLambdaType]
            )

    # -------------------------
    # Core required methods
    # -------------------------
    @override
    def _process_value(self, value: T) -> None:
        """Insert a single value into the adaptive structure following Option 1 rules."""
        kx = self.key_fn(value)

        # If tree empty -> create new run and insert
        if len(self.tree) == 0:
            run = Run([value])
            run_key = self.key_fn(run.start)
            _ = self.tree.insert(run_key, run)
            self.on_run_create(run)
            logger.debug(f"Tree empty -> created run {run!r} with key={run_key!r}")
            return

        # Candidates: floor (largest key <= kx) and ceiling (smallest key >= kx)
        floor_item = self.tree.floor(kx)
        ceil_item = self.tree.ceiling(kx)

        pred_key, pred_run = floor_item if floor_item is not None else (None, None)
        succ_key, succ_run = ceil_item if ceil_item is not None else (None, None)

        # --- 1) containment checks (duplicates and interior insertion) ---
        # Try pred first (floor) because it's the run with the greatest start <= kx
        if pred_run is not None:
            if self._contains(pred_run, value):
                assert pred_key is not None
                self._insert_into_run(pred_key, pred_run, value)
                return

        # Then try succ (ceiling)
        if succ_run is not None:
            if self._contains(succ_run, value):
                assert succ_key is not None
                self._insert_into_run(succ_key, succ_run, value)
                return

        # --- 2) bridging (strict adjacency between pred and succ) ---
        if (pred_run is not None) and (succ_run is not None):
            try:
                if pred_run.end < value < succ_run.start:
                    # merge: append value to pred, then merge succ into pred, and remove succ from tree
                    old_pred_key = pred_key
                    pred_run.append_right(value)
                    # merge succ into pred_run (pred absorbs succ)
                    pred_run.merge_right_run(succ_run)
                    # delete succ node from tree
                    assert succ_key is not None
                    _ = self.tree.delete(succ_key)  # remove successor
                    # pred.start unchanged => no replace_key needed
                    self.on_run_merge(pred_run, succ_run, pred_run)
                    logger.debug(
                        f"Bridged value {value!r}: merged pred {old_pred_key!r} and succ {succ_key!r} into {pred_run!r}"
                    )
                    return
            except TypeError:
                # Incomparable at runtime; fall back to new-run creation below
                pass

        # --- 3) adjacent to left only (extend pred to right) ---
        if pred_run is not None:
            try:
                if pred_run.end < value:
                    old_pred_key = pred_key
                    pred_run.append_right(value)
                    # pred.start unchanged -> no key replace required
                    logger.debug(
                        f"Appended {value!r} to right of pred run key={old_pred_key!r}"
                    )
                    return
            except TypeError:
                pass

        # --- 4) adjacent to right only (extend succ to left) ---
        if succ_run is not None:
            try:
                if value < succ_run.start:
                    # inserting at left may change succ.start -> need to replace key afterwards
                    old_succ_key = succ_key
                    # capture new start after append_left
                    succ_run.append_left(value)
                    new_key = self.key_fn(succ_run.start)
                    if new_key != old_succ_key:
                        assert old_succ_key is not None
                        self.tree.replace_key(old_succ_key, new_key)
                    logger.debug(
                        f"Appended {value!r} to left of succ run; replaced key {old_succ_key!r} -> {new_key!r}"
                    )
                    return
            except TypeError:
                pass

        # --- 5) no adjacency: create new run node ---
        new_run = Run([value])
        new_key = self.key_fn(new_run.start)
        _ = self.tree.insert(new_key, new_run)
        self.on_run_create(new_run)
        logger.debug(f"Created new run for {value!r} with key={new_key!r}")

    @override
    def _get_output(self) -> list[T]:
        """Flatten runs in-order and return sorted list."""
        out: list[T] = []
        for _, run in self.tree.inorder_items():
            out.extend(run.to_list())
        return out

    # -------------------------
    # Helpers
    # -------------------------
    @staticmethod
    def _contains(run: Run[T], value: T) -> bool:
        """Return True if run interval contains value (inclusive)."""
        try:
            return run.start <= value <= run.end
        except TypeError:
            # Fallback: search blocks (slower) for equality
            for block in run.blocks:
                for v in block:
                    if v == value:
                        return True
            return False

    def _insert_into_run(self, key: KT, run: Run[T], value: T) -> None:
        """Insert value into run (using insert_sorted). If run.start changes, update tree key."""
        old_key = key
        # Insert keeping run sorted (handles duplicates)
        run.insert_sorted(value)
        # If start changed (we inserted at left), update tree key
        new_key = self.key_fn(run.start)
        if new_key != old_key:
            # replace_key will remove old key node and reinsert with new key
            self.tree.replace_key(old_key, new_key)
            logger.debug(
                f"Run start changed after inserting {value!r}: replaced tree key {old_key!r} -> {new_key!r}"
            )
        else:
            logger.debug(f"Inserted {value!r} into existing run with key={old_key!r}")
