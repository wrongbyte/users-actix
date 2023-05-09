use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use http_problem::prelude::*;
use tokio_postgres::Client;

pub async fn connect() -> Result<Arc<Client>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let (client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .unwrap();

    tokio::spawn(async move { connection.await.unwrap() });
    Ok(Arc::new(client))
}
