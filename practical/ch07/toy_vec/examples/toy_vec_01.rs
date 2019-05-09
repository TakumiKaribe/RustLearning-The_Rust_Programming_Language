use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::with_capacity(2);
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));
}
