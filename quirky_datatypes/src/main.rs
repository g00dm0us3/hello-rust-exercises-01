
fn main() {
    let t:u8 = 255;

    // destructure a tuple
    let (val, overflow) =  t.overflowing_add(1);

    println!("val {val}, did overflow: {overflow}");

    let sat_max = t.saturating_add(1);
    println!("saturated max {sat_max}");

    let sat_min = (0 as u8).saturating_sub(1);
    println!("saturated min {sat_min}");

    let checked = t.checked_add(1);

    let def = checked.unwrap_or_default();
    println!("unwrapped_default from checked_add {def}");

    let a = [3; 5];

    for i in a {
        // 3 3 3 3 3
        println!("{i}");
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for i in a {
        println!("{i}");
    }

    // uncomment to see bounds checked
    // println!("{a[5]}")
    
}
