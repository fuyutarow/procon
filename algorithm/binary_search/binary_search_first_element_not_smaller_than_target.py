# %%


def first_not_smaller(arr: list[int], target: int) -> int:

    lo = 0
    hi = len(arr) // 2

    if len(arr) == 0:
        return -1
    elif len(arr) == 1:
        if target < arr[0]:
            return 0
        else:
            return -1

    while lo <= hi:
        mid = (lo + hi) // 2

        if arr[mid] <= target < arr[mid + 1]:
            return mid + 1
        elif arr[mid + 1] <= target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1


def test():
    assert (
        first_not_smaller(
            arr=[1, 3, 3, 45, 8, 8, 10],
            target=2,
        )
        == 1
    )

    assert (
        first_not_smaller(
            arr=[2, 3, 5, 7, 11, 13, 17, 19],
            target=6,
        )
        == 3
    )

    assert (
        first_not_smaller(
            arr=[2, 3, 5, 7, 11, 13, 17, 19],
            target=20,
        )
        == -1
    )

    assert (
        first_not_smaller(
            arr=[3],
            target=2,
        )
        == 0
    )


test()


# %%
