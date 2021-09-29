from itertools import permutations

def func(a, b, c, d, e):
    return a + b * c**2 + d**3 - e

for vals in permutations([2, 7, 3, 9, 5], 5):
    if func(*vals) == 399:
        print(vals)
