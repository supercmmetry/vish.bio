pub mod pages;

#[macro_use]
extern crate rust_embed;

use std::{ffi::OsStr, path::PathBuf};

use axum::{
    body::Body,
    extract::Path,
    http::{header, Response, StatusCode},
    response::{ErrorResponse, IntoResponse},
    routing::get,
    Router,
};
use axum::response::Redirect;
use mime_guess::MimeGuess;
use pages::index::IndexTemplate;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

async fn assets(Path(asset_name): Path<String>) -> axum::response::Result<impl IntoResponse> {
    Assets::get(&asset_name).map_or_else(
        || Err(ErrorResponse::from(StatusCode::NOT_FOUND)),
        |d| {
            let path = PathBuf::from(&asset_name);

            let ext = path
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| ErrorResponse::from(StatusCode::BAD_REQUEST))?;

            let content_type = MimeGuess::from_ext(ext)
                .first()
                .ok_or_else(|| ErrorResponse::from(StatusCode::BAD_REQUEST))?;

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type.to_string())
                .body(Body::from(d.data))
                .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;

            Ok(response)
        },
    )
}

async fn index() -> impl IntoResponse {
    IndexTemplate
}

async fn jira() -> impl IntoResponse {
    Redirect::permanent("https://vishaalselvaraj.atlassian.net")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/assets/:asset_name", get(assets))
        .route("/jira", get(jira));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
