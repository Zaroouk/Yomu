use std::sync::Arc;

use crate::services::search_provider::SearchProvider;
use crate::services::slack_client::SlackClient;
use crate::services::views;

#[derive(Clone)]
pub struct SlackService {
    search_provider: Arc<dyn SearchProvider>,
    slack_client: SlackClient,
}

pub struct CommandContext<'a> {
    pub user_id: &'a str,
    pub channel_id: &'a str,
    pub response_url: &'a str,
    pub text: &'a str,
}

impl SlackService {
    pub fn new(search_provider: Arc<dyn SearchProvider>, slack_client: SlackClient) -> Self {
        Self {
            search_provider,
            slack_client,
        }
    }

    /// Parses the command text into a verb + args, dispatches it, and
    /// delivers the result to Slack itself (each verb picks its own
    /// view + transport). Errors are logged, not returned, since by the
    /// time this runs the slash command has already been ack'd.
  pub async fn handle_command(&self, ctx: CommandContext<'_>) {
    let mut parts = ctx.text.trim().splitn(2, char::is_whitespace);
    let verb = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("").trim();
    tracing::info!(user_id = ctx.user_id, verb, rest, "handling slash command");

    let result = match verb {
        "search" if !rest.is_empty() => self.handle_search(ctx.response_url, rest).await,
        "chapters" if !rest.is_empty() => {
            self.handle_chapters(ctx.channel_id, ctx.response_url, rest)
                .await
        }
        "chapter" if !rest.is_empty() => self.handle_chapter(ctx.response_url, rest).await,
        "test" => self.handle_thread_test(ctx.channel_id).await,
        _ => {
            let payload = views::list_view(
                "Usage: `/manhwa search <title>` | `/manhwa chapters <manhwa_url>` | `/manhwa chapter <chapter_url>` | `/manhwa test`",
                &[],
            );
            self.slack_client.respond(ctx.response_url, payload).await
        }
    };

    if let Err(err) = result {
        tracing::error!("failed to deliver slack response: {err:#}");
    }
}

    /// Favorites/search results: a flat list, replied to the invoking
    /// command via response_url.
    async fn handle_search(&self, response_url: &str, query: &str) -> anyhow::Result<()> {
        let results = self.search_provider.search(query).await?;
        let items: Vec<(String, String)> =
            results.into_iter().map(|r| (r.title, r.url)).collect();

        let payload = views::list_view(&format!("Results for \"{query}\":"), &items);
        self.slack_client.respond(response_url, payload).await
    }

    /// Chapters: posted as a parent message + one threaded reply per
    /// chapter, so the channel doesn't get flooded with a wall of text.
    /// Falls back to a flat list via response_url if no SLACK_BOT_TOKEN is
    /// configured (threading needs the bot token; response_url can't thread).
    async fn handle_chapters(
        &self,
        channel_id: &str,
        response_url: &str,
        manhwa_url: &str,
    ) -> anyhow::Result<()> {
        let chapters = self.search_provider.list_chapters(manhwa_url).await?;

        if chapters.is_empty() {
            let payload = views::list_view("No chapters found.", &[]);
            return self.slack_client.respond(response_url, payload).await;
        }

        if !self.slack_client.has_bot_token() {
            let items: Vec<(String, String)> = chapters
                .into_iter()
                .map(|c| (c.title, c.url))
                .collect();
            let payload = views::list_view(&format!("Chapters for {manhwa_url}:"), &items);
            return self.slack_client.respond(response_url, payload).await;
        }

        let parent_ts = self
            .slack_client
            .post_message(
                channel_id,
                None,
                views::list_view(&format!("Chapters for {manhwa_url}:"), &[]),
            )
            .await?;

        for chapter in &chapters {
            self.slack_client
                .post_message(
                    channel_id,
                    Some(&parent_ts),
                    views::thread_item_view(&chapter.title, &chapter.url),
                )
                .await?;
        }

        Ok(())
    }

    /// Chapter images: fetches the chapter's pages and replies with one
    /// image block per page (chunked across messages if the chapter is long).
    async fn handle_chapter(&self, response_url: &str, chapter_url: &str) -> anyhow::Result<()> {
        let chapter = self.search_provider.chapter(chapter_url).await?;

        for payload in views::chapter_images_view(&chapter.title, &chapter.pages) {
            self.slack_client.respond(response_url, payload).await?;
            //self.slack_client.respond_in_thread(response_url, payload).await?;
            
        }
        //for payload in views::thread_item_view(&chapter.title,&chapter.pages)

        Ok(())
    }

    pub async fn post_message(
     &self,
    channel: &str,
    text: &str,
    thread_ts: Option<&str>,
) -> anyhow::Result<String> {
    let payload = serde_json::json!({ "text": text });
    self.slack_client.post_message(channel, thread_ts, payload).await
}
pub async fn handle_thread_test(&self, channel_id: &str) -> anyhow::Result<()> {
    let thread_ts = self
        .slack_client
        .post_message(channel_id, None, json!({ "text": "Starting test job..." }))
        .await?;

    self.slack_client
        .post_message(channel_id, Some(&thread_ts), json!({ "text": "Step 1 done ✅" }))
        .await?;

    self.slack_client
        .post_message(channel_id, Some(&thread_ts), json!({ "text": "Step 2 done ✅" }))
        .await?;

    self.slack_client
        .post_message(channel_id, Some(&thread_ts), json!({ "text": "Done!" }))
        .await?;

    Ok(())
}
}