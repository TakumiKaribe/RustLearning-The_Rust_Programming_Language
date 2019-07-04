use std::cell::RefCell;

struct A {
    c: char,
    s: String,
}

fn a() {
    let a = A { c: 'a', s: "alex".to_string() };
    let r = &a; // 不変の参照を作る
    // r.s.push('a'); // 不変の参照経由でフィールドを変更しようとするとコンパイルエラーになる
}

struct B {
    c: char,
    s: RefCell<String>, // StringをRefCellで包む
}

fn b() {
    let b = B { c: 'a', s: RefCell::new("alex".to_string()) };
    let rb = &b;
    rb.s.borrow_mut().push('a'); // フィールドsのデータに対する可変の参照をとる
    {
        let rbs = b.s.borrow(); // 不変の参照をとる
        assert_eq!(&*rbs, "alexa");

        // RefCellでは他の参照が有効な間に可変の参照をとろうとすると実行時にパニックする
        b.s.borrow_mut();
        // -> thread 'main' panicked at 'already borrowed: BorrowMutError'

        // try_borrow_mutならパニックせずErrを返してくれる
        assert!(b.s.try_borrow_mut().is_err()); // Errが返る
    } // rbsはここでスコープを抜ける
    assert!(b.s.try_borrow_mut().is_ok());
}

fn main() {}
