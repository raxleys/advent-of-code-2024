with open("input-day10.txt") as f:
    data = []
    for l in f.readlines():
        data.append([int(n) for n in list(l.strip())])

# Part 1
def height(data, node):
    return data[node[1]][node[0]]


def siblings(data, node):
    x, y = node
    sibs = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
    return [sib for sib in sibs if sib[0] >= 0 and sib[1] >= 0 and sib[0] < len(data[0]) and sib[1] < len(data)]


def reachable_9s(data, node, parent):
    if height(data, node) == 9:
        return {node}

    reachables = set()
    for sib in siblings(data, node):
        if sib != parent and height(data, sib) - height(data, node) == 1:
            reachables.update(reachable_9s(data, sib, node))

    return reachables


_sum = 0
for j in range(len(data)):
    for i in range(len(data[j])):
        if height(data, (j, i)) == 0:
            _sum += len(reachable_9s(data, (j, i), None))

print("sum:", _sum)

# Part 2
def reachable_9s_2(data, node, parent):
    if height(data, node) == 9:
        return 1

    score = 0
    for sib in siblings(data, node):
        if sib != parent and height(data, sib) - height(data, node) == 1:
            score += reachable_9s_2(data, sib, node)

    return score


_sum = 0
for j in range(len(data)):
    for i in range(len(data[j])):
        if height(data, (j, i)) == 0:
            print((j, i), "has", reachable_9s_2(data, (j, i), None), "reachable 9s")
            _sum += reachable_9s_2(data, (j, i), None)


print("sum:", _sum)
