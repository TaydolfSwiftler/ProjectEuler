import math

def convergents_of_e(y): # x = Input Number of Convergents
    cont_frac = [[2],[1]]
    for i in range (3,y):
        if i % 3 == 0:
            cont_frac[1].append(int((i / 3) * 2))
        else:
            cont_frac[1].append(1)

    h_conv = []
    h_0 = cont_frac[0][0]
    h_1 = (cont_frac[1][0] * cont_frac[0][0] + 1)
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

def sum_digits(n):
    s = 0
    while n:
        s += n % 10
        n //= 10
    return s

e_conv = convergents_of_e(103)
print(sum_digits(e_conv[0][99]))
