import math
def pythagorean_triplet():
    for a in range(1, 1000):
        for b in range(1, 1000):
            c = math.sqrt(pow(a, 2) + pow(b, 2))
            if (a + b + c) == 1000:
                return (a, b, c)
    return (0, 0, 0)

(a, b, c) = pythagorean_triplet()
print(a, b, c)
print(a * b * c)
