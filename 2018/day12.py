# https://adventofcode.com/2018/day/12

with open('day12.txt') as file:
    state = file.readline().split()[2]
    file.readline()
    rules = {line.split()[0]: line.split()[2] for line in file}

def neighbors(s, i):
    return ''.join(s[j] if 0 <= j < len(s) else '.' for j in range(i-2,i+3))

def generate(state, generations):
    prev = [0,0,0,0,0]
    for gen in range(0, generations):
        state = ''.join(rules.get(neighbors(state, i), '.') for i in range(-1, len(state)+1))
        current = sum(i - gen - 1 if state[i] == '#' else 0 for i in range(len(state)))
        diff = current - prev[4]
        if all(prev[i] - prev[i-1] == diff for i in range(1, len(prev))):
            return current + diff * (generations - gen - 1)
        else:
            prev = prev[1:5] + [current]
    else:
        return current

print(generate(state, 20))
print(generate(state, 50000000000))