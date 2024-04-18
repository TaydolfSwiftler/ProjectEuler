import sympy
import math


def isPerfectSquare(x):
    # if x >= 0,
    if (x >= 0):
        sr = int(math.sqrt(x))
        # sqrt function returns floating value so we have to convert it into integer
        # return boolean T/F
        return ((sr * sr) == x)
    return False


counter = 0
for i in range(2,100):
    if i % 500 == 0:
        print(i)
    if isPerfectSquare(i):
        continue
    if len(sympy.continued_fraction(sympy.sqrt(i))[1]) % 2 == 1:
        print(i, " ", math.sqrt(i))
        counter += 1

#This Solution is dumb and shouldnt count