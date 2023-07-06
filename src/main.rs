mod business;
mod data;
mod utils;
mod web;

use envy::read_env_file;
use std::net::SocketAddr;
use utils::{execute_sql_file, tracing};
use web::routes::routes;

#[tokio::main]
async fn main() {
    tracing();
    read_env_file();

    execute_sql_file("migrations/sqlite/initial.sql").await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
