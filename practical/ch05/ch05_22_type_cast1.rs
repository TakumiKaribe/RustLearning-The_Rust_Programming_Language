fn main() {
    let i1 = 42;
    let f1 = i1 as f64 / 2.5;

    let c1 = 'a';
    assert_eq!(97, c1 as u32);

    let i2 = 300;
    let u1 = i2 as u8;

    assert_eq!(44, u1);
}
