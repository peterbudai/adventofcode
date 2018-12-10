# https://adventofcode.com/2018/day/7

import re

def read_graph():
    instruction = re.compile(r'^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$')
    with open('day7.txt') as file:
        edges = [instruction.match(line).groups() for line in file]
    return (sorted({p for e in edges for p in e}), edges)

def linear(points, edges):
    for p in points:
        if all(p != p2 or p1 not in points for p1,p2 in edges):
            return [p] + linear([q for q in points if q != p], edges)
    else:
        return []

def parallel(points, edges):
    workers = [[None, 0] for _ in range(5)]
    now = 0
    while points:
        free_worker = next((w for w in workers if w[0] is None), None)
        if free_worker:
            available_job = next((job for job in [p for p in points if p not in {w[0] for w in workers if w[0] is not None}] if all(job != p2 or p1 not in points for p1,p2 in edges)), None)
            if available_job:
                free_worker[0] = available_job
                free_worker[1] = now + 61 + ord(available_job) - ord('A')
                continue

        done_worker = min((w for w in workers if w[0] is not None), key=lambda w: w[1])
        now = done_worker[1]
        points.remove(done_worker[0])
        done_worker[0] = None
    return max(w[1] for w in workers)

graph = read_graph()
print(''.join(linear(*graph)))
print(parallel(*graph))