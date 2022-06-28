use std::io;
use std::cmp::Ordering;

const MAX_FIB_NUMBER_FOR_RECURSION: i32 = 20;
const MAX_FIB_NUMBER: i32 = 30;

fn fib_rec(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    if n <= 0 {
        return 0;
    }

    fib_rec(n-1) + fib_rec(n-2)
}

fn fib_iter(n: i32) -> i32 {
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

// deliberately use matches
fn main() {
   
    let mut number: i32 = 0;

    let mut should_recurse = false;
    loop {
        let mut buff = String::new();
        match io::stdin().read_line(&mut buff) {
            io::Result::Ok(_) => {
                // aka `guard let bla = maybeBla else { ... }`
                number = match buff.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Cannot parse number!");
                        continue;
                    }
                };

                if number > MAX_FIB_NUMBER {
                    println!("{} Fibbonacci number won't fit into 32 bits!", number);
                    continue;
                }

                if number == 0 {
                    println!("There is no {} Fibonacci number", number);
                    continue;
                }

                match number.cmp(&MAX_FIB_NUMBER_FOR_RECURSION) {
                    Ordering::Greater => should_recurse = false,
                    _ => break
                };


                break;
            }
            // doesn't make much sense, since any error on input is likely to be underlying OS error
            io::Result::Err(_) => { 
                println!("No input!!");
                continue; 
            }
        };
    };

    let mut fib_num: i32 = 0;
    match should_recurse {
        true => fib_num = fib_rec(number),
        false => fib_num = fib_iter(number)
    };

    println!("{}th Fibonacci number is {}", number, fib_num);
}
