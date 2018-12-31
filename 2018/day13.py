# https://adventofcode.com/2018/day/13

with open('day13.txt') as file:
    tracks = [[char for char in line.strip('\r\n')] for line in file]

trains = [[r,c,t,0] for r,l in enumerate(tracks) for c,t in enumerate(l) if t in set('<>^v')]
for r,c,d,t in trains:
    tracks[r][c] = '-' if d in set("<>") else '|'

control = {
    'v': ({
        '+': (['>','v','<'], 1),
        '\\': (['>','>','>'], 0),
        '/': (['<','<','<'], 0),
        '|': (['v','v','v'], 0),
    }, (1,0)),
    '^': ({
        '+': (['<','^','>'], 1),
        '\\': (['<','<','<'], 0),
        '/': (['>','>','>'], 0),
        '|': (['^','^','^'], 0),
    }, (-1,0)),
    '>': ({
        '+': (['^','>','v'], 1),
        '\\': (['v','v','v'], 0),
        '/': (['^','^','^'], 0),
        '-': (['>','>','>'], 0),
    }, (0,1)),
    '<': ({
        '+': (['v','<','^'], 1),
        '\\': (['^','^','^'], 0),
        '/': (['v','v','v'], 0),
        '-': (['<','<','<'], 0),
    }, (0,-1)),
}

crash = False
while len(trains) > 1:
    trains.sort()
    i = 0
    while i < len(trains):
        train = trains[i]
        d, m = control[train[2]]
        train[0] += m[0]
        train[1] += m[1]
        d, t = d[tracks[train[0]][train[1]]]
        train[2] = d[train[3] % 3]
        train[3] += t
        other = next((i2 for i2,t2 in enumerate(trains) if i2 != i and t2[0:2] == train[0:2]), None)
        if other is None:
            i += 1
        else:
            if not crash:
                print('{},{}'.format(train[1], train[0]))
            crash = True
            if other < i:
                del trains[i]
                del trains[other]
                i -= 1
            else:
                del trains[other]
                del trains[i]
print('{},{}'.format(trains[0][1], trains[0][0]))