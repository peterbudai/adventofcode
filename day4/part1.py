import re
import operator
from datetime import date, time, timedelta

SLEEP = -1
AWAKE = -2

def read_input(fn):
    line_re = re.compile(r'^\[(\d+)-(\d+)-(\d+)\s+(\d+):(\d+)\]\s+(.+)$')
    action_re = re.compile(r'Guard #(\d+) begins shift')
    with open(fn) as file:
        for year, month, day, hour, minute, action in (line_re.match(line).groups() for line in file):
            d = date(int(year), int(month), int(day))
            t = time(int(hour), int(minute))
            if action == 'falls asleep':
                action = SLEEP
            elif action == 'wakes up':
                action = AWAKE
            else:
                action = int(action_re.match(action).group(1))
                if t > time(12, 0):
                    d += timedelta(days=1)
                    t = time(0)
            yield (d, t, action)

# determine boundary values
# print(min(read_input(), key=lambda i: i[0]))
# print(max(read_input(), key=lambda i: i[0]))
# print(len(set(map(lambda i: i[1], read_input()))))
# print(min(filter(lambda i: i[1] > time(12,0), read_input()), key=lambda i: i[1]))
# print(max(filter(lambda i: i[1] < time(12,0), read_input()), key=lambda i: i[1]))

def sleep_calendar(input):
    current = None
    for date, time, action in sorted(input):
        if action > 0:
            if current is not None:
                yield current
            current = (date, action, [0 for _ in range(60)])
        elif action == SLEEP:
            sleep = time
        elif action == AWAKE:
            current[2][sleep.minute:time.minute] = [1 for _ in range(sleep.minute, time.minute)]
    if current is not None:
        yield current

cal = list(sleep_calendar(read_input('input.txt')))
guards = {c[1]: [0 for _ in range(60)] for c in cal}
for c in cal:
    for i in range(60):
        guards[c[1]][i] += c[2][i]

winner_guard = max(guards, key=lambda g: sum(guards[g]))
winner_minute = max(enumerate(guards[winner_guard]), key=lambda m: m[1])[0]
print(winner_guard * winner_minute)