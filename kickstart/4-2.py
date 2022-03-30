# %%
from functools import reduce
import operator


def is_interst(num, prod_mp, sum_mp):
    key = num // 10
    tail = num % 10

    if not key in prod_mp:
        is_interst(key, prod_mp, sum_mp)
    if not key in sum_mp:
        is_interst(key, prod_mp, sum_mp)

    pd = prod_mp[num] = prod_mp[key] * tail
    sm = sum_mp[num] = sum_mp[key] + tail

    return pd % sm == 0


def solution(a, b, prod_mp, sum_mp):
    a = int(a)
    b = int(b)

    if b < 11:
        return str(b - a + 1)

    cnt = max(0, 10 - a)
    aa = max(10, a)
    for num in range(aa, b + 1):
        if is_interst(num, prod_mp, sum_mp):
            cnt += 1
    return str(cnt)


def main():
    prod_mp = {i: i for i in range(11)}
    sum_mp = {i: i for i in range(11)}

    T = input()
    for t in range(int(T)):
        a, b = input().split()
        r = solution(a, b, prod_mp, sum_mp)
        print(f"Case #{t+1}: {r}")


# main()


def test():
    prod_mp = {i: i for i in range(10)}
    sum_mp = {i: i for i in range(10)}

    s = "1 9"
    a, b = s.split()
    r = solution(a, b, prod_mp, sum_mp)
    print(r)
    assert r == "9"

    s = "91 99"
    a, b = s.split()
    r = solution(a, b, prod_mp, sum_mp)
    print(r)
    assert r == "0"

    s = "451 460"
    a, b = s.split()
    r = solution(a, b, prod_mp, sum_mp)
    assert r == "5"

    s = "501 1000"
    a, b = s.split()
    r = solution(a, b, prod_mp, sum_mp)
    assert r == "176"

    s = "5 712"
    a, b = s.split()
    r = solution(a, b, prod_mp, sum_mp)
    assert r == "237"

    print("ok")


test()


# %%

# %%
s = "100 120"
a, b = s.split()
r = solution(a, b, prod_mp, sum_mp)
r
# %%


def is_interst2(num):
    digits = [int(c) for c in str(num)]
    s = sum(digits)
    p = reduce(operator.mul, digits, 1)
    return p % s == 0


def solution2(a, b):
    a = int(a)
    b = int(b)

    cnt = 0
    for num in range(a, b + 1):
        if is_interst2(num):
            cnt += 1
    return str(cnt)


import random


N = 100
for i in range(100):
    prod_mp = {i: i for i in range(10)}
    sum_mp = {i: i for i in range(10)}

    a = random.choice(range(N))
    b = random.choice(range(N))
    if a < b:
        r = solution(a, b, prod_mp, sum_mp)
        r2 = solution2(a, b)

        print(a, b, r, r2)
        assert r == r2

# %%
