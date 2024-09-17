mod server;

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::{net::TcpListener, signal};

use server::{get_router, App, AppState, Config};

fn main() {
    let config = Config::from_file("server.json").init_tracing();
    let router = get_router(&config, AppState(Arc::new(App::new(&config))));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(config.threads)
        .max_blocking_threads(config.threads)
        .build()
        .unwrap()
        .block_on(async {
            axum::serve(
                TcpListener::bind((config.ip, config.port)).await.unwrap(),
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
