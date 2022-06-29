#[cfg(test)]
mod sort_tests { 
    use crate::primitive_arr::sorts::insert_sort::*;
    #[test]
    fn test_insert_sort() {
        let mut arr:[i32; 6] = [2, 1, 89, 1, -7, 225];

        insert_sort(&mut arr, 6);

        println!("{}", arr.map(|el| el.to_string()).join(", "));

        let equal = arr.iter().zip([-7, 1, 1, 2, 89, 225].iter()).all(|(a, b)| a == b);
        assert!(equal);
    }
}