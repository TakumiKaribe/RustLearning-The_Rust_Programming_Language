fn main() {
    let v1 = vec![false, true, false];
    let v2 = vec![0.0, -1.0, 1.0, 0.5];

    assert_eq!(v2.len(), 4);

    let v3 = vec![0; 100];
    assert_eq!(v3.len(), 100);

    let v4 = vec![vec!['a', 'b', 'c'], vec!['d']];

//    let v5 = vec![false, 'a'];
}
