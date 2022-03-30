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


for i in range(int(input())):
    print("Case #{}: {}".format(i + 1, newNumber(list(input()))))
