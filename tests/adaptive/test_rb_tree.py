"""Testing the Red-Black Tree Functionality and structure."""

import random
from collections.abc import Generator

import pytest

from arslib.adaptive.rb_tree import RBNode, RBTree
from arslib.utils.shared_defaults import KT, V


@pytest.fixture(autouse=True)
def deterministic_seed() -> Generator[None, None, None]:
    """Ensure deterministic behaviour for tests that use randomness."""
    random.seed(42)
    yield
    random.seed()


# ============================================================
# Helper: Validate Red–Black Tree invariants
# ============================================================


def assert_rb_properties(tree: RBTree[KT, V]) -> None:
    """Assert that the Red-Black Tree satisfies all structural invariants."""
    root: RBNode[KT, V] | None = tree.root

    # 1. Root must be black (unless empty tree)
    if root is not None:
        assert root.color is False, "RBTree invariant violated: root must be black"

    # 2. No red node may have a red child (no double-red)
    def dfs_no_double_red(node: RBNode[KT, V] | None) -> None:
        if node is None:
            return
        if node.color:  # RED
            if node.left:
                assert (
                    node.left.color is False
                ), "RBTree invariant violated: red node has red LEFT child"
            if node.right:
                assert (
                    node.right.color is False
                ), "RBTree invariant violated: red node has red RIGHT child"
        dfs_no_double_red(node.left)
        dfs_no_double_red(node.right)

    dfs_no_double_red(root)

    # 3. Every root-to-leaf NIL path must have the same black-height
    def black_height(node: RBNode[KT, V] | None) -> tuple[int, bool]:
        """Return black-height and boolean indicating whether subtree is valid."""
        if node is None:
            return 1, True  # NIL counts as black

        left_h, ok_left = black_height(node.left)
        right_h, ok_right = black_height(node.right)
        ok = ok_left and ok_right and (left_h == right_h)

        bh = left_h + (0 if node.color else 1)
        return bh, ok

    _, ok = black_height(root)
    assert ok, "RBTree invariant violated: black-height mismatch among paths"


# ============================================================
# RBTree basic operation tests
# ============================================================


def test_insert_and_get() -> None:
    """Test insertion and retrieval of values via get()."""
    tree: RBTree[int, int] = RBTree()
    for i in [5, 1, 9, 3, 7]:
        _ = tree.insert(i, i * 10)

    for i in [1, 3, 5, 7, 9]:
        assert tree.get(i) == i * 10

    assert_rb_properties(tree)


def test_inorder_items_sorted() -> None:
    """Test that inorder traversal yields sorted keys."""
    tree: RBTree[int, int] = RBTree()
    keys = [5, 1, 9, 3, 7, 2, 4, 6, 8]
    for k in keys:
        _ = tree.insert(k, k)

    items = list(tree.inorder_items())
    assert [k for k, _ in items] == sorted(keys)

    assert_rb_properties(tree)


def test_predecessor_successor() -> None:
    """Test predecessor() and successor() for correctness."""
    tree: RBTree[int, str] = RBTree()
    data = {10: "a", 20: "b", 30: "c", 40: "d"}
    for k, v in data.items():
        _ = tree.insert(k, v)

    pred_25 = tree.predecessor(25)
    succ_25 = tree.successor(25)
    assert pred_25 is not None
    assert pred_25[0] == 20
    assert tree.predecessor(10) is None

    assert succ_25 is not None
    assert succ_25[0] == 30
    assert tree.successor(40) is None

    assert_rb_properties(tree)


def test_floor_ceiling() -> None:
    """Test floor() and ceiling() with boundary and interior values."""
    tree: RBTree[int, str] = RBTree()
    keys = [5, 10, 15, 20]
    for k in keys:
        _ = tree.insert(k, str(k))

    tree_floor_16 = tree.floor(16)
    tree_floor_5 = tree.floor(5)
    assert tree_floor_16 is not None
    assert tree_floor_5 is not None
    assert tree_floor_16[0] == 15
    assert tree_floor_5[0] == 5
    assert tree.floor(2) is None

    tree_ciel_16 = tree.ceiling(16)
    tree_ciel_20 = tree.ceiling(20)
    assert tree_ciel_16 is not None
    assert tree_ciel_20 is not None
    assert tree_ciel_16[0] == 20
    assert tree_ciel_20[0] == 20
    assert tree.ceiling(100) is None

    assert_rb_properties(tree)


def test_delete_basic() -> None:
    """Test deletion of nodes from various positions in the tree."""
    keys = [20, 10, 30, 5, 15, 25, 35]
    tree: RBTree[int, int] = RBTree()
    for k in keys:
        _ = tree.insert(k, k)

    removed = tree.delete(10)
    assert removed == 10
    assert 10 not in [k for k, _ in tree.inorder_items()]

    assert_rb_properties(tree)


def test_replace_key() -> None:
    """Test replacing a key using replace_key()."""
    tree: RBTree[int, str] = RBTree()
    _ = tree.insert(10, "x")
    _ = tree.insert(20, "y")

    tree.replace_key(10, 15)

    assert tree.get(15) == "x"
    with pytest.raises(KeyError):
        _ = tree.get(10)

    assert_rb_properties(tree)


def test_duplicate_key_replaces_value() -> None:
    """Test that inserting a duplicate key replaces the stored value."""
    tree: RBTree[int, str] = RBTree()
    _ = tree.insert(10, "a")
    old = tree.insert(10, "b")

    assert old == "a"
    assert tree.get(10) == "b"
    assert len(tree) == 1

    assert_rb_properties(tree)


# ============================================================
# Fuzz / randomized tests for robustness
# ============================================================


def test_random_insert_delete_fuzz() -> None:
    """Randomized fuzz test for insertion, deletion, and invariant preservation."""
    tree: RBTree[int, int] = RBTree()
    values: dict[int, int] = {}

    # Insert many random keys
    for _ in range(100):
        k = random.randint(0, 200)
        v = random.randint(1000, 2000)
        _ = tree.insert(k, v)
        values[k] = v  # stored value always represents latest insert

    # Delete 20 random keys from the tree
    keys_to_remove = random.sample(list(values.keys()), k=20)
    for k in keys_to_remove:
        removed = tree.delete(k)
        assert removed == values[k]
        del values[k]
        assert_rb_properties(tree)

    # Remaining inorder traversal must match sorted keys
    inorder = list(tree.inorder_items())
    assert [k for k, _ in inorder] == sorted(values.keys())

    assert_rb_properties(tree)
