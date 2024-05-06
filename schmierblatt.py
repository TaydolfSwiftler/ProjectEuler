from time import process_time
start = process_time()
def fac (n):
    fact = 1
    while (n > 0):
        fact = fact * n
        n = n - 1
    return fact


print(fac(50000))

end = process_time()
print(end - start)

# 0.578125s