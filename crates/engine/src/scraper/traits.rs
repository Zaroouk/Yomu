use crate::models::chapter::Chapter;
use crate::models::chapter_summary::ChapterSummary;
use crate::models::search_result::SearchResult;

#[async_trait::async_trait]
pub trait Provider {
    /// Look up manhwa by name/title, e.g. from a Slack search command.
    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>>;

    /// List the available chapters for a given manhwa (as returned by `search`).
    async fn list_chapters(&self, manhwa_url: &str) -> anyhow::Result<Vec<ChapterSummary>>;

    /// Fetch a single chapter's pages, given a chapter URL (as returned by `list_chapters`).
    async fn chapter(&self, url: &str) -> anyhow::Result<Chapter>;
}