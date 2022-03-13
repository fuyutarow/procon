# %%


def dfs(root, target: int) -> int | None:
    if not root:
        return None
    if root.val == target:
        return root

    left = dfs(root.left, target)
    if not left:
        return left

    right = dfs(root.right, target)
    return right
