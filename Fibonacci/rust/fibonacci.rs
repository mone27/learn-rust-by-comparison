use std::io;
use std::io::Write;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut n = String::new();

    print!("Calcolo n-esimo numero di Fibonacci: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut n).expect("errore nella lettura da stdin");

    let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("errore di conversione"),
    };

    println!("\nF({})={}", n, fibonacci(n));
}

