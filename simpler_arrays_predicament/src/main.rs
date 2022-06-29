mod primitive_arr;

use crate::primitive_arr::*;
mod tests;

fn main() {
    let arr = [1,2,234,4,556,6,1888];

    let min_int = find_min(&arr);

    let arr_string = arr.map(|el| el.to_string()).join(", ");
    println!("(__i32__) min = {} in {}", min_int, arr_string);

    let sorted_qual = if is_sorted(&arr, 7) { "is sorted "} else { "is unsorted" };

    println!("array {} {}", arr_string, sorted_qual);

    
}
