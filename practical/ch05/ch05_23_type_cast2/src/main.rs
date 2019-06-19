fn main() {
    let t1 = ('a', 42);
    // let t2 = as (u32, u8);

    let v1 = vec![b'h', b'e', b'l', b'l', b'o'];
    // let v2 = v1 as Vec<u16>;

    let t3 = (t1.0 as u32, t1.1 as u8);
    let v3 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();

    let v4: Vec<u8> = From::from("hello");
    assert_eq!(v1, v4);
}
