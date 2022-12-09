import string
PRIORITY_MAP = {val: letter for letter, val in enumerate(string.ascii_letters,1)}
with open("p3_1.txt") as f:
    data = f.read().splitlines()
print("part 2: ")
iter_data = iter(data)
ans = 0
while True:
    try:
        ans += PRIORITY_MAP[set.intersection(*[set(next(iter_data)) for _ in range(3)]).pop()]
    except:
        break
print(ans)
