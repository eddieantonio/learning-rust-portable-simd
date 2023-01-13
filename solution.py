from data import ARRAY

print(
    "problem_1(a) =",
    max(sum(1 for x in ARRAY if x > 0), sum(1 for x in ARRAY if x < 0)),
)
