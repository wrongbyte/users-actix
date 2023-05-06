mod db;
use db::connect::connect;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let _client = connect()
        .await
        .expect("Database connection error. Quitting");
}
