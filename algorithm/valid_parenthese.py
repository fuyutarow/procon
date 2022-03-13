# %%
def valid_parentheses(s: str) -> bool:

    brackets = [
        ("(", ")"),
        ("{", "}"),
        ("[", "]"),
    ]
    stack = []
    bras = [bracket[0] for bracket in brackets]
    ket2bra = {bracket[1]: bracket[0] for bracket in brackets}
    for c in s:
        if c in bras:
            stack.append(c)
        elif c in ket2bra:
            if stack and stack.pop() == ket2bra[c]:
                continue
            else:
                return False
        else:
            raise Exception("unexpected cha")

    if stack:
        return False
    else:
        return True


def test():
    s = "(())()"
    assert valid_parentheses(s) == True

    s = "(]"
    assert valid_parentheses(s) == False

    s = ""
    assert valid_parentheses(s) == True

    s = ")("
    assert valid_parentheses(s) == False


test()

# %%
