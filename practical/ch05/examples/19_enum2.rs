use crate::Task::*;

type UserName = String;

#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(UserName),
    Working {
        assignee: UserName,
        remaining_hours: u16,
    },
    Done,
}

fn main() {
    let tasks = vec![
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => {
                println!("タスク{}は{}さんにアサインされています", i, assignee)
            }
            Working { assignee, remaining_hours } => {
                println!("タスク{}は{}さんが作業中です。残り{}時間の見込み",
                    i, assignee, remaining_hours)
            }
            _ => println!("タスク{}はその他のステータス（{:?}）です", i, task)
        }
    }
}
