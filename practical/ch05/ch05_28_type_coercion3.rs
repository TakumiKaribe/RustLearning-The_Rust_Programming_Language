fn f1(slice: &[usize]) -> usize {
    slice.len()
}

fn f2(slice: &mut [usize]) {
    let len = f1(slice);
    slice[0] = len;
}

fn main() {
    let mut v = vec![0; 10];
    f2(&mut v[..]);
    assert_eq!(10, v[0]);
}
