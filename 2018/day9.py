# https://adventofcode.com/2018/day/9

import re
import sys
from collections import deque

with open('day9.txt') as file:
    players, marbles = (int(i) for i in re.match(r'^(\d+) players; last marble is worth (\d+) points$', file.readlines()[0]).groups())

def high_score(players, marbles):
    circle = deque([0], marbles)
    players = [0 for _ in range(players)]
    for marble in range(1, marbles+1):
        if marble % 23 == 0:
            circle.rotate(7)
            players[marble % len(players)] += marble + circle.popleft()
        else:
            circle.rotate(-2)
            circle.appendleft(marble)
    return max(players)

print(high_score(players, marbles))
print(high_score(players, marbles * 100))