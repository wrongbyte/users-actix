mod db;
mod domain;
mod handlers;
mod repositories;
mod routes;
mod utils;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
pub mod response;

use actix_web::{web, App, HttpServer};
use db::connect::connect;
use handlers::user::{DynUserHandler, UserHandlerImpl};
use repositories::user::SqlUserRepository;
use routes::user::user_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    env_logger::init();
    let client = connect()
        .await
        .expect("Database connection error. Quitting");
    let user_repository = Box::new(SqlUserRepository { client });

    let user_handler: Arc<DynUserHandler> = Arc::new(UserHandlerImpl { user_repository });

    let user_handler = web::Data::from(user_handler.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(user_handler.clone())
            .configure(user_routes)
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))
    .expect("Unable to run server on port {port}. Quitting")
    .run()
    .await
    .unwrap();
}
