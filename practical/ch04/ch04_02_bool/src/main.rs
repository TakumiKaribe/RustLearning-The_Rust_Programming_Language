fn main() {
    let b1 = true;
    let b2 = !b1;

    let n1 = 8;
    let n2 = 12;

    let b3 = n1 >= 10;
    let b4 = n2 >= 10;

    let b5 = b3 && b4;
    let b6 = b3 || b4;

    assert_eq!(std::mem::size_of::<bool>(), 1);
}
