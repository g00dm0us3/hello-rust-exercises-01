pub fn fib_iter(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n <= 2 {
        return 1;
    }

    let mut f1: i32 = 1;
    let mut f2: i32= 1;

    for _i in 0..n-2 {
        let t = f2;
        f2 = f2.saturating_add(f1);
        f1 = t;
    }

    return f2;
}