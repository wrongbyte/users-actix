mod domain;
mod handlers;
mod repositories;
mod routes;
mod utils;
mod error;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
pub mod response;

use actix_web::{web, App, HttpServer};
use sqlx::{PgPool, postgres::PgPoolOptions};
use handlers::user::{DynUserHandler, UserHandlerImpl};
use repositories::user::SqlUserRepository;
use routes::{user::user_routes, auth::auth_routes};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let pool: PgPool = PgPoolOptions::new()
        .connect(&database_url)
        .await.expect("Could not connect to database");

    let user_repository = Box::new(SqlUserRepository { pool });

    let user_handler: Arc<DynUserHandler> = Arc::new(UserHandlerImpl { user_repository });

    let user_handler = web::Data::from(user_handler.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(user_handler.clone())
            .configure(user_routes)
            .configure(auth_routes)
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))
    .expect("Unable to run server on port {port}. Quitting")
    .run()
    .await
    .unwrap();
}
