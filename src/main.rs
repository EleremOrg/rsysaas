mod api_docs;
mod customer;
mod data;
mod recommendation;
mod routes;
mod server;

use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, signal};

use routes::custom_routes;
use server::{get_router, App, AppState, Config};

fn main() {
    let config = Config::from_file("server.json").init_tracing();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(config.threads)
        .max_blocking_threads(config.threads)
        .build()
        .unwrap()
        .block_on(async {
            let state = AppState(Arc::new(App::new(&config)));
            let router = get_router(&config, state.clone(), custom_routes(state.clone()));
            let addr = TcpListener::bind((config.ip, config.port)).await.unwrap();
            axum::serve(
                addr,
                router.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .with_graceful_shutdown(shutdown_signal())
            .await
        })
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
