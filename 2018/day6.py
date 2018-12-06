# https://adventofcode.com/2018/day/6

def dist(a,b):
    return abs(b[0]-a[0]) + abs(b[1]-a[1])

def find_single_closest(x,y, points):
    closest_points = []
    closest_dist = -1
    for p in range(len(points)):
        d = dist(points[p], (x,y))
        if d < closest_dist or closest_dist == -1:
            closest_dist = d
            closest_points = []
        if d == closest_dist:
            closest_points.append(p)
    if len(closest_points) == 1:
        return closest_points[0]
    return None

with open('day6.txt') as file:
    input = sorted(tuple(map(int, line.split(','))) for line in file)

left = min(x for x,y in input)
right = max(x for x,y in input)
top = min(y for x,y in input)
bottom = max(y for x,y in input)

# Part 1
owned = [0 for p in range(len(input))]
for x in range(left, right+1):
    for y in range(top, bottom+1):
        closest = find_single_closest(x, y, input)
        if closest is not None:
            owned[closest] += 1

for x in [left-1, right+1]:
    for y in range(top, bottom+1):
        closest = find_single_closest(x, y, input)
        if closest is not None:
            owned[closest] = -1

for x in range(left, right+1):
    for y in [top-1, bottom+1]:
        closest = find_single_closest(x, y, input)
        if closest is not None:
            owned[closest] = -1
print(max(owned))

# Part 2
count = 0
for x in range(left, right+1):
    for y in range(top, bottom+1):
        sum_dist = sum(dist(input[p], (x,y)) for p in range(len(input)))
        if sum_dist < 10000:
            count += 1
print(count)
