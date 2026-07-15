use axum::{
    extract::{Form, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::models::slack::SlashCommandRequest;
use crate::services::slack_service::CommandContext;
use crate::state::app_state::AppState;

/// Acks immediately (Slack requires a response within 3s), then does the
/// real work in the background — SlackService picks how (and where) to
/// deliver the result.
pub async fn slash_command(
    State(state): State<AppState>,
    Form(payload): Form<SlashCommandRequest>,
) -> impl IntoResponse {
    let slack_service = state.slack_service.clone();

    tokio::spawn(async move {
        slack_service
            .handle_command(CommandContext {
                user_id: &payload.user_id,
                channel_id: &payload.channel_id,
                response_url: &payload.response_url,
                text: &payload.text,
            })
            .await;
    });

    Json(json!({
        "response_type": "ephemeral",
        "text": "Working on it…",
    }))
}