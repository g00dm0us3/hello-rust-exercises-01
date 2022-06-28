use std::io;
use std::cmp::Ordering;

// shortcut
use crate::fibonacci as fib;

// to make compiler look for code.
mod fibonacci;

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

                if number == 0 {
                    println!("There is no {} Fibonacci number", number);
                    continue;
                }

                match number.cmp(&fib::MAX_FIB_NUMBER_FOR_RECURSION) {
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
        true => fib_num = fib::fib_rec::fib_rec(number),
        false => fib_num = fib::fib_iter::fib_iter(number)
    };

    println!("{}th Fibonacci number is {}", number, fib_num);
}
