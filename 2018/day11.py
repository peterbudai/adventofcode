# https://adventofcode.com/2018/day/11

with open('day11.txt') as file:
    serial = int(file.read())

power = [[[((x + 10) * y + serial) * (x + 10) % 1000 // 100 - 5 for y in range(1,301)] for x in range(1,301)]]
for s in range(2,301):
    h = s // 2
    if s % 2 == 0:
        power.append([[power[h-1][x][y] + power[h-1][x+h][y] + power[h-1][x][y+h] + power[h-1][x+h][y+h] for y in range(300-s+1)] for x in range(300-s+1)])
    else:
        power.append([[power[h-1][x][y] + power[h-1][x+h+1][y] + power[h-1][x][y+h+1] + power[h-1][x+h+1][y+h+1] + sum(power[0][x+h][i] for i in range(y,y+s)) + sum(power[0][i][y+h] for i in range(x,x+s)) - power[0][x+h][y+h] for y in range(300-s+1)] for x in range(300-s+1)])

def print_coord(c):
    print(','.join(map(lambda p: str(p+1), c)))

print_coord(max(((x,y) for x in range(298) for y in range(298)), key=lambda p: power[2][p[0]][p[1]]))
print_coord(max(((x,y,s) for s in range(301) for x in range(300-s) for y in range(300-s)), key=lambda p: power[p[2]][p[0]][p[1]]))