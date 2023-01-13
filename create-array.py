#!/usr/bin/env python3

import sys
from random import randint

try:
    N = int(sys.argv[1])
except (IndexError, ValueError):
    N = 32

assert (N % 8) == 0

a = [randint(-2000, 2000) for _ in range(N)]
a.sort()

positive = sum(1 for x in a if x > 0)
negative = sum(1 for x in a if x < 0)
print("positive:", positive)
print("negative:", negative)
if positive > negative:
    print(f"({positive} positives)")
else:
    print(f"({negative} negatives)")

contents = f"""pub const ARRAY: [i16; {len(a)}] = {a!r};
"""

with open("src/data.rs", "w") as f:
    f.write(contents)
