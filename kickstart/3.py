# %%


def is_palin(s):
    left = 0
    right = len(s) - 1
    while left < right:
        if "?" in [s[left], s[right]]:
            left += 1
            right -= 1
        elif s[left] != s[right]:
            return False
        else:
            left += 1
            right -= 1
    return True


def solution(s):
    res = False
    for i in range(len(s)):
        for j in range(i + 1, len(s) + 1):
            if not i + 5 <= j:
                continue

            palin_enable = is_palin(s[i:j])
            if not palin_enable:
                return "IMPOSSIBLE"
    return "POSSIBLE"


def main():
    T = input()
    for t in range(int(T)):
        _ = input()
        s = input()
        r = solution(s)
        print(f"Case #{t+1}: {r}")


# main()


def test():
    s = "100???001"
    r = solution(s)
    print(r)
    assert r == "IMPOSSIBLE"

    s = "100??"
    r = solution(s)
    print(r)
    assert r == "POSSIBLE"

    s = "0000??000"
    r = solution(s)
    print(r)
    assert r == "POSSIBLE"

    s = "00"
    r = solution(s)
    print(r)
    assert r == "IMPOSSIBLE"

    s = "??????"
    r = solution(s)
    print(r)
    assert r == "POSSIBLE"

    print("ok")


test()


# %%
([4, 5])

# %%
