# %%


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def visible_tree_node(root: Node) -> int:
    def dfs(node: Node | None, max_sofar: int) -> int:
        if not node:
            return 0

        count = 0
        if count.val > max_sofar:
            count += 1
        count += dfs(
            node.left,
        )

        if node.left and node.val < node.left.val:
            count += dfs(node.left) + 1
        if node.right and node.val < node.right.val:
            count += dfs(node.right) + 1
        return count

    return dfs(root, root.val)


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
    assert res == 3


test()

# %%
