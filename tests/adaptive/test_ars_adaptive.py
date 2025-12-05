"""Pytest file for testing `src/ars/adaptive/ars_adaptive.py`."""

import random
from collections.abc import Callable, Generator

import pytest

from arslib.adaptive.ars_adaptive import ARSAdapt
from arslib.utils.shared_defaults import T


@pytest.fixture(autouse=True)
def deterministic_seed() -> Generator[None, None, None]:
    """Ensure deterministic behaviour for tests that use randomness."""
    random.seed(42)
    yield
    random.seed()


# ============================================================
# Utility
# ============================================================


def assert_sorted_equal(a: list[T], b: list[T]) -> None:
    """Assert that two lists match exactly."""
    assert a == b, f"{a} != {b}"


# ============================================================
# Basic functional tests (default key_fn)
# ============================================================


def test_single_insertion() -> None:
    """Test sorting with a single-element input."""
    s = ARSAdapt[int, int]()
    assert s.sort([5]) == [5]


def test_sorted_input() -> None:
    """Test already-sorted input produces identical output."""
    s = ARSAdapt[int, int]()
    data = [1, 2, 3, 4, 5]
    assert s.sort(data) == data


def test_reverse_sorted_input() -> None:
    """Test reverse-sorted input is sorted correctly."""
    s = ARSAdapt[int, int]()
    data = [5, 4, 3, 2, 1]
    assert s.sort(data) == sorted(data)


def test_duplicate_insertion() -> None:
    """Test that duplicates remain and are inserted correctly."""
    s = ARSAdapt[int, int]()
    data = [3, 1, 2, 3, 2, 3]
    assert s.sort(data) == sorted(data)


def test_containment_insertion() -> None:
    """Test insertion inside an existing run interval (containment case)."""
    s = ARSAdapt[int, int]()
    data = [5, 1, 3, 2]
    assert s.sort(data) == [1, 2, 3, 5]


def test_adjacent_right() -> None:
    """Test right adjacency (value extends run on the right)."""
    s = ARSAdapt[int, int]()
    assert s.sort([2, 3, 4]) == [2, 3, 4]


def test_adjacent_left() -> None:
    """Test left adjacency (value extends run on the left)."""
    s = ARSAdapt[int, int]()
    assert s.sort([5, 6, 4]) == [4, 5, 6]


def test_bridging_merge() -> None:
    """Test bridging case where pred and succ are merged via middle value."""
    s = ARSAdapt[int, int]()
    data = [2, 1, 6, 5, 3]  # 3 bridges [1,2] and [5,6]
    assert s.sort(data) == [1, 2, 3, 5, 6]


def test_new_run_creation() -> None:
    """Test new-run creation when no adjacency exists."""
    s = ARSAdapt[int, int]()
    _ = s.sort([10])  # produces run [10]
    _ = s._process_value(100)  # pyright:ignore[reportPrivateUsage]
    assert s._get_output() == [10, 100]  # pyright:ignore[reportPrivateUsage]


def test_multiple_runs_created() -> None:
    """Test several disjoint runs eventually merge or stay separate."""
    s = ARSAdapt[int, int]()
    data = [10, 1, 20, 2, 30, 3]
    assert s.sort(data) == sorted(data)


# ============================================================
# Tests with custom key functions
# ============================================================


def test_custom_key_fn_negative() -> None:
    """Test key_fn that reverses order by negating keys."""
    s = ARSAdapt[int, int](key_fn=lambda x: -x)
    data = [5, 1, 3, 2, 4]
    assert s.sort(data) == sorted(data, reverse=True)


def test_custom_key_fn_modulus() -> None:
    """Test key_fn that groups keys via modulus."""
    s = ARSAdapt[int, int](key_fn=lambda x: x % 3)
    data = [7, 4, 1, 5, 2, 6, 3]
    assert s.sort(data) == sorted(data)


def test_custom_key_fn_strings() -> None:
    """Test ARSAdapt with string values."""
    s = ARSAdapt[int, str]()
    data = ["delta", "alpha", "charlie", "bravo"]
    assert s.sort(data) == sorted(data)


def test_custom_key_fn_tuples() -> None:
    """Test tuple sorting with default tuple ordering."""
    s = ARSAdapt[int, tuple[int, str]]()
    data = [(2, "b"), (1, "a"), (2, "a"), (1, "c")]
    assert s.sort(data) == sorted(data)


# ============================================================
# Deep behavioral tests
# ============================================================


def test_incremental_insertion() -> None:
    """Test incremental manual insertions maintain correct sorted state."""
    s = ARSAdapt[int, int]()
    for x in [5, 2, 8, 6, 3]:
        s._process_value(x)  # pyright:ignore[reportPrivateUsage]
    assert s._get_output() == [2, 3, 5, 6, 8]  # pyright:ignore[reportPrivateUsage]


def test_large_gap_causes_new_run() -> None:
    """Test values far apart produce isolated runs until bridged."""
    s = ARSAdapt[int, int]()
    data = [1, 100, 2, 50, 3]
    assert s.sort(data) == sorted(data)


def test_bridging_after_multiple_runs() -> None:
    """Test bridging happens correctly after many disjoint runs."""
    s = ARSAdapt[int, int]()
    data = [1, 10, 20, 5, 15]
    assert s.sort(data) == sorted(data)


def test_left_extension_changes_key() -> None:
    """Test that left-extensions trigger RBTree key replacement."""
    s = ARSAdapt[int, int]()
    _ = s.sort([5, 6])
    s._process_value(  # pyright:ignore[reportPrivateUsage]
        4
    )  # extends left and MUST change key
    assert s._get_output() == [4, 5, 6]  # pyright:ignore[reportPrivateUsage]


# ============================================================
# Randomized property tests
# ============================================================


@pytest.mark.parametrize("seed", [1, 5, 10, 42])
def test_randomized_correctness(seed: int) -> None:
    """Property test: ARSAdapt output equals sorted() for random integers."""
    random.seed(seed)
    s = ARSAdapt[int, int]()

    data = [random.randint(-100, 100) for _ in range(200)]
    assert s.sort(data) == sorted(data)


def test_random_string_sorting() -> None:
    """Random test using ASCII strings."""
    import string

    def rand_str() -> str:
        l_asci = random.randint(1, 10)
        return "".join(random.choice(string.ascii_lowercase) for _ in range(l_asci))

    s = ARSAdapt[int, str]()
    data = [rand_str() for _ in range(200)]
    assert s.sort(data) == sorted(data)
