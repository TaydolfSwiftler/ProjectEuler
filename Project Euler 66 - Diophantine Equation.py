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

def convergents(x, y): # x = Input the square, y = Input Number of Convergents
    cont_frac = sympy.continued_fraction((sympy.sqrt(x)))

    h_conv = []
    h_0 = cont_frac[0]
    h_1 = (cont_frac[1][0] * cont_frac[0] + 1)
    h_conv.append(h_0)
    h_conv.append(h_1)
    for n in range(2, y):
        h_n = cont_frac[1][(n-1) % len(cont_frac[1])] * h_conv[len(h_conv) - 1] + h_conv[len(h_conv) - 2]
        h_conv.append(h_n)

    k_conv = []
    k_0 = 1
    k_1 = cont_frac[1][0]
    k_conv.append(k_0)
    k_conv.append(k_1)
    for i in range(2, y):
        k_i = cont_frac[1][(i-1) % len(cont_frac[1])] * k_conv[len(k_conv) - 1] + k_conv[len(k_conv) - 2]
        k_conv.append(k_i)
    return [h_conv, k_conv]


max_val = 0
max_D = 0

for D in range(2, 1000):
    if isPerfectSquare(D):
        continue
    D_Conv = convergents(D, 80)
    for i in range(0, len(D_Conv[0])):
        if D_Conv[0][i]**2 - D * D_Conv[1][i]**2 == 1:
            print("D = ", D, " x = ", D_Conv[0][i], " y = ", D_Conv[1][i])
            if D_Conv[0][i] > max_val:
                max_val = D_Conv[0][i]
                max_D = D
            break

print(max_val, " ", max_D)