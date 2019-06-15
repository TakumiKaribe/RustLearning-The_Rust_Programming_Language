fn f1(mut n: u32) {
    n = 1;
    println!("f1:   n = {}", n);
}

fn f2(n_ptr: &mut u32) {
    println!("f2:   n_ptr = {:p}", n_ptr);

    *n_ptr = 2;
    println!("f2:   *n_ptr = {}", *n_ptr);
}

fn main() {
    let mut n = 0;
    println!("main: n = {}", n);

    f1(n);
    println!("main: n = {}", n);

    f2(&mut n);
    println!("main: n = {}", n);
}
