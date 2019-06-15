fn main() {
    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    assert_eq!(unsafe { *c1_ptr }, 'A');

    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!(unsafe { *n1_ptr }, 0);

    unsafe {
        *n1_ptr = 1_000;
        assert_eq!(*n1_ptr, 1_000);
    }
}
