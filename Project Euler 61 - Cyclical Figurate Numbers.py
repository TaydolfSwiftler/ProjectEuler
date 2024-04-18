import numpy as np
import itertools as it

def polygonal_number_generator(x, n): # x = Numbertype i.e. 3 = triagonal, n = nth Number
    s1 = ((x-2) * n**2 - (x-4) * n) / 2
    return int(s1)

def iscyclical(n, m):
    if str(n)[-2:] == str(m)[:2]:
        return True
    return False

octa = []
for j in range(1, 100):
    poly = polygonal_number_generator(8, j)
    if poly >= 10000:
        break
    if poly < 1000:
        continue
    octa.append(poly)

Polygon_numbers =[[],[],[],[],[],[]] # 0th element = trig Numbers

for i in range (3, 9):
    for j in range(1,500):
        poly = polygonal_number_generator(i,j)
        if poly >= 10000:
            break
        if poly < 1000:
            continue
        Polygon_numbers[i-3].append(poly)


# all_shapes = []
# for i in range(3, 9):
#     for j in range(1, 500):
#         poly = polygonal_number_generator(i, j)
#         if poly >= 10000:
#             break
#         if poly < 1000:
#             continue
#
#         shape = ""
#         match i:
#             case 3:
#                 shape = "triangle"
#             case 4:
#                 shape = "square"
#             case 5:
#                 shape = "pent"
#             case 6:
#                 shape = "hex"
#             case 7:
#                 shape = "hep"
#             case 8:
#                 shape = "oct"
#             case _:
#                 print("Critical Error")
#         all_shapes.append([poly, shape])

pos_permutations = list(it.permutations([x for x in range(0,6)]))
print(pos_permutations)

for x in pos_permutations:
    for i in x:
        for j in Polygon_numbers[i]:
