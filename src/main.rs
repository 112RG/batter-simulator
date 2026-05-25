use std::net::SocketAddr;

use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;
mod handler;
mod response;
mod routes;
mod worker;

#[tokio::main]
async fn main() {
    let shutdown = CancellationToken::new();
    {
        let shutdown = shutdown.clone();

        tokio::spawn(async move {
            if let Err(err) = worker::battery_worker::battery_worker(shutdown).await {
                eprintln!("battery worker failed: {err:#}");
            }
        });
    }
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=trace"))
                .unwrap(),
        )
        .init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, routes::root::routes()).await.unwrap();
}
