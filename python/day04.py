# Part 1
def find_horizontal_fwd(line):
    count = 0
    i = 0
    while i < len(line):
        if line[i] != 'X':
            i += 1
            continue

        if i + len("MAS") >= len(line):
            break

        if not "".join(line[i:i + len("MAS") + 1]) == "XMAS":
            i += 1
            continue

        count += 1
        i += len("MAS") + 1

    return count


def find_all_horizontal(data):
    count = 0
    for l in data:
        count += find_horizontal_fwd(l)
        count += find_horizontal_fwd(l[::-1])
    return count


def find_all_vertical(data):
    # build flipped arrays
    arrs = [[] for l in range(len(data[0]))]
    for i in range(len(data[0])):
        for l in data:
            arrs[i].append(l[i])

    return find_all_horizontal(arrs)


def find_diag_fwd(data):
    count = 0
    for j in range(len(data)):
        l = data[j]
        for i in range(len(l)):
            if l[i] != 'X':
                continue

            if i + len("MAS") >= len(l) or j + len("MAS") >= len(data):
                break

            if data[j + 1][i + 1] != 'M' or data[j + 2][i + 2] != 'A' or data[j + 3][i + 3] != 'S':
                continue

            count += 1

    return count


def find_diag_fwd_up(data):
    return find_diag_fwd(data[::-1])


def find_diag_bwd_up(data):
    rev = []
    for l in data:
        rev.append(l[::-1])

    return find_diag_fwd(rev)


def find_diag_bwd(data):
    return find_diag_bwd_up(data[::-1])


def find_all_diag(data):
    return find_diag_fwd(data) + find_diag_fwd_up(data) + find_diag_bwd_up(data) + find_diag_bwd(data)


with open("input-day04.txt") as f:
    data = []
    for l in f.readlines():
        data.append(list(l.strip()))

occurs = find_all_horizontal(data)
occurs += find_all_vertical(data)
occurs += find_all_diag(data)
print("total:", occurs)

# Part 2
lets = {'M', 'S'}
count = 0
for j in range(len(data) - 2):
    for i in range(len(data[j]) - 2):
        let = data[j][i]
        if let not in lets:
            continue

        adj = data[j][i + 2]
        if adj not in lets or data[j + 1][i + 1] != 'A':
            continue

        let_o = list(lets - {let}).pop()
        adj_o = list(lets - {adj}).pop()

        let_d = data[j + 2][i]
        adj_d = data[j + 2][i + 2]
        if let_d not in lets or adj_d not in lets:
            continue

        if let == adj:
            if let == let_d or adj == adj_d:
                continue
        else:
            if let != let_d or adj != adj_d:
                continue

        count += 1

print("total:", count)
