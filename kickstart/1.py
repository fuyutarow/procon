# %%
def solution(I, P):
    slow = 0

    for fast in range(len(P)):
        if P[fast] == I[slow]:
            slow += 1

        if slow >= len(I):
            r = len(P) - len(I)
            return str(r)
    return "IMPOSSIBLE"


def main():
    T = input()
    for t in range(int(T)):
        I = input()
        P = input()
        r = solution(I, P)
        print(f"Case #{t+1}: {r}")


main()


# def test():
#     (
#         I,
#         P,
#     ) = """aaaa
# aaaaa""".split(
#         "\n"
#     )
#     r = solution(I, P)
#     assert r == "1"

#     (
#         I,
#         P,
#     ) = """bbbbb
# bbbbc""".split(
#         "\n"
#     )
#     r = solution(I, P)
#     assert r == "IMPOSSIBLE"

#     (
#         I,
#         P,
#     ) = """Ilovecoding
# IIllovecoding""".split(
#         "\n"
#     )
#     r = solution(I, P)
#     assert r == "2"

#     (
#         I,
#         P,
#     ) = """KickstartIsFun
# kkickstartiisfun""".split(
#         "\n"
#     )
#     r = solution(I, P)
#     assert r == "IMPOSSIBLE"

#     print("ok")


# test()

# # %%

# for s in "as":
#     print(s)
# # %%
