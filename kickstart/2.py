# %%


def solution(s):
    digits = [int(c) for c in s]
    sums = sum(digits)

    if s == "9":
        return "90"

    # insert_num = 9 - sums % 9
    insert_num = 9 - sums % 9

    if insert_num == 9:
        digits = digits[:1] + ["0"] + digits[1:]
        rr = "".join([str(di) for di in (digits)])
        return rr

    inserted = False

    for i in range(len(digits)):
        if insert_num < digits[i]:
            digits.insert(i, str(insert_num))

            return "".join(digits)

    return "".join([*digits, insert_num])


def main():
    T = input()
    for t in range(int(T)):
        s = input()
        r = solution(s)
        print(f"Case #{t+1}: {r}")


# main()


def test():
    s = "9"
    r = solution(s)
    print(r)
    assert r == "90"

    s = "5"
    r = solution(s)
    print(r)
    assert r == "45"

    s = "33"
    r = solution(s)
    print(r)
    assert r == "333"

    s = "354"
    r = solution(s)
    print(r)
    assert r == "3546"

    s = "364"
    r = solution(s)
    print(r)
    assert r == "3645"

    s = "12121"
    r = solution(s)
    print(r)
    assert r == "121212"

    s = "999"
    r = solution(s)
    print(r)
    assert r == "9099"

    s = "990"
    r = solution(s)
    print(r)
    assert r == "9090"

    print("ok")


test()


# %%
# Challenge Nine
def newNumber(x):
    s = 0
    ny = []
    for i in x:
        ny.append(int(i))
    insertNumber = (9 - sum(ny) % 9) % 9
    if insertNumber != 0:
        for i in range(len(ny)):
            if insertNumber < ny[i]:
                x.insert(i, str(insertNumber))
                return "".join(x)
    else:
        for i in range(1, len(ny)):
            if insertNumber < ny[i]:
                x.insert(i, str(insertNumber))
                return "".join(x)
    return "".join(x) + str(insertNumber)


for i in range(12100, 12200):
    r = solution(str(i))
    r2 = newNumber(list(str(i)))

    print(r, r2)
    assert r == r2


# %%
