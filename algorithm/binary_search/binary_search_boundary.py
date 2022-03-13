# %%

from xml.etree.ElementPath import find


def find_boundary(arr: list[bool]) -> int:
    lo = 0
    hi = len(arr) - 1

    res = -1
    while lo <= hi:
        mid = (lo + hi) // 2

        if arr[mid]:
            res = mid
            hi = mid - 1
        else:
            lo = mid + 1

    return res


def test():
    arr = [False, False, True, True, True]
    assert find_boundary(arr) == 2

    arr = [False, False, False, True, True]
    assert find_boundary(arr) == 3

    arr = []
    assert find_boundary(arr) == -1

    arr = [True]
    assert find_boundary(arr) == 0

    arr = [True, True]
    assert find_boundary(arr) == 0

    arr = [False, True]
    assert find_boundary(arr) == 1


test()

# %%
3 != 4 != 5
# %%
