
// wtf is this? 
// preprocessor macro, checks for build config?
#[cfg(test)]
mod tests {
    use crate::fibonacci::*;

    #[test]
    fn test_recursive() {
        assert_eq!(0, fib_rec::fib_rec(0));
        assert_eq!(1, fib_rec::fib_rec(1));
        assert_eq!(1, fib_rec::fib_rec(2));
        assert_eq!(21, fib_rec::fib_rec(8));
    }

    #[test]
    fn test_iter() {
        assert_eq!(0, fib_iter::fib_iter(0));
        assert_eq!(1, fib_iter::fib_iter(1));
        assert_eq!(1, fib_iter::fib_iter(2));
        assert_eq!(21, fib_iter::fib_iter(8));
    }
}