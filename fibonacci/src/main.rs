use std::io;

fn fib(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    if n <= 0 {
        return 0;
    }

    fib(n-1) + fib(n-2)
}

fn main() {
    let f = fib(20);
    println!("{f}");

    // - todo: user input, limits, O(n)
}
