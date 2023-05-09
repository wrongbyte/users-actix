mod db;
mod domain;
mod handlers;
mod repositories;
mod routes;
mod utils;
use actix_web::{App, HttpServer};
use db::connect::connect;
use routes::user::user_routes;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let _client = connect()
        .await
        .expect("Database connection error. Quitting");

    HttpServer::new(move || {
        App::new()
            .configure(user_routes)
    })
    .bind(("127.0.0.1", 3000))
    .expect("Unable to run server on port 3000. Quitting")
    .run()
    .await
    .unwrap();
}
