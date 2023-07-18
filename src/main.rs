mod business;
mod data;
mod utils;
mod web;

use envy::read_env_file;
use utils::{execute_sql_file, tracing};
use web::routes::routes;

use std::{net::SocketAddr, path::PathBuf};

use axum_server::tls_rustls::RustlsConfig;

#[tokio::main]
async fn main() {
    tracing();
    read_env_file();

    execute_sql_file("migrations/sqlite/initial.sql").await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    println!("listening on {}", addr);

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    // To use with https
    // axum_server::bind_rustls(addr, config)
    //     .serve(routes().into_make_service())
    //     .await
    //     .unwrap();

    axum::Server::bind(&addr)
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
