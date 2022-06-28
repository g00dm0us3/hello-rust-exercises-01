pub const MAX_FIB_NUMBER_FOR_RECURSION: i32 = 20;
pub const MAX_FIB_NUMBER: i32 = 30;

pub fn fib_rec(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    if n <= 0 {
        return 0;
    }

    fib_rec(n-1) + fib_rec(n-2)
}

pub fn fib_iter(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    }

    let mut f1 = 1;
    let mut f2 = 1;

    for _i in 0..n-2 {
        let t = f2;
        f2 = f1 + f2;
        f1 = t;
    }

    return f2;
}