pub mod assets;
pub mod content;
pub mod pages;

use axum::{response::IntoResponse, response::Redirect, routing::get, Router};
use pages::index::IndexTemplate;

async fn index() -> impl IntoResponse {
    IndexTemplate::new()
}

async fn jira() -> impl IntoResponse {
    Redirect::permanent("https://vishaalselvaraj.atlassian.net")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        // Catch-all, not `:asset_name` — a single-segment param cannot match
        // `/assets/fonts/…` or `/assets/js/…`.
        .route("/assets/*asset_path", get(assets::serve))
        .route("/jira", get(jira));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|e| panic!("failed to bind 0.0.0.0:{port}: {e}"));

    println!("listening on http://0.0.0.0:{port}");

    axum::serve(listener, app).await.unwrap();
}
