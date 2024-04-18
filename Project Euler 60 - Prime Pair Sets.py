import numpy as np
from sympy import isprime
import itertools
import random


def sieve(n):
    flags = np.ones(n, dtype=bool)
    flags[0] = flags[1] = False
    for i in range(2, n):
        if flags[i]:
            flags[i*i::i] = False
    return np.flatnonzero(flags)

def concatenate(x, y):
    return int(str(x) + str(y))

def con_set_maker(x):
    result = []
    prime_pairs = itertools.permutations(x, 2)
    for i in prime_pairs:
        result.append(concatenate(i[0], i[1]))
    return result

def list_prime_check(x):
    for i in x:
        if not isprime(i):
            return False
    return True


# Erkenntnisse
# 2 und 5 sind Primzahlen, aber können nicht Teil der Lösung sein
# Gesucht wir die niedrigste Summe
# Jede Teilgruppe muss uach die Bedingung erfüllen!
# Teilgruppen erstellen und erweitern
# Der nte Teilnehmer ist zwangsläufig schon mit n-2 Teilnehmern in einer Gruppe, bei gleich großer Primzahlbase

primes = list(sieve(10000))
primes.remove(2)
primes.remove(5)

#Erzeugen aller Zweierpaare
conc_prime_pairs = []
for i in itertools.combinations(primes, 2):
    j = con_set_maker(i)
    if list_prime_check(j):
        conc_prime_pairs.append(list(i))

#Erzeugen aller Dreipaare ohne Dupliakte
conc_prime_triples = []
for i in conc_prime_pairs:
    first_prime = i[0]
    second_prime = i[1]
    for j in conc_prime_pairs:
        if j[0] != first_prime or j[1] <= second_prime:
            continue
        pot_trip = i.copy()
        pot_trip.append(j[1])
        pot_trip_con = con_set_maker(pot_trip)
        if list_prime_check(pot_trip_con):
            conc_prime_triples.append(pot_trip)


#Erzeugen aller Viererpaare ohne Dupliakte
conc_prime_quadruples = []
for i in conc_prime_triples:
    for j in conc_prime_triples:
        if j[0] != i[0] or i[1] != j[1] or j[2] <= i[2]:
            continue
        pot_quad = i.copy()
        pot_quad.append(j[2])
        pot_quad_con = con_set_maker(pot_quad)
        if list_prime_check(pot_quad_con):
            conc_prime_quadruples.append(pot_quad)

#Erzeugen aller Fünferpaare ohne Dupliakte
conc_prime_quintets = []
for i in conc_prime_quadruples:
    for j in conc_prime_quadruples:
        if j[0] != i[0] or j[1] != i[1] or j[2] != i[2] or j[3] <= i[3]:
            continue
        pot_quin = i.copy()
        pot_quin.append(j[3])
        pot_quin_con = con_set_maker(pot_quin)
        if list_prime_check(pot_quin_con):
            conc_prime_quintets.append(pot_quin)

print(conc_prime_pairs)
print(conc_prime_triples)
print(conc_prime_quadruples)
print(conc_prime_quintets)

print("the solution is: ", sum(conc_prime_quintets[0]))