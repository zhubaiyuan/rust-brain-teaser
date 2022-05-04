use std::cmp::Ordering::Less;

fn float_sort<T: PartialOrd>(data: &mut [T]) {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
}

fn main() {
    let mut floats = vec![3.1, 1.2, 4.5, 0.3, std::f32::INFINITY, std::f32::NAN];
    float_sort(&mut floats);
    println!("{:#?}", floats);
}
