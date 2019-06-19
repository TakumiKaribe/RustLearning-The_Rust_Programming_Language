fn main() {
    let n1 = 200u8;
    let n2 = 3u8;

    assert_eq!(n1.checked_mul(n2), None);

    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);

    assert_eq!(n1.wrapping_mul(n2), 88);

    assert_eq!(n1.overflowing_mul(n2), (88, true));
}
