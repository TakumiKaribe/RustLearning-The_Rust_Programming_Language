fn f1(n: &mut usize, str: &str, slice: &[i32]) {
    *n = str.len() + slice.len()
}

fn main() {
    let mut b1 = Box::new(0);
    let s1 = String::from("deref");
    let v1 = vec![1, 2, 3];

    f1(&mut b1, &s1, &v1);
    assert_eq!(8, *b1);
}
