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


with open('input.txt') as f:
    a, b = find_pair([l.strip() for l in f])

i = next(n for n, c in enumerate(a) if a[n] != b[n])
print(a[:i]+b[i+1:])