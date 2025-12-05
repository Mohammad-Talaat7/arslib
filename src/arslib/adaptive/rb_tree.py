"""Red-Black Tree Implementation for ARSAdapt."""

from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass
from typing import Generic, override

from arslib.utils.shared_defaults import KT, V

RED = True
BLACK = False


@dataclass
class RBNode(Generic[KT, V]):
    """Red-Black tree node that keyed by KT and store Run[T]."""

    key: KT
    value: V
    color: bool = RED
    parent: RBNode[KT, V] | None = None
    left: RBNode[KT, V] | None = None
    right: RBNode[KT, V] | None = None

    @override
    def __repr__(self) -> str:
        c = "R" if self.color == RED else "B"
        return f"RBNode({self.key!r},{c})"


class RBTree(Generic[KT, V]):
    """A simple Red-Black Tree map from keys -> values.

    Important public API methods:
      - insert(key, value) -> old_value | None
      - delete(key) -> removed_value
      - get(key) -> value
      - predecessor(key) -> (k, v) | None
      - successor(key) -> (k, v) | None
      - floor(key) -> (k, v) | None  # largest key <= given
      - ceiling(key) -> (k, v) | None  # smallest key >= given
      - min_item(), max_item()
      - inorder_items() -> Iterator[(k,v)]
      - replace_key(old_key, new_key) -> None  # convenience
    """

    def __init__(self) -> None:
        self.root: RBNode[KT, V] | None = None
        self.size: int = 0

    # ----------------------
    # Rotation helpers
    # ----------------------
    def _rotate_left(self, x: RBNode[KT, V]) -> None:
        y = x.right
        if y is None:
            return
        x.right = y.left
        if y.left:
            y.left.parent = x
        y.parent = x.parent
        if x.parent is None:
            self.root = y
        elif x is x.parent.left:
            x.parent.left = y
        else:
            x.parent.right = y
        y.left = x
        x.parent = y

    def _rotate_right(self, x: RBNode[KT, V]) -> None:
        y = x.left
        if y is None:
            return
        x.left = y.right
        if y.right:
            y.right.parent = x
        y.parent = x.parent
        if x.parent is None:
            self.root = y
        elif x is x.parent.right:
            x.parent.right = y
        else:
            x.parent.left = y
        y.right = x
        x.parent = y

    # ----------------------
    # Insertion
    # ----------------------
    def insert(self, key: KT, value: V) -> V | None:
        """Insert key->value. If key exists, replace value and return old value."""
        node = RBNode(key=key, value=value, color=RED)
        y: RBNode[KT, V] | None = None
        x = self.root
        while x is not None:
            y = x
            if node.key < x.key:
                x = x.left
            elif node.key > x.key:
                x = x.right
            else:
                # key exists -> replace
                old = x.value
                x.value = value
                return old

        node.parent = y
        if y is None:
            self.root = node
        elif node.key < y.key:
            y.left = node
        else:
            y.right = node

        self.size += 1
        self._insert_fixup(node)
        return None

    def _insert_fixup(self, z: RBNode[KT, V]) -> None:
        while z.parent is not None and z.parent.color == RED:
            assert z.parent.parent is not None
            if z.parent is z.parent.parent.left:
                y = z.parent.parent.right
                if y and y.color == RED:
                    # case 1
                    z.parent.color = BLACK
                    y.color = BLACK
                    z.parent.parent.color = RED
                    z = z.parent.parent
                else:
                    assert z.parent is not None
                    if z is z.parent.right:
                        # case 2
                        z = z.parent
                        self._rotate_left(z)
                    # case 3
                    assert z.parent and z.parent.parent is not None
                    z.parent.color = BLACK
                    z.parent.parent.color = RED
                    self._rotate_right(z.parent.parent)
            else:
                assert z.parent is not None
                assert z.parent.parent is not None
                y = z.parent.parent.left
                if y and y.color == RED:
                    # case 1 (mirror)
                    z.parent.color = BLACK
                    y.color = BLACK
                    z.parent.parent.color = RED
                    z = z.parent.parent
                else:
                    assert z.parent is not None
                    if z is z.parent.left:
                        # case 2 (mirror)
                        z = z.parent
                        self._rotate_right(z)
                    # case 3 (mirror)
                    assert z.parent and z.parent.parent is not None
                    z.parent.color = BLACK
                    z.parent.parent.color = RED
                    self._rotate_left(z.parent.parent)
        assert self.root is not None
        self.root.color = BLACK

    # ----------------------
    # Search helpers
    # ----------------------
    def _search_node(self, key: KT) -> RBNode[KT, V] | None:
        x = self.root
        while x is not None:
            if key < x.key:
                x = x.left
            elif key > x.key:
                x = x.right
            else:
                return x
        return None

    def get(self, key: KT) -> V:
        """Return the value associated with specific key."""
        n = self._search_node(key)
        if n is None:
            raise KeyError(key)
        return n.value

    # ----------------------
    # Deletion
    # ----------------------
    def delete(self, key: KT) -> V:
        """Delete the value associated with specific key."""
        z = self._search_node(key)
        if z is None:
            raise KeyError(key)
        removed_value = z.value
        self._delete_node(z)
        return removed_value

    def _transplant(self, u: RBNode[KT, V], v: RBNode[KT, V] | None) -> None:
        if u.parent is None:
            self.root = v
        elif u is u.parent.left:
            u.parent.left = v
        else:
            u.parent.right = v
        if v:
            v.parent = u.parent

    def _minimum_node(self, x: RBNode[KT, V]) -> RBNode[KT, V]:
        while x.left is not None:
            x = x.left
        return x

    def _delete_node(self, z: RBNode[KT, V]) -> None:
        """Delete node z from the tree and restore RB properties.

        This implements the CLRS deletion algorithm while computing a correct
        x_parent to pass into the fixup routine (so we can handle x==None).
        """
        y = z
        y_original_color = y.color

        # x will be the node that replaces y's original position (may be None)
        x: RBNode[KT, V] | None
        # x_parent is the parent of x after the transplant (used when x is None)
        x_parent: RBNode[KT, V] | None

        if z.left is None:
            x = z.right
            x_parent = z.parent
            # transplant z with z.right
            self._transplant(z, z.right)
        elif z.right is None:
            x = z.left
            x_parent = z.parent
            # transplant z with z.left
            self._transplant(z, z.left)
        else:
            # z has two children: find successor y = min(z.right)
            y = self._minimum_node(z.right)
            assert y is not None
            y_original_color = y.color
            x = y.right
            if y.parent is z:
                # x's parent will be y after transplant (even if x is None, parent is y)
                x_parent = y
            else:
                # x_parent will be y.parent because we will transplant y with y.right
                x_parent = y.parent
                # replace y with its right child
                self._transplant(y, y.right)
                # attach z.right under y
                y.right = z.right
                if y.right:
                    y.right.parent = y

            # Now replace z with y
            self._transplant(z, y)
            # attach z.left under y
            y.left = z.left
            if y.left:
                y.left.parent = y
            # preserve color
            y.color = z.color

        # decrement size
        self.size -= 1

        # If the removed node (y) was black, we must fixup
        if y_original_color == BLACK:
            # call fixup with x and the recorded parent
            self._delete_fixup(x, x_parent)

    def _delete_fixup(
        self,
        x: RBNode[KT, V] | None,
        parent: RBNode[KT, V] | None,
    ) -> None:
        """Restore red-black properties after deletion.

        `x` is the node that moved into the position originally occupied by the
        deleted node (may be None). `parent` is the parent of `x` after the
        transplant (useful when x is None).
        """
        # Loop while x is not root and x is black (or x is None treated as black)
        while (x is not self.root) and (x is None or x.color == BLACK):
            # If parent is None, we cannot continue; break out (should be safe)
            if parent is None:
                break

            if x is parent.left:
                w = parent.right
                # Case 1: sibling is red
                if w is not None and w.color == RED:
                    w.color = BLACK
                    parent.color = RED
                    self._rotate_left(parent)
                    # after rotation, update sibling
                    w = parent.right

                # Now sibling w is black (or None)
                if (w is None) or (
                    (w.left is None or w.left.color == BLACK)
                    and (w.right is None or w.right.color == BLACK)
                ):
                    # Case 2: both of sibling's children are black -> recolor sibling red
                    if w:
                        w.color = RED
                    # move up the tree
                    x = parent
                    parent = x.parent
                else:
                    # Case 3: sibling's right child is black, left child red -> rotate right at sibling
                    if w.right is None or w.right.color == BLACK:
                        if w.left:
                            w.left.color = BLACK
                        w.color = RED
                        self._rotate_right(w)
                        w = parent.right  # update sibling after rotation

                    # Case 4: sibling's right child is red -> rotate left at parent
                    if w:
                        w.color = parent.color
                        if w.right:
                            w.right.color = BLACK
                    parent.color = BLACK
                    self._rotate_left(parent)
                    # make x the root to finish
                    x = self.root
                    parent = None
            else:
                # Mirror cases: x is parent's right child
                w = parent.left
                if w is not None and w.color == RED:
                    w.color = BLACK
                    parent.color = RED
                    self._rotate_right(parent)
                    w = parent.left

                if (w is None) or (
                    (w.left is None or w.left.color == BLACK)
                    and (w.right is None or w.right.color == BLACK)
                ):
                    if w:
                        w.color = RED
                    x = parent
                    parent = x.parent
                else:
                    if w.left is None or w.left.color == BLACK:
                        if w.right:
                            w.right.color = BLACK
                        w.color = RED
                        self._rotate_left(w)
                        w = parent.left

                    if w:
                        w.color = parent.color
                        if w.left:
                            w.left.color = BLACK
                    parent.color = BLACK
                    self._rotate_right(parent)
                    x = self.root
                    parent = None

        # Ensure x is black before finishing (also handles x is None case by no-op)
        if x:
            x.color = BLACK
        if self.root:
            self.root.color = BLACK

    # ----------------------
    # Predecessor / Successor / floor / ceiling
    # ----------------------
    def _maximum_node(self, x: RBNode[KT, V]) -> RBNode[KT, V]:
        while x.right is not None:
            x = x.right
        return x

    def predecessor(self, key: KT) -> tuple[KT, V] | None:
        """Return the largest key that are smaller than the given key with it's value as well."""
        # largest key < given key
        x = self.root
        pred: RBNode[KT, V] | None = None
        while x is not None:
            if key <= x.key:
                x = x.left
            else:
                pred = x
                x = x.right
        if pred:
            return pred.key, pred.value
        return None

    def successor(self, key: KT) -> tuple[KT, V] | None:
        """Return the smallest key that are bigger than the given key with it's value as well."""
        # smallest key > given key
        x = self.root
        succ: RBNode[KT, V] | None = None
        while x is not None:
            if key >= x.key:
                x = x.right
            else:
                succ = x
                x = x.left
        if succ:
            return succ.key, succ.value
        return None

    def floor(self, key: KT) -> tuple[KT, V] | None:
        """Return the largest key that are smaller than or equal the given key with it's value as well."""
        # largest key <= given key
        x = self.root
        res: RBNode[KT, V] | None = None
        while x is not None:
            if key < x.key:
                x = x.left
            else:
                res = x
                x = x.right
        if res:
            return res.key, res.value
        return None

    def ceiling(self, key: KT) -> tuple[KT, V] | None:
        """Return the smallest key that are bigger than or equal the given key with it's value as well."""
        # smallest key >= given key
        x = self.root
        res: RBNode[KT, V] | None = None
        while x is not None:
            if key > x.key:
                x = x.right
            else:
                res = x
                x = x.left
        if res:
            return res.key, res.value
        return None

    # ----------------------
    # Convenience / traversal
    # ----------------------
    def min_item(self) -> tuple[KT, V] | None:
        """Return the minimum key with its value."""
        if self.root is None:
            return None
        n = self._minimum_node(self.root)
        return n.key, n.value

    def max_item(self) -> tuple[KT, V] | None:
        """Return the maximum key with its value."""
        if self.root is None:
            return None
        n = self._maximum_node(self.root)
        return n.key, n.value

    def inorder_items(self) -> Iterator[tuple[KT, V]]:
        """Yield all nodes in ascending order of their keys."""

        def _inorder(node: RBNode[KT, V] | None) -> Iterator[tuple[KT, V]]:
            if node is None:
                return
            yield from _inorder(node.left)
            yield node.key, node.value
            yield from _inorder(node.right)

        yield from _inorder(self.root)

    def replace_key(self, old_key: KT, new_key: KT) -> None:
        """Extract node with old_key and reinsert with new_key preserving value.

        Raises KeyError if old_key not present. If new_key already exists, it will be replaced.
        """
        node = self._search_node(old_key)
        if node is None:
            raise KeyError(old_key)
        val = node.value
        _ = self.delete(old_key)
        _ = self.insert(new_key, val)

    def __len__(self) -> int:
        return self.size
