# %%
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def tree_max_depth(root: Node) -> int:
    def dfs(node: Node | None, depth: int) -> int:
        if not node:
            return depth

        return max(dfs(node.left, depth), dfs(node.right, depth)) + 1

    return dfs(root, 0)


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
    res = tree_max_depth(root)
    assert res == 3


test()


# %%
