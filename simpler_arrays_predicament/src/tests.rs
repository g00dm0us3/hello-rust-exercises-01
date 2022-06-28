#[cfg(test)]
mod tests{ 
    use crate::primitive_arr::*;

    #[test]
    fn test_is_sorted() {
        let inc:[i32; 5] = [1, 2, 3, 4, 5];
        assert!(is_sorted(&inc, 5));

        let dec = [5, 4, 3, 2, 1];
        assert!(is_sorted(&dec, 5));

        let non_dec = [1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 5];
        assert!(is_sorted(&non_dec, 12));

        let non_inc = [5, 4, 4, 3, 3, 3, 2, 2, 2, 1];
        assert!(is_sorted(&non_inc, 10));
        
        let inc_dec = [1, 1, 2, 2, 3, 2, 1];
        assert!(!is_sorted(&inc_dec, 7));

        let inc_dec_mult = [9, 9, 9, 7, 7, 7, 9, 9, 9, 10, 10, 10, 3, 2, 1];
        assert!(!is_sorted(&inc_dec_mult, 15));
    }
}