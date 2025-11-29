mod handlers;
mod metrics;

use axum::{http::Method, routing::get, Router};
use std::{error::Error, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

pub async fn start_web_server(bind: String) -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = bind.parse()?;

    let cors = CorsLayer::new()
        .allow_origin(Any) // Do not restrict the origin
        .allow_methods([Method::GET])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(handlers::health_handler))
        .route("/board", get(handlers::board_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
