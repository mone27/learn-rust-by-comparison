#!/usr/bin/env python3
def fibonacci(n):
    if n < 2:
        return 1
    return fibonacci(n-2) + fibonacci(n-1)

numero=int(input("Calcolo n-esimo numero di Fibonacci: "))
print("F(%d)=%d" % (numero, fibonacci(numero)))
