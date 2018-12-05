# https://adventofcode.com/2018/day/2

def find_pair(lines):
    for i in range(len(lines)-1):
        for j in range(1, len(lines)):
            d = 0
            for n, c in enumerate(lines[i]):
                if c != lines[j][n]:
                    d += 1
                if d > 1:
                    break
            else:
                if d == 1:
                    return (lines[i], lines[j])


with open('day2.txt') as f:
    lines = [l.strip() for l in f]

stats = [{line.count(char) for char in set(line)} for line in lines]
print(len(list(filter(lambda s: 2 in s, stats))) * len(list(filter(lambda s: 3 in s, stats))))

a, b = find_pair(lines)
i = next(n for n, c in enumerate(a) if a[n] != b[n])
print(a[:i]+b[i+1:])