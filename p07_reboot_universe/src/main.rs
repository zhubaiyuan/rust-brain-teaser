fn main() {
    // if 0.1 + 0.2 == 0.3 {
    if (0.1f32 + 0.2f32 - 0.3f32).abs() < std::f32::EPSILON {
        println!("Arithmetic still works.");
    } else {
        println!("Please reboot the universe.");
    }
}
