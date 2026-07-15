use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlashCommandRequest {
    pub command: String,
    pub text: String,
    pub user_id: String,
    pub channel_id: String,
    pub response_url: String,
}