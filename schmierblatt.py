import itertools as it
import math

def multi_list(a):
    result = 1
    for x in a:
        result = result * x
    return result

for i in it.combinations([2,3,7,5], 3):
    print(multi_list(i))

mydicht = {}
mydicht[2] = 3
print(mydicht)

print(math.sqrt(9999999))