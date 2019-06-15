fn print_info(name: &str, s1: &[char]) {
    println!("  {:9} - {}, {:?}, {:?}, {:?}",
        name,
        s1.len(),
        s1.first(),
        s1[1],
        s1.last()
    );
}

fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    println!("a1: {:?}", a1);

    print_info("&a1[..]", &a1[..]);
    print_info("&a1", &a1);
    print_info("&a1[1..3]", &a1[1..3]);

    let v1 = vec!['e', 'f', 'g', 'h'];
    println!("\nv1: {:?}", v1);

    print_info("&v1[..]", &v1[..]);
    print_info("&v1", &v1);
    print_info("&v1[1..3]", &v1[1..3]);
}
