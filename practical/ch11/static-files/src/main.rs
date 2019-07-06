use actix_web::{fs, server, App};

fn main() {
    server::new(|| {
        App::new().handler(
            "/",
            // StaticFilesを用いて現在のディレクトリ下のファイルを提供する
            fs::StaticFiles::new(".").unwrap(),
        )
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
