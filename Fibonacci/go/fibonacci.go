package main

import "fmt"

func fib(n uint) uint {
	if n == 0 || n == 1 {
		return 1
	}
	return fib(n-2) + fib(n-1)
}

func main() {
	var n uint
	fmt.Print("Calcolo n-esimo numero di Fibonacci: ")
	if _, err := fmt.Scan(&n); err != nil {
		fmt.Print("errore di lettura ", err)
		return
	}
	fmt.Printf("F(%d)=%d\n", n, fib(n))
}
