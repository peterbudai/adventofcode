# https://adventofcode.com/2018/day/14

with open('day14.txt') as file:
    input = int(file.read())

def reset():
    global recipes, current
    recipes = [3, 7]
    current = [0, 1]

def new_recipe():
    global recipes, current
    recipes += [int(c) for c in str(sum(recipes[i] for i in current))]
    current = [(i + recipes[i] + 1) % len(recipes) for i in current]

reset()
while len(recipes) < 10+input:
    new_recipe()
print(''.join(str(n) for n in recipes[input:input+10]))

reset()
target = [int(c) for c in str(input)]
check_from = 0
while True:
    if len(recipes) >= check_from + len(target):
        if all(recipes[check_from+i] == t for i,t in enumerate(target)):
            break
        check_from += 1
        continue
    new_recipe()
print(check_from)