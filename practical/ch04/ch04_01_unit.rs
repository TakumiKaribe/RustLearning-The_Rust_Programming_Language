fn hello() {
    println!("Hello");
}

fn main() {
    let ret = hello();
    assert_eq!(ret, ());
    assert_eq!(std::mem::size_of::<()>(), 0);
}
