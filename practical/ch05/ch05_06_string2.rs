fn main() {
    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));

    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();

    assert!(r2.is_err());
    println!("{:?}", r2);

    let cs = ['t', 'r', 'u', 's', 't'];
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust");

    let bad_utf8: [u8; 7] = [
        b'a',
        0xf0, 0x90, 0x80,
        0xe3, 0x81, 0x82,
    ];

    let s = String::from_utf8_lossy(&bad_utf8);
    assert_eq!(s, "a\u{fffd}あ");
}
