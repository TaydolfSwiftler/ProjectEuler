import itertools as it

a = [1, 2, 3, 4, 5]
b = [6, 7, 8, 9, 10]
c= [11, 12 , 13, 14]
d = [15, 16 ,17, 18, 19 , 21 ,22, 23, 24]
e = [a, b, c, d]

print(len(list(it.product(*e))))