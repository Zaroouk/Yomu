use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

pub type ChapterResult = SearchResult;

#[derive(Debug, Clone)]
pub struct ChapterImages {
    pub title: String,
    pub pages: Vec<String>,
}

/// Implement this against your real scraping/engine code (e.g. wrap
/// `engine::scraper::traits::Provider` — see `engine_provider.rs`) and swap
/// `StubSearchProvider` for it in `main.rs`. Nothing else in slack-bot needs
/// to change.
#[async_trait]
pub trait SearchProvider: Send + Sync {
    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>>;

    async fn list_chapters(&self, manhwa_url: &str) -> anyhow::Result<Vec<ChapterResult>>;

    /// Fetch a single chapter's page images, given a chapter URL.
    async fn chapter(&self, chapter_url: &str) -> anyhow::Result<ChapterImages>;
}

pub struct StubSearchProvider;

#[async_trait]
impl SearchProvider for StubSearchProvider {
    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        Ok(vec![SearchResult {
            title: format!("(stub result for \"{query}\")"),
            url: "https://example.com".to_string(),
        }])
    }

    async fn list_chapters(&self, manhwa_url: &str) -> anyhow::Result<Vec<ChapterResult>> {
        Ok(vec![ChapterResult {
            title: format!("(stub chapter for {manhwa_url})"),
            url: "https://example.com/chapter/1".to_string(),
        }])
    }

    async fn chapter(&self, chapter_url: &str) -> anyhow::Result<ChapterImages> {
        Ok(ChapterImages {
            title: format!("(stub chapter for {chapter_url})"),
            pages: vec!["https://example.com/page1.jpg".to_string()],
        })
    }
}