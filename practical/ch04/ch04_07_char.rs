fn main() {
    let c1 = 'A';
    let c2 = 'a';
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    let c4 = '\t';
    let c5 = '\n';
    let c6 = '\'';
    let c7 = '\\';
    let c8 = '\x7F';

    let c9 = '漢';

    let c10 = '\u{5b57}';
    let c11 = '\u{1f600}';

    assert_eq!(std::mem::size_of::<char>(), 4);
}
