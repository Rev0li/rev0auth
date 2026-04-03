use rev0auth_api::app::build_router;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .compact()
        .init();

    let app = build_router().await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!(%addr, "API listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
