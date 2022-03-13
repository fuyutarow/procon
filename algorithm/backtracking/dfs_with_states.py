# %%

from dataclasses import dataclass


@dataclass
class Node:
    val: int
    children: list[int] = []


def ternary_tree_paths(root: Node) -> list[str]:

    res = []

    def def
    while node.children:
        stack = []
        for child in node.children:
            child


    return []


def build_tree(nodes, f):
    val = next(nodes)
    num = int(next(nodes))
    children = [build_tree(nodes, f) for _ in range(num)]
    return Node(f(val), children)



# %%

