mod primitive_arr;

use crate::primitive_arr::*;
mod tests;

fn main() {
    let arr = [1,2,234,4,556,6,1888];

    let min_int = find_min(&arr);

    println!("(__i32__) min = {} in {}", min_int, arr.map(|el| el.to_string()).join(", "));
}
