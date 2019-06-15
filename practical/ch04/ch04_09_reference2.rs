fn main() {
    let c1 = 'A';
    let c1_ptr = &c1;
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;
    let n1_ptr = &mut n1;
    assert_eq!(*n1_ptr, 0);

    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1_000);
}
