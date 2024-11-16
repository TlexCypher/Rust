use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to get input.");
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse as a number."),
    };
    println!("The {}th fibonacci number is {}.", n, n_fib_number(n));
}

fn n_fib_number(n: i32) -> i32 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        n_fib_number(n - 1) + n_fib_number(n - 2)
    }
}
