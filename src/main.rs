mod db;
mod domain;
mod handlers;
mod repositories;
mod routes;
mod utils;
use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use db::connect::connect;
use handlers::user::UserHandlerImpl;
use repositories::user::SqlUserRepository;
use routes::user::user_routes;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let _client = connect()
        .await
        .expect("Database connection error. Quitting");

    let user_repo = Arc::new(SqlUserRepository);

    let user_handler = Arc::new(UserHandlerImpl {
        user_repository: user_repo.clone(),
    });

    let user_handler = web::Data::from(user_handler);

    HttpServer::new(move || {
        App::new()
            .app_data(user_handler.clone())
            .configure(user_routes)
    })
    .bind(("127.0.0.1", 3000))
    .expect("Unable to run server on port 3000. Quitting")
    .run()
    .await
    .unwrap();
}
