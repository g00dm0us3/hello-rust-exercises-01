mod sorts;
// borrowing - passing w/o taking ownership
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

pub fn bin_search(needle: i32, arr: &[i32], sz: usize) -> Option<usize>{
    if !is_sorted(arr, sz) {
        return None;
    }

    if sz == 0 {
        return None;
    }

    let prepare_arr = || -> (&[i32], usize) {
        if sz % 2 == 0 {
            (&arr[0..sz-1], sz-1)
        } else {
            (&arr, sz)
        }
    };

    let (arr__, size) = prepare_arr();

    if size < sz && arr[size] == needle {
        return Some(size);
    }

    let mut left: usize = 0;
    let mut right: usize = size - 1;

    while left <= right {

        if left == right && needle != arr__[left] { break; }

        if left.abs_diff(right) == 1 {
            if arr__[left] == needle {
                return Some(left);
            }
            if arr__[right] == needle {
                return Some(right);
            }
            break;
        }

        let middle = (left+right) / 2;
        if arr__[middle] < needle {
            left = middle;
        } else if arr__[middle] > needle {
            right = middle;
        } else {
            return Some(middle);
        }
    };

    return None;
}