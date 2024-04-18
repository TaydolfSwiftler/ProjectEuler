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
      flags[i * i::i] = False
  return np.flatnonzero(flags)
