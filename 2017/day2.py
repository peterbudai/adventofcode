# https://adventofcode.com/2017/day/2

from itertools import product
from operator import mod

with open('day2.txt') as file:
    input = [map(int, line.strip().split()) for line in file]

print(sum(max(row) - min(row) for row in input))
print(sum(next(p[0] // p[1] for p in product(row, row) if p[0] != p[1] and p[0] % p[1] == 0) for row in input))