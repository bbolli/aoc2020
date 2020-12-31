#! /usr/bin/env python3

from itertools import product

report = [int(x) for x in open('01.in').read().split('\n') if x]

for a, b in product(report, report):
    if a + b == 2020:
        print(a * b)
        break

for a, b, c in product(report, report, report):
    if a + b + c == 2020:
        print(a * b * c)
        break
