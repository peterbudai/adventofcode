import re

area = [0 for i in range(1000*1000)]
regex = re.compile(r'^#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)$')
with open('input.txt') as f:
    for left,top,width,height in (regex.match(line).groups()[1:] for line in f):
        for h in range(int(height)):
            for w in range(int(width)):
                area[(int(top)+h)*1000+int(left)+w] += 1

print(len([a for a in area if a > 1]))
