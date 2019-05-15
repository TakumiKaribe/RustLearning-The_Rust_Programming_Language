use toy_vec::ToyVec;

fn main() {
    let _e: Option<&String>;
    {
        let mut v = ToyVec::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());

        _e = v.get(1);
    }
}
