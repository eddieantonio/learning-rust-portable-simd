#!/usr/bin/env python3

import sys
from random import randint

try:
    N = int(sys.argv[1])
except (IndexError, ValueError):
    N = 2000

a = [randint(-2000, 2000) for _ in range(N)]
a.sort()

# Just go ahead and solve the problem here:
positive = sum(1 for x in a if x > 0)
negative = sum(1 for x in a if x < 0)
answer = max(positive, negative)

explanation = f"answer = {answer} ({positive} positives, {negative} negatives)"
print(explanation)

# Write Rust source code:
with open("src/data.rs", "w") as f:
    f.write(
        f"""/// {explanation}
pub const ARRAY: [i16; {len(a)}] = {a!r};
"""
    )


# Write Python source code:
with open("data.py", "w") as f:
    f.write(
        f"""# {explanation}
ARRAY = {a!r}
"""
    )
