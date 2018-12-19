# https://adventofcode.com/2018/day/10

import re
from time import sleep

point = re.compile(r'^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$')
with open('day10.txt') as file:
    points = [tuple(map(int, point.match(line).groups())) for line in file]

def bounding_box(t, plot=False):
    current = [(p[0] + t * p[2], p[1] + t * p[3]) for p in points]
    minx = min(p[0] for p in current)
    maxx = max(p[0] for p in current)
    miny = min(p[1] for p in current)
    maxy = max(p[1] for p in current)
    height = maxy-miny+1
    width = maxx-minx+1
    if plot:
        for y in range(height):
            for x in range(width):
                if any(p[0]-minx == x and p[1]-miny == y for p in current):
                    print('#', end='')
                else:
                    print('.', end='')
            print()
    return height*width

candidate = min(range(20000), key=bounding_box)
bounding_box(candidate, True)
print(candidate)