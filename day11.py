from collections import defaultdict


with open("input-day11.txt") as f:
    orig_stones = [int(n) for n in f.read().strip().split(" ")]

# Part 1
def blink(stones):
    new_stones = []
    for stone in stones:
        if stone == 0:
            new_stones.append(1)
            continue

        s = str(stone)
        if len(s) % 2 == 0:
            new_stones.append(int(s[:len(s) // 2]))
            new_stones.append(int(s[len(s) // 2:]))
        else:
            new_stones.append(stone * 2024)

    return new_stones


stones = orig_stones
for i in range(25):
    stones = blink(stones)
print("n stones =", len(stones))


# Part 2
registry = {0: (1,)}
def get_next(stone):
    if stone not in registry:
        s = str(stone)
        if len(s) % 2 == 0:
            registry[stone] = (int(s[:len(s) // 2]), int(s[len(s) // 2:]))
        else:
            registry[stone] = (stone * 2024,)

    return registry[stone]


def prepare(stones):
    new = defaultdict(int)
    for stone in stones:
        new[stone] += 1
    return new


def blink2(stones):
    new = defaultdict(int)
    for stone, count in stones.items():
        for next_stone in get_next(stone):
            new[next_stone] += 1 * count
    return new


stones = prepare(orig_stones)
for i in range(75):
    stones = blink2(stones)

stone_count = 0
for _, count in stones.items():
    stone_count += count

print("stone_count =", stone_count)
