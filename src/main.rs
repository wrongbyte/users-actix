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
            // .app_data(new_todo_controller.clone())
            .configure(user_routes)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Unable to run server on port 8080. Quitting")
    .run()
    .await
    .unwrap();
}
