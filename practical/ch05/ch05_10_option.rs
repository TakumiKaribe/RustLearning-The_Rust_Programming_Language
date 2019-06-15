fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);
    let mut o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        assert_eq!(s, 20);
    }

    let mut o2 = Some(String::from("Hello"));
    // assert_eq!(o2.unwrap(), "Hello");

    o2 = None;
    // o2.unwrap();

    assert_eq!(o2.unwrap_or_else(|| String::from("o2 is none")), "o2 is none");

    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));

    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);

    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
            .and_then(|n| if n >= 200 { Some(n) } else { None }),
        None
    );

    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));

    assert_eq!(add_elems(&[7, 11]), None);
}

fn add_elems(s: &[i32]) -> Option<i32> {
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}
