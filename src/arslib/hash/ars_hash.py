"""ARS-Hash Class that perform ARS for integer values only."""

from __future__ import annotations

from typing import override

from arslib.base.base_sorter import BaseSorter
from arslib.base.run import Run
from arslib.utils.logger import setup_logger

# from ars.utils.adjacency_int import is_adjacent_left_int, is_adjacent_right_int
from arslib.utils.merge_decision_int import merge_decision_int

logger = setup_logger("ARSHash", "ars_hash_int.log")


class ARSHash(BaseSorter[int]):
    """ARS-Hash integer variant.

    Key ideas:
    - Maintain mapping:
        start_map[start_value] -> Run
        end_map[end_value] -> Run
        value_map[value] -> Run (points to a run that contains at least one copy of value)
    - Use maps to decide merges in O(1).
    - Keep runs' internal blocks sorted. When inserting duplicates, insert at proper
      position inside the run to preserve sorted order.
    - At the end, produce output by iterating runs ordered by start_map keys.
    """

    def __init__(self) -> None:
        super().__init__()
        # boundary maps
        self.start_map: dict[int, Run[int]] = {}
        self.end_map: dict[int, Run[int]] = {}
        # quick membership: maps a value -> a run that currently contains that value
        self.value_map: dict[int, Run[int]] = {}

    # ------------------------
    # Hooks & lifecycle
    # ------------------------
    @override
    def on_start(self, data: list[int]) -> None:
        """Start Sort Hook."""
        super().on_start(data)
        self.start_map.clear()
        self.end_map.clear()
        self.value_map.clear()
        # clear runs list; we'll append created runs as we go (and remove on merges)
        self.runs: list[Run[int]] = []

    @override
    def on_run_create(self, run: Run[int]) -> None:
        """Create Run Hook."""
        # add the created run to the run registry
        self.runs.append(run)
        super().on_run_create(run)

    @override
    def on_run_merge(self, left: Run[int], right: Run[int], result: Run[int]) -> None:
        """Merge Runs Hook."""
        # Remove right from run registry (left becomes result). Keep left in place.
        try:
            self.runs.remove(right)
        except ValueError:
            # right might have been already removed in earlier merges; that's fine
            pass
        super().on_run_merge(left, right, result)

    # ------------------------
    # Core processing helpers
    # ------------------------
    def _insert_value_sorted_into_run(self, run: Run[int], value: int) -> None:
        """Insert `value` into `run` ensuring the run stays sorted.

        Uses only the public Run API.
        """
        # Fast paths for adjacency
        if value < run.start:
            run.append_left(value)
            return

        if value > run.end:
            run.append_right(value)
            return

        # Value inside the run -> find global insertion index for sorted order
        global_index = 0
        # TODO:check for num-FLOPs per second to replace with run.insert_sorted
        for block in run.blocks:
            if not block:
                continue

            # If this block ends <= value, skip entire block
            if block[-1] <= value:
                global_index += len(block)
                continue

            # Find index of first element > value
            for elem in block:
                if elem > value:
                    break
                global_index += 1

            break

        # Use the public insertion API
        run.insert_at(global_index, value)

    def _map_run_values(self, run: Run[int]) -> None:
        """Update value_map for all values present in `run`.

        Note: This walks the run's values (O(run_size)) which is necessary after merges.
        """
        for v in run.to_list():
            self.value_map[v] = run

    # ------------------------
    # Main algorithm (BaseSorter required)
    # ------------------------
    @override
    def _process_value(self, value: int) -> None:
        """Insert a single integer value into the structure.

        performing adjacency-based merges as needed.
        """
        logger.debug("ARSHashInt: processing value %r", value)

        # If value already mapped to a run that contains it, insert duplicate into that run
        existing_run = self.value_map.get(value)
        if existing_run is not None:
            # Insert duplicate preserving run sorted order
            logger.debug(
                "Value %r already present in run %s — inserting duplicate",
                value,
                existing_run,
            )
            self._insert_value_sorted_into_run(existing_run, value)
            # value_map entry stays valid
            return

        # adjacency checks using boundary maps
        left_run = self.end_map.get(value - 1)
        right_run = self.start_map.get(value + 1)

        decision = merge_decision_int(value, left_run, right_run)
        logger.debug("merge_decision for %r -> %s", value, decision)

        if decision == "none":
            # create new run with single value
            run = self._create_run(value)
            # update boundary maps
            self.start_map[value] = run
            self.end_map[value] = run
            self.value_map[value] = run
            logger.debug("Created new run for %r: %s", value, run)
            return

        if decision == "left":
            # extend left run on the right with 'value'
            assert left_run is not None
            old_end = left_run.end
            # remove old end mapping
            if old_end in self.end_map:
                del self.end_map[old_end]
            # append value to right of left_run
            left_run.append_right(value)
            # update new end mapping
            self.end_map[left_run.end] = left_run
            # map this value
            self.value_map[value] = left_run
            logger.debug("Extended left run with %r -> %s", value, left_run)
            return

        if decision == "right":
            # extend right run on the left with 'value'
            assert right_run is not None
            old_start = right_run.start
            if old_start in self.start_map:
                del self.start_map[old_start]
            right_run.append_left(value)
            self.start_map[right_run.start] = right_run
            self.value_map[value] = right_run
            logger.debug("Extended right run with %r -> %s", value, right_run)
            return

        # decision == "both": bridge left and right with value in the middle
        assert left_run is not None and right_run is not None
        logger.debug(
            "Bridging runs: left=%s right=%s with value=%r", left_run, right_run, value
        )

        # Remove old boundary entries
        if left_run.end in self.end_map:
            del self.end_map[left_run.end]
        if right_run.start in self.start_map:
            del self.start_map[right_run.start]

        # We'll append value to left_run (making left.end == value), then append right_run to left_run
        left_run.append_right(value)
        # Merge right_run blocks into left_run
        left_run.merge_right_run(right_run)
        # Update maps for merged run
        self.start_map[left_run.start] = left_run
        self.end_map[left_run.end] = left_run

        # Update mapping of all values that now belong to left_run (cost = run_size)
        self._map_run_values(left_run)

        # Remove right_run from runs registry and call merge hook
        self.on_run_merge(left_run, right_run, left_run)
        logger.debug("After bridge merge run is %s", left_run)

    @override
    def _get_output(self) -> list[int]:
        """Produce final sorted output.

        For maximum runtime performance during ingestion we used boundary maps.
        For output, iterate runs by sorted start keys and flatten each run.
        Also rebuild self.runs in canonical sorted order for compatibility.
        """
        # Rebuild canonical self.runs ordered by start
        ordered_starts = sorted(self.start_map.keys())
        ordered_runs: list[Run[int]] = [self.start_map[s] for s in ordered_starts]

        # Reassign canonical runs list (useful for tests/hooks expecting runs)
        self.runs = ordered_runs

        # Flatten runs into output
        out: list[int] = []
        for run in ordered_runs:
            out.extend(run.to_list())
        return out
