# %%
from typing import List


def remove_duplicates(arr: List[int]) -> int:
    slow = 0
    for fast in range(len(arr)):
        if arr[slow] < arr[fast]:
            slow += 1
            arr[slow] = arr[fast]
    return slow + 1


def test():
    input = '0 0 1 1 1 2 2'
    arr = [int(x) for x in input.split()]
    res = remove_duplicates(arr)
    output = ' '.join(map(str, arr[:res]))
    assert output == '0 1 2'
test()


# %%
arr = [2, 3, 4, 5, 5]
arr[[2]]

# %%
