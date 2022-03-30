
# %%

import randam

for _ in range(10):


    lo,hi = sorted([random.choice(range(10)) for _ in range(2)])

    mid2 = (lo+hi)// 2

    mid = lo + (hi-lo) //2

    print(mid2, mid)

# %%
