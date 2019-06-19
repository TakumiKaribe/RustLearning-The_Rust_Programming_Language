fn main() {
    let mut v1 = vec![0, 1, 2, 3];
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());

    let s1 = v1.into_boxed_slice();

    let v2 = s1.into_vec();
    println!("v1 len: {}, capacity: {}", v2.len(), v2.capacity());
}
