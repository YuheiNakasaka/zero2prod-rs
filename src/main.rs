use std::net::TcpListener;

use zero2prod::run;

// `cargo expand`で見るとわかるが
// #[actix_web::main]によりentrypointはasyncが使えないので`actix_web::rt`のランタイムで実行されるようになってる
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    run(listener)?.await
}
