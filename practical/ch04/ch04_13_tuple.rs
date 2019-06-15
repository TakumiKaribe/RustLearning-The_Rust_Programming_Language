fn main() {
    let t1 = (88, true);

    assert_eq!(t1.0, 88);

    assert_eq!(t1.1, true);

    let i = 0;
    // let t1a = t1.i;

    let mut t1 = (88, true);

    t1.0 += 100;

    assert_eq!(t1, (188, true));
}
