with open('input.txt') as f:
    stats = [{line.count(char) for char in set(line)} for line in f]
print(len(list(filter(lambda s: 2 in s, stats))) * len(list(filter(lambda s: 3 in s, stats))))
