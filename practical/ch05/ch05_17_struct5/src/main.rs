#[derive(Debug, PartialEq)]
struct UniqueValue;

fn main() {
    let uv1 = UniqueValue;
    let uv2 = UniqueValue;
    assert_eq!(uv1, uv2);
}
