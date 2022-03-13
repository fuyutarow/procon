# %%

import random
import timeit

# %%

# %%

# time: O(n^2)
def insert_sort(nums: list[int]) -> list[int]:

    for i in range(len(nums) - 1):
        for j in range(i + 1, len(nums)):
            if nums[i] > nums[j]:
                nums[i], nums[j] = nums[j], nums[i]

    return nums


def test_insert_sort():
    N = int(1e4)
    nums = random.sample(range(N), N)
    assert insert_sort(nums) == nums


timeit.timeit(test_insert_sort, number=2)


# %%

# time: O(n^2)
def selection_sort(nums: list[int]) -> list[int]:

    for i in range(len(nums) - 1):
        min_n = nums[i]
        argmin = i

        for j in range(i + 1, len(nums)):
            if nums[j] < min_n:
                min_n = nums[j]
                argmin = j

        nums[i], nums[j] = nums[j], nums[i]

    return nums


def test_selection_sort():
    N = int(1e4)
    nums = random.sample(range(N), N)
    assert selection_sort(nums) == nums


timeit.timeit(test_selection_sort, number=2)

# %%

# time: O(n^2)
def bubble_sort(nums: list[int]) -> list[int]:
    for i in range(len(nums) - 1):
        for j in range(i + 1, len(nums)):
            if nums[j - 1] > nums[j]:
                nums[j - 1], nums[j] = nums[j], nums[j - 1]
    return nums


def test_bubble_sort():
    N = int(1e4)
    nums = random.sample(range(N), N)
    assert bubble_sort(nums) == nums


timeit.timeit(test_bubble_sort, number=2)
# %%

import sys
from collections import deque


def merge_sort(nums: list[int]) -> list[int]:
    if len(nums) == 1:
        return nums

    left = 0
    right = len(nums) - 1
    mid = (left + right) // 2

    left_sorted = deque(merge_sort(merge_sort(nums[:mid])))
    right_sorted = deque(merge_sort(merge_sort(nums[mid:])))

    res = []
    if left_sorted[0] <= right_sorted[0]:
        e = left_sorted.leftpop()
        res.append(e)
    else:
        e = right_sorted.leftpop()
        res.append(e)
    return res


def test_merge_sort():
    N = int(1e4)
    nums = random.sample(range(N), N)
    assert merge_sort(nums) == nums


timeit.timeit(test_merge_sort, number=2)


# %%
def quick_sort(nums: list[int]) -> list[int]:
    if len(nums) <= 1:
        return nums

    pivot = nums[-1]

    le = []
    gt = []

    for num in nums[:-1]:
        if num <= pivot:
            le.append(num)
            le = quick_sort(le)
        else:
            gt.append(num)
            gt = quick_sort(le)

    return [*le, pivot, *gt]


def test_quick_sort():
    N = int(1e4)
    nums = random.sample(range(N), N)
    assert quick_sort(nums) == nums


# %%
