from collections import defaultdict

# Part 1
with open("input-day01.txt") as f:
    lines = [l.strip() for l in f.readlines()]

firsts = []
seconds = []
for l in lines:
    f, s = [w for w in l.split(" ") if w != ""]
    firsts.append(int(f))
    seconds.append(int(s))

firsts.sort()
seconds.sort()

tot = 0
for i in range(len(firsts)):
    tot += abs(seconds[i] - firsts[i])

print(f"Distance is: {tot}")

# Part 2
occurs = defaultdict(int)
firsts = []
for l in lines:
    f, s = [w for w in l.split(" ") if w != ""]
    firsts.append(int(f))
    occurs[int(s)] += 1

sim = 0
for n in firsts:
    sim += n * occurs[n]
    
print(f"Similarity: {sim}")

