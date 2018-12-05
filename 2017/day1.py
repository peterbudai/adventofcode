# https://adventofcode.com/2017/day/1

with open('day1.txt') as f:
    input = f.read().strip()

print(sum(ord(c) - ord('0') for i,c in enumerate(input) if c == input[(i+1) % len(input)]))
print(sum(ord(c) - ord('0') for i,c in enumerate(input) if c == input[(i + len(input) // 2) % len(input)]))