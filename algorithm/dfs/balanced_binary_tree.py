# %%

import math


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def is_balanced(tree: Node) -> int:
    max_depth = 0

    def dfs(node: Node | None, current_depth: int, max_depth: int) -> bool:
        if not node:
            if max_depth - 1 <= current_depth <= max_depth + 1:
                return True
            else:
                return False

        current_depth += 1
        max_depth = max(max_depth, current_depth)

        return dfs(node.left, current_depth, max_depth) and dfs(
            node.right, current_depth, max_depth
        )

    return dfs(tree, 0, 0)


def build_tree(nodes, f):
    val = next(nodes)
    if val == "x":
        return None
    left = build_tree(nodes, f)
    right = build_tree(nodes, f)
    return Node(f(val), left, right)


def test():
    input = "1 2 4 x 7 x x 5 x x 3 x 6 x x"
    root = build_tree(iter(input.split()), int)
    res = is_balanced(root)
    print(res)
    assert res == True

    input = "1 2 4 x 7 x x 5 x x 3 x 6 8 x x x"
    root = build_tree(iter(input.split()), int)
    res = is_balanced(root)
    print(res)
    assert res == False

    input = "5 8 3 x x 8 x x 6 x x"
    root = build_tree(iter(input.split()), int)
    res = is_balanced(root)
    print(res)
    assert res == 4


test()

# %%
abs(1-2)

# %%
