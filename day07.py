import itertools

# Part 1
data = []
with open("input-day07.txt") as f:
    for line in (l.strip() for l in f.readlines() if l.strip() != ""):
        val, eq = line.split(":")
        data.append((int(val), [int(n) for n in eq.split(" ") if n != ""]))

# We can consider all the 'equations' as all possible configurations
# of a list of length N - 1, where each value can be either '+' or '*'.
# This is basically the cartesian product of N - 1 sets of {'+', '*'}.
cal_res = 0
for val, nums in data:
    for opstr in itertools.product(['+', '*'], repeat=len(nums) - 1):
        cval = nums[0]
        for i, op in enumerate(opstr):
            if op == '+':
                cval += nums[i + 1]
            else:
                cval *= nums[i + 1]

        if cval == val:
            cal_res += val
            break

print("cal_res =", cal_res)

# Part 2
# Adding the 3rd operator makes this take much, much longer. 20s on my PC.
cal_res = 0
for val, nums in data:
    for opstr in itertools.product(['+', '*', '||'], repeat=len(nums) - 1):
        cval = nums[0]
        for i, op in enumerate(opstr):
            if op == '+':
                cval += nums[i + 1]
            elif op == '*':
                cval *= nums[i + 1]
            else:
                cval = int(str(cval) + str(nums[i + 1]))

        if cval == val:
            cal_res += val
            break

print("cal_res =", cal_res)
