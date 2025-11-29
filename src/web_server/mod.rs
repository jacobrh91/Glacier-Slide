mod handlers;
mod metrics;

use axum::{routing::get, Router};
use std::{error::Error, net::SocketAddr};

pub async fn start_web_server(bind: String) -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = bind.parse()?;

    // Router wiring is small enough to live here for now.
    let app = Router::new()
        .route("/health", get(handlers::health_handler))
        .route("/board", get(handlers::board_handler));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
