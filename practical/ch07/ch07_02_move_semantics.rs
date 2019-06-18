#[derive(Debug)]
struct Parent(isize, Child, Child);

#[derive(Debug)]
struct Child(isize);

fn f1(p: Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &Parent) {
    println!("p: {:?}", p);
}

fn f3(p: &mut Parent) {
    p.0 *= -1;
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1);

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1);

    let p3 = Parent(3, Child(31), Child(32));
    f1(p3);
    // println!("p3: {:?}", p3);

    let mut p4 = Parent(4, Child(41), Child(42));
    f2(&p4);
    f3(&mut p4);
    println!("p4: {:?}", p4);
}
