use axum::{
    routing::post,
    Router,
};

use crate::handlers::slack_handler;
use crate::state::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/commands",
            post(slack_handler::slash_command),
        )
}