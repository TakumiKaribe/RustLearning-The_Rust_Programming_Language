fn f1(p: &[i32]) -> i32 { p[0] }
fn f2(p: Box<[i32]>) -> i32 { p[0] }

fn main() {
    let a1 = [1, 2, 3, 4];
    assert_eq!(1, f1(&a1));
    assert_eq!(1, f2(Box::new(a1)));

    let mut d: Box<std::fmt::Debug>;

    d = Box::new([1, 2]);
    d = Box::new(Some(1));
}
