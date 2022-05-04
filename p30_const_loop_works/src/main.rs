const fn fib(n: u128) -> u128 {
    let mut a = 1;
    let mut b = 1;
    let mut counter = 2;
    while counter < n {
        let tmp = a + b;
        a = b;
        b = tmp;
        counter += 1;
    }
    b
}

fn main() {
    for i in 0..5 {
        println!("Fib {} = {}", i, fib(i));
    }
}
// Fib 0 = 1
// Fib 1 = 1
// Fib 2 = 1
// Fib 3 = 2
// Fib 4 = 3
