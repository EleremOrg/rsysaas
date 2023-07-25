mod business;
mod data;

mod web;

use envy::read_env_file;

use data::orm::run_migrations;
use std::{net::SocketAddr, path::PathBuf};
use web::routes::routes;

use axum_server::tls_rustls::RustlsConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    read_env_file();

    run_migrations("migrations/sqlite/initial.sql").await;

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
