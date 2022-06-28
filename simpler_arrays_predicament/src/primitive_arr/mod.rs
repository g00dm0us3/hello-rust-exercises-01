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

pub fn is_sorted(arr: &[i32], sz: usize) -> bool {
    if sz <= 2 {
        return true;
    }

    let equals_sign = 0;

    let mut second_sign = equals_sign;

    for i in 0..sz-1 {
        let sign: i32 = arr[i+1] - arr[i];

        if equals_sign == second_sign && sign != second_sign {
            second_sign = sign;
        }

        if sign != equals_sign && sign != second_sign {
            return false;
        }
    }

    return true;
}

pub fn bin_search(arr: &[i32], sz: usize) -> Option<i32> {
    if !is_sorted(arr, sz) {
        return None;
    }

    return Some(0);
}