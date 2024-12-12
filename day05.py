from collections import defaultdict

with open("input-day05.txt") as f:
    data = f.read()

# Part 1
ordering = []
orders = []
saw_newline = False
for line in data.split("\n"):
    line = line.strip()
    if line == "":
        saw_newline = True
        continue

    if not saw_newline:
        ordering.append(tuple((int(n) for n in line.split("|"))))
    else:
        orders.append([int(n) for n in line.split(",")])

rules = defaultdict(set)
for first, second in ordering:
    rules[second].add(first)

good = []
for order in orders:
    bad = False
    for i in range(len(order)):
        head = order[i]
        tail = order[i + 1:]
        for n in tail:
            if n in rules[head]:
                bad = True
                break

    if not bad:
        good.append(order)

tot = 0
for order in good:
    tot += order[len(order) // 2]

print("total =", tot)

# Part 2
bad = []
for order in orders:
    if order not in good:
        bad.append(order)

def sort(order, rules):
    if len(order) <= 1:
        return order

    head = order[0]
    tail = order[1:]
    before = []
    after = []

    for n in tail:
        if n in rules[head]:
            before.append(n)
        else:
            after.append(n)

    return sort(before, rules) + [head] + sort(after, rules)

tot = 0
for order in bad:
    _sorted = sort(order, rules)
    tot += _sorted[len(_sorted) // 2]

print("total =", tot)
