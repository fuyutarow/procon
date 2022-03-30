# %%
from functools import reduce
import operator


def is_interst(num):
    digits = [int(c) for c in str(num)]
    s = sum(digits)
    p = reduce(operator.mul, digits, 1)
    return p % s == 0


def solution(a, b):
    a = int(a)
    b = int(b)

    cnt = 0
    for num in range(a, b + 1):
        if is_interst(num):
            cnt += 1
    return str(cnt)


def main():
    T = input()
    for t in range(int(T)):
        a, b = input().split()
        r = solution(a, b)
        print(f"Case #{t+1}: {r}")


def test():
    s = "1 9"
    a, b = s.split()
    r = solution(a, b)
    assert r == "9"

    s = "91 99"
    a, b = s.split()
    r = solution(a, b)
    assert r == "0"

    s = "451 460"
    a, b = s.split()
    r = solution(a, b)
    assert r == "5"

    s = "501 1000"
    a, b = s.split()
    r = solution(a, b)
    assert r == "176"

    s = "5 712"
    a, b = s.split()
    r = solution(a, b)
    assert r == "237"

    s = "42 66"
    a, b = s.split()
    r = solution(a, b)
    assert r == "5"

    print("ok")


test()


# %%

