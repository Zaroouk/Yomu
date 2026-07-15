use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::state::app_state::AppState;
use crate::utils::crypto::verify_signature;

const MAX_TIMESTAMP_SKEW_SECS: i64 = 60 * 5;

pub async fn verify(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let (parts, body) = req.into_parts();

    let timestamp = parts
        .headers
        .get("X-Slack-Request-Timestamp")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let signature = parts
        .headers
        .get("X-Slack-Signature")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let (timestamp, signature) = match (timestamp, signature) {
        (Some(t), Some(s)) => (t, s),
        _ => return (StatusCode::UNAUTHORIZED, "missing slack headers").into_response(),
    };

    let is_fresh = timestamp
        .parse::<i64>()
        .ok()
        .map(|ts| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            (now - ts).abs() < MAX_TIMESTAMP_SKEW_SECS
        })
        .unwrap_or(false);

    if !is_fresh {
        return (StatusCode::UNAUTHORIZED, "stale request").into_response();
    }

    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid body").into_response(),
    };

    let body_str = String::from_utf8_lossy(&body_bytes);

    if !verify_signature(
        &state.settings.slack_signing_secret,
        &timestamp,
        &body_str,
        &signature,
    ) {
        return (StatusCode::UNAUTHORIZED, "invalid signature").into_response();
    }

    let req = Request::from_parts(parts, Body::from(body_bytes));
    next.run(req).await
}