# https://adventofcode.com/2018/day/9

import re
import sys
from collections import deque

with open('day9.txt') as file:
    players, marbles = (int(i) for i in re.match(r'^(\d+) players; last marble is worth (\d+) points$', file.readlines()[0]).groups())

def high_score(players, marbles):
    circle = deque([0], marbles)
    length = 1
    players = [0 for _ in range(players)]
    current = 0
    for marble in range(1, marbles+1):
        player = marble % len(players)
        if marble % 23 == 0:
            current = current - 7 if current >= 7 else length - (7 - current)
            players[player] += marble + circle[current]
            del circle[current]
            length -= 1
        else:
            current = current + 2 if current <= length - 2 else 1
            circle.insert(current, marble)
            length += 1
        # print("[{}] {}".format(player, ''.join("({})".format(c) if i == current else " {} ".format(c) for i,c in enumerate(circle))))
        if marble % 1000 == 0:
            print(marble//1000, file=sys.stdout)
    return max(players)

print(high_score(players, marbles))
#print(high_score(players, marbles * 100))