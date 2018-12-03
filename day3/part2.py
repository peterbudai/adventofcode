import re

area = [set() for i in range(1000*1000)]
claims = {}
regex = re.compile(r'^#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)$')
with open('input.txt') as f:
    for id, left,top,width,height in (regex.match(line).groups() for line in f):
        for h in range(int(height)):
            for w in range(int(width)):
                a = area[(int(top)+h)*1000+int(left)+w]
                a.add(id)
                for aa in a:
                    claims.setdefault(aa, set()).update(a)

print(next(id for id in claims if len(claims[id]) == 1))