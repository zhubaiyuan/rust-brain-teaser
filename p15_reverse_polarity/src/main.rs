fn display_neutron_flow(polarity: isize) {
    println!(
        "Neutron Flow is {}",
        if polarity < 0 { "reversed" } else { "normal" }
    );
}

fn main() {
    let polarity = 1;
    {
        let polarity = polarity - 2;
        display_neutron_flow(polarity);
    }
    display_neutron_flow(polarity);
    {
        let x = 123; //(1)
        {
            //(2)
            let mut x = x as f32; //(3)
            x *= 2.0; //(4)
            println!("x = {}", x); //(5)
        } //(6)
        println!("x = {}", x); //(7)
    }
}
// Neutron Flow is reversed
// Neutron Flow is normal
// x = 246
// x = 123
