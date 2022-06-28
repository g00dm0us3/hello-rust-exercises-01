// borrowing - passing w/o takinbg ownership
pub fn find_min(arr: &[i32]) -> i32 {
    let mut min = i32::max_value();

    for el in arr {
        if *el < min {
            min = *el;
        }
    }

    min
}