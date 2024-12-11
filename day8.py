from collections import defaultdict

# Part 1
_map = []
with open("input-day8.txt") as f:
    for line in f.readlines():
        _map.append(list(line.strip()))

antennas = defaultdict(set)
for row in range(len(_map)):
    for col in range(len(_map[row])):
        if _map[row][col].isalnum():
            antennas[_map[row][col]].add((col, row))

antinodes = set()
for freq, coords in antennas.items():
    working = coords.copy()
    while len(working) > 0:
        cur = working.pop()
        for other in working:
            cx, cy = cur
            ox, oy = other
            dx, dy = ox - cx, oy - cy
            a0x, a0y = cx - dx, cy - dy
            a1x, a1y = ox + dx, oy + dy

            if a0x >= 0 and a0x < len(_map[0]) and a0y >= 0 and a0y < len(_map):
                antinodes.add((a0x, a0y))

            if a1x >= 0 and a1x < len(_map[0]) and a1y >= 0 and a1y < len(_map):
                antinodes.add((a1x, a1y))

print("# of antinodes:", len(antinodes))

# Part 2
antinodes = set()
for freq, coords in antennas.items():
    working = coords.copy()
    while len(working) > 0:
        cur = working.pop()
        for other in working:
            cx, cy = cur
            ox, oy = other
            dx, dy = ox - cx, oy - cy

            count = 0
            ax, ay = cx - count * dx, cy - count * dy
            while ax >= 0 and ax < len(_map[0]) and ay >= 0 and ay < len(_map):
                antinodes.add((ax, ay))
                ax, ay = cx - count * dx, cy - count * dy
                count += 1

            count = 0
            ax, ay = ox + count * dx, oy + count * dy
            while ax >= 0 and ax < len(_map[0]) and ay >= 0 and ay < len(_map):
                antinodes.add((ax, ay))
                ax, ay = ox + count * dx, oy + count * dy
                count += 1

print("# of antinodes:", len(antinodes))
