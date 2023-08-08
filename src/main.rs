mod business;
mod data;

mod web;

use aromatic::run_cli;
use envy::{get_env, read_env_file};

use std::{env, net::SocketAddr};
use web::routes::routes;

use tracing_appender::{non_blocking, rolling::hourly};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

async fn is_cli_requested() -> bool {
    env::args().collect::<Vec<String>>().len() > 1
}

#[tokio::main]
async fn main() {
    read_env_file();

    if is_cli_requested().await {
        run_cli().await;
        return;
    }

    let (non_blocking, _guard) = non_blocking(hourly(get_env("LOGS_PATH"), "webservice"));
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "webservice=error,aromatic=error,tower_http=error,axum::rejection=error".into()
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
