fn main() {
    let v1: Vec<u8> = vec![3, 4, 5];
    // let v1 = vec![3u8, 4u8, 5u8];

    assert_eq!(Some(&3u8), v1.first());
    // assert_eq!(Some(&3u8), (&v1[..].first()));

    let mut s1 = String::from("Type coercion ");
    let s2 = String::from("is actually easy.");

    s1.push_str(&s2);
    // (&mut s1).push_str(s2.as_str());

    let mut i1 = 0u8;
    i1 = 255;

    let p1 = &&&&[1, 2, 3, 4];
    let p2: &[i32; 4] = p1;

    let p3 = &&[1, 2, 3, 4];

    let p4: &[i32; 4] = p3;
    let p5: &[i32] = p4;

    // let p6: &[i32] = p3;
}
