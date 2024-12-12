with open("input-day02.txt") as f:
    lines = [l.strip() for l in f.readlines() if l.strip() != ""]

# Part 1
safe_count = 0
for report in lines:
    nums = [int(n) for n in report.split(" ")]

    # Case where len < 2?
    diff = nums[1] - nums[0]
    if abs(diff) > 3 or abs(diff) < 1:
        continue

    inc = (diff > 0)
    safe = True
    for i in range(2, len(nums)):
        diff = nums[i] - nums[i - 1]

        if (diff > 0) != inc:
            safe = False
            break

        if abs(diff) > 3 or abs(diff) < 1:
            safe = False
            break

    if safe:
        safe_count += 1

print(safe_count)

# Part 2
def safe(nums, bad):
    if len(nums) < 2:
        return True

    i = 0
    j = 1
    diff = nums[j] - nums[i]
    inc = (diff > 0)
    while j < len(nums):
        diff = nums[j] - nums[i]
        if abs(diff) > 3 or abs(diff) < 1 or (diff > 0) != inc:
            if bad:
                return False

            # Case where last num can be chopped off
            if j == len(nums) - 1:
                return True

            # Case where first num can be chopped off
            if i - 1 == 0:
                if safe(nums[i:], True):
                    return True

            return safe(nums[:j] + nums[j + 1:], True) or safe(nums[:i] + nums[j:], True)

        i += 1
        j += 1

    return True

safe_count = 0
for report in lines:
    nums = [int(n) for n in report.split(" ")]
    if safe(nums, False):
        safe_count += 1

print(safe_count)
