mod business;
mod cli;
mod data;
mod web;

use std::net::SocketAddr;

use cli::{is_cli_requested, run_cli};
use tracing_appender::{non_blocking, rolling::hourly};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use menva::{get_env, read_env_file};

use web::routes::routes;

#[tokio::main]
async fn main() {
    read_env_file(".env");

    let (non_blocking, _guard) = non_blocking(hourly(get_env("LOGS_PATH"), "webservice"));
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "webservice=trace,aromatic=trace,tower_http=trace,axum::rejection=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(non_blocking)
                .log_internal_errors(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_current_span(true)
                .with_span_events(FmtSpan::FULL)
                .with_span_list(true)
                .with_target(true),
        )
        .init();

    if is_cli_requested().await {
        run_cli().await;
        return;
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
