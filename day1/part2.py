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

with open('input.txt') as f:
    print(find_repeat([int(line) for line in f]))
