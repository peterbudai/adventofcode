def find_repeat(input):
    previous = {0}
    current = 0
    while True:
        for i in input:
            current += i
            if current in previous:
                return current
            else:
                previous.add(current)
                
with open('day1.txt') as f:
    input = [int(line) for line in f]

print(sum(input))
print(find_repeat(input))