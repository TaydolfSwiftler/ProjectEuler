import numpy as np
import itertools as it

def polygonal_number_generator(x, n): # x = Numbertype i.e. 3 = triagonal, n = nth Number
    s1 = ((x-2) * n**2 - (x-4) * n) / 2
    return int(s1)

def iscyclical(n, m):
    if str(n)[-2:] == str(m)[:2]:
        return True
    return False

Polygon_numbers =[[],[],[],[],[],[]] # 0th element = trig Numbers

for i in range (3, 9):
    for j in range(1,500):
        poly = polygonal_number_generator(i,j)
        if poly >= 10000:
            break
        if poly < 1000:
            continue
        Polygon_numbers[i-3].append(poly)


two_cyclical = []

for i in Polygon_numbers[0]:
    for j in Polygon_numbers[1]:
        if iscyclical(i, j):
            two_cyclical.append([i, j])
        if iscyclical(j,i):
            two_cyclical.append([j, i])

print(two_cyclical)
print(len(two_cyclical))