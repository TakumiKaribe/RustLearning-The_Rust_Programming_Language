#[derive(Debug, PartialEq)]
enum Weekday {
    Monday, Tuesday, Wednesday, Thursday, Friday,
}

enum Month {
    January = 1, February = 2, March = 3, /* 中略 */ December = 12,
}

fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("TGIF!");
    } else {
        println!("まだ{:?}か", weekday);
    }
}

fn main() {
    say_something(Weekday::Friday);

    assert_eq!(3, Month::March as isize);
}
