use std::io;

fn main() {
    println!("Please input temperture!");

    let mut temperture = String::new();
    io::stdin().read_line(&mut temperture).expect("Failed to read line");

    let temperture: f64 = temperture.trim().parse().expect("Failed to parse input");

    println!("convert to Fahrenheit!");
    println!("{} to {}.", temperture, to_fahrenheit(temperture));
    println!();
    println!("convert to Celsius!");
    println!("{} to {}.", temperture, to_celsius(temperture));
}

fn to_fahrenheit(x: f64) -> f64 {
    (9.0 / 5.0) * x
}

fn to_celsius(x: f64) -> f64 {
    (5.0 / 9.0) * (x - 32.0)
}
