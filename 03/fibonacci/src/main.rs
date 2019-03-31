use std::io;

fn main() {
    println!("Please input to calc fibonacci number.");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line.");
    let number: i32 = number.trim().parse().expect("Failed to parse number");
    println!("ficonacci {} is {}", number, calc_fibonacci(number));
}

fn calc_fibonacci(i: i32) -> i32 {
    if i == 0 {
        0
    } else if i == 1 {
        1
    } else {
        calc_fibonacci(i - 1) + calc_fibonacci(i - 2)
    }
}
