# %%
def binary_search(arr: list[int], target: int) -> int:

    lo = 0
    hi = len(arr) - 1

    while lo <= hi:
        mid = (lo + hi) // 2

        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1

    return -1


def test():
    arr = [10, 20]
    target = 20

    assert binary_search(arr, target) == 1

test()
# %%
