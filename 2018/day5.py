# https://adventofcode.com/2018/day/5

with open('day5.txt') as f:
    input = f.read().strip()

def react(input):
    i = 0
    poly = [c for c in input]
    while i < len(poly) - 1:
        a = poly[i]
        b = poly[i+1]
        if a.upper() == b.upper() and ((a.isupper() and b.islower()) or (a.islower() and b.isupper())):
            del poly[i:i+2]
            i = max(0, i-2)
        else:
            i += 1
    return len(poly)

print(react(input))
print(min(react(input.replace(chr(c),'').replace(chr(c).upper(),'')) for c in range(ord('a'), ord('z')+1)))
