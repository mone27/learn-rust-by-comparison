#!/usr/bin/env python2
def fibonacci(n):
    a,b=1,1
    for i in range(n):
        a,b=b,a+b
    return a

numero=int(input("Calcolo n-esimo numero di Fibonacci: "))
print "F(%d)=%d" % (numero, fibonacci(numero))
