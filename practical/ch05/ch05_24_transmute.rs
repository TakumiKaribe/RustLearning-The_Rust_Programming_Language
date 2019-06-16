fn main() {
    let p1 = Box::new(10);

    // let p2 = p1 as *mut i32;

    let p3: *mut i32 = unsafe { std::mem::transmute(p1) };

    let f1 = 5.6789e+3_f32;

    let i1 = f1 as i32;
    println!("{}", i1);

    let i2: i32 = unsafe { std::mem::transmute(f1) };
    println!("{}", i2);
}
