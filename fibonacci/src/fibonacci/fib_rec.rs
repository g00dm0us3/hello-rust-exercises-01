pub fn fib_rec(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    if n <= 0 {
        return 0;
    }

    fib_rec(n-1).saturating_add(fib_rec(n-2))
}
