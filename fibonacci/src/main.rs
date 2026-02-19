use std::io;

fn main() {
    println!("Enter the Fibonacci number index:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");
    let n: i64 = n.trim().parse().expect("Failed to parse the integer");
    let number = fib(n);
    println!("The result is: {number}");
}

fn fib(n: i64) -> i64 {
    if n == 1  || n == 2 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
