# https://adventofcode.com/2018/day/11

with open('day11.txt') as file:
    serial = int(file.read())

grid = [[((x + 10) * y + serial) * (x + 10) % 1000 // 100 - 5 for y in range(1,301)] for x in range(1,301)]

def region_power(x,y,s):
    return sum(grid[x+i-1][y+j-1] for j in range(s) for i in range(s))

def max_power(s):
    return max((region_power(x,y,s),x,y) for y in range(1,302-s) for x in range(1,302-s))

print(','.join(str(p) for p in max_power(3)[1:]))
print(','.join(str(p) for p in max(max_power(s) for s in range(1,301))))
