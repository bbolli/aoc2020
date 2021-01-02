#! /usr/bin/env python3

import re

input = [l for l in open('02.in').read().split('\n') if l]

valid1 = valid2 = 0
for i in input:
    range, letter, pwd = i.split()
    lo, hi = map(int, range.split('-'))
    letter = letter[0]
    if lo <= pwd.count(letter) <= hi:
        valid1 += 1
    if (pwd[lo-1] + pwd[hi-1]).count(letter) == 1:
        valid2 += 1
print(valid1, valid2)


