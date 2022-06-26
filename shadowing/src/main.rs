// variables are immutable, shadowing creates a new var, to be used in the scope
fn main() {

    let s = 5;

    {
        // uncomment to get double assignment error
        //s = s *45;
        let s = s*45;

        // 5 * 45
        println!("{s}")
    }

    // 5
    println!("{s}");

    let mut s = s - 4;

    // 1024
    s <<= 10;
    println!("{s}")
}
