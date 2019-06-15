fn main() {
    let t1 = (3, "birds".to_string());
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));
}
