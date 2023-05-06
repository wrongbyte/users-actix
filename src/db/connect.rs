use std::sync::Arc;

use http_problem::prelude::*;
use tokio_postgres::Client;

pub async fn connect() -> Result<Arc<Client>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=onboarding",
        tokio_postgres::NoTls,
    )
    .await.unwrap();

    tokio::spawn(async move { connection.await.unwrap() });
    Ok(Arc::new(client))
}
