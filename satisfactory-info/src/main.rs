use axum::{response::Html, routing::get, Router};
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Wow, such webdev, lala</h1>") }))
        .layer(LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await?;
    axum::serve(listener, app).await?;

    Ok(())
}