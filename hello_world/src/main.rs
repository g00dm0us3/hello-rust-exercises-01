fn main() {
    for num in 0..21 {
        let raised = 1 << num;
        println!("2^{num} = {raised}");
    }
}
