# %%

import math


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def visible_tree_node(root: Node) -> int:
    def dfs(node: Node | None, max_sofar: int) -> int:
        if not node:
            return 0
        print(">", node.val, max_sofar)

        count = 0
        if node.val >= max_sofar:
            count += 1

        max_sofar = max(max_sofar, node.val)
        count += dfs(node.left, max_sofar)
        count += dfs(node.right, max_sofar)

        print("<", node.val, max_sofar, count)
        return count

    return dfs(root, -math.inf)


def build_tree(nodes, f):
    val = next(nodes)
    if val == "x":
        return None
    left = build_tree(nodes, f)
    right = build_tree(nodes, f)
    return Node(f(val), left, right)


def test():
    input = "5 4 3 x x 8 x x 6 x x"
    root = build_tree(iter(input.split()), int)
    res = visible_tree_node(root)
    print(res)
    assert res == 3

    input = "-100 x -500 x -50 x x"
    root = build_tree(iter(input.split()), int)
    res = visible_tree_node(root)
    print(res)
    assert res == 2

    input = "5 8 3 x x 8 x x 6 x x"
    root = build_tree(iter(input.split()), int)
    res = visible_tree_node(root)
    print(res)
    assert res == 4


test()

# %%
