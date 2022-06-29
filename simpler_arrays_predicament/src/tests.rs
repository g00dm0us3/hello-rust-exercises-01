#[cfg(test)]
mod tests { 
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

    #[test]
    fn test_bin_search_edge_cases() {
        let short_arr:[i32; 0] = [];

        match bin_search(7, &short_arr, 0) {
            Some(_idx) => assert!(false),
            None => assert!(true)
        }

        let one_arr = [1];

        match bin_search(1, &one_arr, 1) {
            Some(idx) => assert!(idx == 0),
            None => assert!(false)
        }

        match bin_search(7, &one_arr, 1) {
            Some(_idx) => assert!(false),
            None => assert!(true)
        }

        let two_arr = [1, 2];
        match bin_search(2, &two_arr, 2) {
            Some(idx) => assert!(idx == 1),
            None => assert!(true)
        }

        match bin_search(1, &two_arr, 2) {
            Some(idx) => assert!(idx == 0),
            None => assert!(true)
        }

    }

    #[test]
    fn test_bin_search_odd_arr() {
        let odd_arr = [1,2,3,4,5];

        match bin_search(1, &odd_arr, 5) {
            Some(idx) => assert!(idx == 0),
            None => assert!(true)
        }

        match bin_search(5, &odd_arr, 5) {
            Some(idx) => assert!(idx == 4),
            None => assert!(true)
        }
    }

    #[test]
    fn test_bin_search_even_arr() {
        let even_arr = [1,2,3,4];

        match bin_search(4, &even_arr, 4) {
            Some(idx) => assert!(idx == 3),
            None => assert!(true)
        }

        match bin_search(3, &even_arr, 4) {
            Some(idx) => assert!(idx == 2),
            None => assert!(true)
        }

        match bin_search(1, &even_arr, 4) {
            Some(idx) => assert!(idx == 0),
            None => assert!(true)
        }
    }


}