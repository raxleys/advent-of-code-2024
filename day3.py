with open("input-day3.txt") as f:
    data = f.read()

# Part 1
stack = []
ptr = 0
n = None
m = None
tot = 0
while ptr < len(data):

    if len(stack) == 0:
        if data[ptr] == 'm':
            stack.append(data[ptr])

        ptr += 1
        continue

    if len(stack) < 4:
        if data[ptr] == 'm':
            # New m encountered, current expression invalid, may be start
            # of valid one
            stack = [data[ptr]]
        else:
            stack.append(data[ptr])

        ptr += 1
        continue

    # Stack is 4 chars long
    if "".join(stack) != "mul(":
        stack.clear()
        continue

    stack.clear()

    # Collect digits
    while ptr < len(data) and data[ptr].isdigit():
        stack.append(data[ptr])
        ptr += 1

    # Need to look ahead 2
    if ptr >= len(data) - 1:
        break

    if len(stack) == 0 or data[ptr] != ',' and not data[ptr + 1].isdigit():
        stack.clear()
        ptr += 1
        continue

    # First number OK
    n = int("".join(stack))
    stack.clear()

    # consume ','
    ptr += 1

    # Same basic thing for 2nd number
    while ptr < len(data) and data[ptr].isdigit():
        stack.append(data[ptr])
        ptr += 1

    # Need to look ahead 1
    if ptr >= len(data):
        break

    if len(stack) == 0 or data[ptr] != ')':
        stack.clear()
        ptr += 1
        continue

    m = int("".join(stack))
    tot += n * m

    # reset stack and consume ')'
    stack.clear()
    ptr += 1

print("tot =",tot)

# Part 2
def match_tok(data, ptr):
    toks = ["don't", "do", "mul"]
    start = ptr
    for tok in toks:
        ptr = start
        for c in tok:
            if c != data[ptr]:
                break
            ptr += 1
        if ptr - start == len(tok):
            return tok
    return None


def match_num(data, ptr):
    start = ptr
    while data[ptr].isdigit():
        ptr += 1

    if ptr - start == 0:
        return None, 0

    return int(data[start:ptr]), ptr - start


ptr = 0
tot = 0
do = True
while ptr < len(data):
    if data[ptr] != 'm' and data[ptr] != 'd':
        ptr += 1
        continue

    tok = match_tok(data, ptr)

    if tok is None:
        ptr += 1
        continue

    ptr += len(tok)
    if ptr >= len(data) or data[ptr] != '(':
        continue

    # consume (
    ptr += 1

    if tok == "mul":
        n, digits = match_num(data, ptr)
        if n is None:
            ptr += 1
            continue

        ptr += digits
        if ptr >= len(data) or data[ptr] != ',':
            continue

        # consume ,
        ptr += 1
        m, digits = match_num(data, ptr)
        if m is None:
            ptr += 1
            continue

        ptr += digits
        if ptr >= len(data) or data[ptr] != ')':
            continue

        # consume )
        ptr += 1

        if do:
            tot += n * m

    elif tok == "do":
        if ptr >= len(data) or data[ptr] != ')':
            continue
        do = True
        # consume )
        ptr += 1

    else:
        if ptr >= len(data) or data[ptr] != ')':
            continue
        do = False
        # consume )
        ptr += 1

print("tot =", tot)
