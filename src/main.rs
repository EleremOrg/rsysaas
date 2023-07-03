mod business;
mod data;
mod web;

use std::net::SocketAddr;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web::routes::routes;

fn read_env_file() {
    let file = File::open(".env").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();
            let is_readable = !trimmed_line.is_empty() && !trimmed_line.starts_with('#');
            if is_readable {
                let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    env::set_var(key, value);
                }
            }
        }
    }
}

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

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
