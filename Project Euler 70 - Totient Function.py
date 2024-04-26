import numpy as np
import math
import itertools as it

def is_permutation(a, b):
    if sorted(str(a)) == sorted(str(b)):
        return True
    return False

def is_prime(n):
  if n == 2 or n == 3: return True
  if n < 2 or n%2 == 0: return False
  if n < 9: return True
  if n%3 == 0: return False
  r = int(n**0.5)
  f = 5
  while f <= r:
    if n % f == 0: return False
    if n % (f+2) == 0: return False
    f += 6
  return True

def is_coprime(a, b):
    return math.gcd(a, b) == 1

def phi(n):
    result = 1
    for i in range(2, n):
        if is_coprime(i, n):
            result += 1
    return result

def sieve(n):
    flags = np.ones(n, dtype=bool)
    flags[0] = flags[1] = False
    for i in range(2, n):
        if flags[i]:
            flags[i*i::i] = False
    return np.flatnonzero(flags)

def multi_list(a):
    result = 1
    for x in a:
        result = result * x
    return result

def prime_phi(a):   # Gives phi for the product of an input list filled with only primes
    result = 1
    for x in a:
        result = result * (x - 1)
    return result

# Wenn GCD(a,b) = 1, dann gillt phi(ab) = Phi(a) * phi(b)
# Für eine Primzahl p gillt phi(p) = p-1
# Sieve laufen lassen als Dict mit PHI ergebnissen angehängt

def phi_sieve(n): # Erzeugt alle zahlen kleiner n und ihr dazugehöriges Phi
    primes = sieve(n)
    totient = [1] * (n+1)
    for i in primes:
        s = 1
        while s <= n:
            try:
                s += i
                totient[s] = totient[s] * (i-1)
            except:
                continue

    return totient

print(phi_sieve(100))
print(phi(7))