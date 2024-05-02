from time import process_time

start = process_time()
def sum_squares(x):
    sum_before = 0
    sum_after = 0
    for i in range(1,x+1):
        sum_before += i**2
        sum_after += i
    sum_after *= sum_after
    return sum_after - sum_before

print("The Solution is: ", sum_squares(25000))

end = process_time()
print(end - start)