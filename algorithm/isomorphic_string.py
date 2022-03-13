# %%
def is_isomorphic(str_1: str, str_2: str) -> bool:
    if len(str_1) != len(str_2):
        return False

    m1 = {}
    m2 = {}
    for (i, (c1, c2)) in enumerate(zip(str_1, str_2)):
        m1.setdefault(c1, i)
        m2.setdefault(c2, i)

        if m1[c1] == m2[c2]:
            continue
        else:
            return False

    return True


def t1():
    s1 = "abcd"
    s2 = "abce"
    res = is_isomorphic(s1, s2)
    assert res == True

    s1 = "wow"
    s2 = "aea"
    res = is_isomorphic(s1, s2)
    assert res == True

    s1 = "wow"
    s2 = "aee"
    res = is_isomorphic(s1, s2)
    assert res == False


t1()

# %%
