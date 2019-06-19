fn main() {
    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57];

    if let Ok(s) = String::from_utf16(&utf16) {
        assert_eq!(s, "ab漢字");
    } else {
        unreachable!();
    }

    let bs1 = b"abs\xe3\x81\x82";
    assert_eq!(bs1, &[b'a', b'b', b'c', 0xe3, 0x81, 0x82]);

    let bs2 = br#"ab\ncd"#;
    assert_eq!(bs2, &[b'a', b'b', b'\\', b'n', b'c', b'd']);
}
