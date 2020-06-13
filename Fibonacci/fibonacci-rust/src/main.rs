use std::io;
use std::io::Write;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        __ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut n = String::new();

    print!("Calculating nth Fibonacci number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut n).expect("error in writing in stdin");

    let n: u32 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => panic!("error in number conversion"),
    };

    println!("\nF({})={}", n, fibonacci(n));
}

