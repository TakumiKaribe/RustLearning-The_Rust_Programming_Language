struct Rectabgle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectabgle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectabgle) -> u32 {
    rectangle.width * rectangle.height
}
