import timeit
start = timeit.timeit()

f_n_minus1 = 1
f_n = 2
sum = 2
dummy = 0

while f_n < 4000000:
    dummy = f_n
    f_n = f_n + f_n_minus1
    f_n_minus1 = dummy
    if f_n % 2 == 0:
        sum += f_n

print("The Solution is: ", sum)
end = timeit.timeit()
print("Time elapsed: ", end - start)