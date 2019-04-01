#[derive(Debug)]
struct Rectabgle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectabgle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
