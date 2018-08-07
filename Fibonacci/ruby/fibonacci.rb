#!/usr/bin/ruby

def fib(x)
  x < 2 ? 1 : fib(x-1) + fib(x-2)
end

print "Calcolo n-esimo numero di Fibonacci: "
numero  = gets.chomp.to_i
puts "F(#{numero})=#{fib(numero)}"

