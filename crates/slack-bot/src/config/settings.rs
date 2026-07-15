use std::env;

#[derive(Clone)]
pub struct Settings {
    pub slack_signing_secret: String,
    /// Needed only for `chat.postMessage`/threaded replies (`chat:write` scope).
    /// Slash-command replies via `response_url` work without it.
    pub slack_bot_token: Option<String>,
}

impl Settings {
    pub fn from_env() -> Self {
        Self {
            slack_signing_secret: env::var("SLACK_SIGNING_SECRET")
                .expect("missing SLACK_SIGNING_SECRET"),
            slack_bot_token: env::var("SLACK_BOT_TOKEN").ok(),
        }
    }
}