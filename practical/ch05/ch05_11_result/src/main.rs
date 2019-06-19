fn main() {
    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);

    assert_eq!(add0("3", "127"), Ok(3 + 127));
    assert!(add0("3", "abc").is_err());

    assert_eq!(add1("3", "abc"), Err("s1が整数ではありません".to_string()));
}

fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
    let s0 = s0.parse::<i32>()?;
    let s1 = s1.parse::<i32>()?;
    Ok(s0 + s1)
}

fn add1(s0: &str, s1: &str) -> Result<i32, String> {
    let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
    let s1 = s1.parse::<i32>().map_err(|_e| "s1が整数ではありません")?;
    Ok(s0 + s1)
}
