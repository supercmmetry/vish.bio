use std::{ffi::OsStr, fmt::Write as _, path::PathBuf};

use axum::{
    body::Body,
    extract::{Path, RawQuery},
    http::{header, HeaderMap, Response, StatusCode},
    response::ErrorResponse,
};
use mime_guess::{mime, MimeGuess};
use rust_embed::RustEmbed;

/// Baked into the binary in release builds; read from disk at runtime in debug builds.
/// `build.rs` keeps the macro expansion honest when files are added or removed.
#[derive(RustEmbed)]
#[folder = "assets/"]
#[exclude = ".gitignore"]
#[exclude = "*.DS_Store"]
#[exclude = "**/.DS_Store"]
pub struct Assets;

/// Short content hash used as a `?v=` cache-busting token on asset URLs.
///
/// Debug builds recompute this per request because rust-embed reads from disk there, so
/// editing a stylesheet takes effect on reload without restarting the server.
pub fn fingerprint(path: &str) -> String {
    Assets::get(path)
        .map(|file| hex(&file.metadata.sha256_hash()[..6]))
        .unwrap_or_else(|| "dev".to_owned())
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

pub async fn serve(
    Path(path): Path<String>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
) -> axum::response::Result<Response<Body>> {
    let file = Assets::get(&path).ok_or_else(|| ErrorResponse::from(StatusCode::NOT_FOUND))?;

    let ext = PathBuf::from(&path)
        .extension()
        .and_then(OsStr::to_str)
        .map(str::to_owned)
        .ok_or_else(|| ErrorResponse::from(StatusCode::BAD_REQUEST))?;

    let guessed = MimeGuess::from_ext(&ext)
        .first()
        .ok_or_else(|| ErrorResponse::from(StatusCode::BAD_REQUEST))?;

    // MimeGuess never carries a charset. Without one, a `content: "→"` in the stylesheet
    // decodes as mojibake in browsers that fall back to latin-1.
    let content_type = match (guessed.type_(), guessed.subtype().as_str()) {
        (mime::TEXT, _) | (mime::APPLICATION, "javascript") => format!("{guessed}; charset=utf-8"),
        _ => guessed.to_string(),
    };

    let etag = format!("\"{}\"", hex(&file.metadata.sha256_hash()[..8]));

    // Versioned URLs are immutable by construction: change the bytes, change the token.
    // Everything else revalidates cheaply against the ETag.
    let cache_control = if cfg!(debug_assertions) {
        "no-cache"
    } else if query.as_deref().is_some_and(|q| q.starts_with("v=")) {
        "public, max-age=31536000, immutable"
    } else {
        "public, max-age=3600, stale-while-revalidate=86400"
    };

    let fresh = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.split(',').any(|candidate| candidate.trim() == etag));

    let builder = Response::builder()
        .header(header::CACHE_CONTROL, cache_control)
        .header(header::ETAG, &etag)
        .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff");

    let response = if fresh {
        builder.status(StatusCode::NOT_MODIFIED).body(Body::empty())
    } else {
        builder
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from(file.data))
    };

    response.map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR).into())
}
