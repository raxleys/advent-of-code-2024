with open("input-day9.txt") as f:
    data = f.read().strip()

# Part 1
disk = []
_id = 0
for i, n in enumerate((int(n) for n in data)):
    if i % 2 == 0:
        disk += [_id] * n
        _id += 1
    else:
        disk += [-1] * n

# For part 2
disk_copy = disk.copy()

fp, bp = 0, len(disk) - 1
while True:
    while disk[fp] != -1:
        fp += 1

    while disk[bp] == -1:
        bp -= 1

    if fp >= bp:
        break

    disk[fp] = disk[bp]
    disk[bp] = -1

checksum = 0
for i, n in enumerate(disk):
    if n == -1:
        break

    checksum += i * n
print("checksum =", checksum)

# Part 2
disk = disk_copy

fp = 0
bp = len(disk) - 1
while bp > 0:
    while disk[bp] == -1:
        bp -= 1

    sbp = bp
    while disk[bp] == disk[sbp]:
        bp -= 1

    size = sbp - bp

    fp = 0
    while True:
        while fp <= bp and disk[fp] != -1:
            fp += 1

        sfp = fp
        while fp <= bp and disk[fp] == -1:
            fp += 1

        free = fp - sfp
        if free < size and fp > bp:
            # No space
            break

        if size <= free:
            # Move
            for i in range(size):
                disk[sfp + i] = disk[sbp - i]
                disk[sbp - i] = -1
            break

checksum = 0
for i, n in enumerate(disk):
    if n == -1:
        continue

    checksum += i * n
print("checksum =", checksum)
