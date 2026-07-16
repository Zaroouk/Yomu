use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

/// Everything about *sending* a message to Slack lives here. Callers hand it
/// a JSON payload (built by `views.rs`) and where to send it — this is the
/// only place that knows about response_url / chat.postMessage / auth.
#[derive(Clone)]
pub struct SlackClient {
    http: Client,
    bot_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PostMessageResponse {
    ok: bool,
    ts: Option<String>,
    error: Option<String>,
}

impl SlackClient {
    pub fn new(http: Client, bot_token: Option<String>) -> Self {
        Self { http, bot_token }
    }

    pub fn has_bot_token(&self) -> bool {
        self.bot_token.is_some()
    }

    /// One-shot reply to a slash command via its `response_url` (valid for
    /// ~30 minutes after the command). Can't be threaded.
    pub async fn respond(&self, response_url: &str, payload: Value) -> anyhow::Result<()> {
        self.http
            .post(response_url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn respond_in_thread(&self, response_url: &str, payload: Value) -> anyhow::Result<()> {
        let res = self.http
            .post(response_url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

            res;
            println!("[LOGGER]");
          let body_json: Value = res.json().await?;
          println!("{:#?}", body_json);
        
        Ok(())
    }

    /// Post into a channel via the bot token (requires `chat:write` +
    /// `SLACK_BOT_TOKEN`). Pass `thread_ts` to reply inside a thread.
    /// Returns the posted message's `ts`, which you can pass back in as
    /// `thread_ts` for follow-up replies underneath it.
    pub async fn post_message(
        &self,
        channel: &str,
        thread_ts: Option<&str>,
        mut payload: Value,
    ) -> anyhow::Result<String> {
        let bot_token = self
            .bot_token
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("SLACK_BOT_TOKEN not configured"))?;

        if let Some(obj) = payload.as_object_mut() {
            obj.insert("channel".into(), channel.into());
            if let Some(ts) = thread_ts {
                obj.insert("thread_ts".into(), ts.into());
            }
        }

        let res: PostMessageResponse = self
            .http
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(bot_token)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        if !res.ok {
            anyhow::bail!("chat.postMessage failed: {:?}", res.error);
        }

        res.ts
            .ok_or_else(|| anyhow::anyhow!("chat.postMessage response missing ts"))
    }
}