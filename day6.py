# Part 1
_map = []
with open("input-day6.txt") as f:
    for line in f.readlines():
        _map.append(list(line.strip()))

class Vec:
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y

    def __add__(self, o):
        return Vec(self.x + o.x, self.y + o.y)

    def __repr__(self):
        return f"({self.x}, {self.y})"

    def right(self):
        if self.x == 0 and self.y == 1:
            return Vec(-1, 0)
        elif self.x == 0 and self.y == -1:
            return Vec(1, 0)
        elif self.x == 1 and self.y == 0:
            return Vec(0, 1)
        elif self.x == -1 and self.y == 0:
            return Vec(0, -1)
        else:
            raise Exception("Invalid configuration:", self)

    def tup(self):
        return (self.x, self.y)

for i in range(len(_map)):
    for j in range(len(_map[i])):
        if _map[j][i] == '^':
            pos = Vec(i, j)
            i = len(_map)
            break

print("start =", pos)
start = pos  # keep track for part 2

#       x,  y
v = Vec(0, -1)
positions = {pos.tup()}
while True:
    npos = pos + v
    if npos.x < 0 or npos.y < 0 or npos.x >= len(_map[0]) or npos.y >= len(_map):
        break

    if _map[npos.y][npos.x] == '#':
        v = v.right()
    else:
        tmp = pos
        pos = npos
        positions.add(pos.tup())

print("count =", len(positions))

# Part 2
def is_loop(pos, _map):
    """If we land on the same tile in the same direction, it's a loop."""
    v = Vec(0, -1)
    posdirs = {(pos.tup(), v.tup())}
    while True:
        npos = pos + v
        if npos.x < 0 or npos.y < 0 or npos.x >= len(_map[0]) or npos.y >= len(_map):
            return False
        elif (npos.tup(), v.tup()) in posdirs:
            return True

        if _map[npos.y][npos.x] == '#':
            v = v.right()
        else:
            pos = npos

        posdirs.add((pos.tup(), v.tup()))

# Brute force solution. Takes a while (~40 seconds on my PC)
loops = 0
for row in range(len(_map)):
    for col in range(len(_map[row])):
        if _map[row][col] == '.':
            _map[row][col] = '#'
            if is_loop(start, _map):
                loops += 1
            _map[row][col] = '.'

print("loops =", loops)
