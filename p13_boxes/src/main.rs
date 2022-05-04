fn main() {
    let c = Box::new([0u32; 10_000_000]);
    println!("{}", c.len());
}
// stack overflow
// let x = vec![0u32; 10_000_000];
