use async_trait::async_trait;
use engine::scraper::traits::Provider;

use crate::services::search_provider::{ChapterImages, ChapterResult, SearchProvider, SearchResult};

/// Wraps an `engine::scraper::traits::Provider` (e.g. `YCBMProvider`) so it
/// can be plugged in as slack-bot's `SearchProvider`. Just does type
/// translation — all real scraping logic lives in the engine crate.
pub struct EngineSearchProvider<P: Provider + Send + Sync> {
    provider: P,
}

impl<P: Provider + Send + Sync> EngineSearchProvider<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl<P: Provider + Send + Sync> SearchProvider for EngineSearchProvider<P> {
    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        let results = self.provider.search(query).await?;
        Ok(results
            .into_iter()
            .map(|r| SearchResult {
                title: r.title,
                url: r.url,
            })
            .collect())
    }

    async fn list_chapters(&self, manhwa_url: &str) -> anyhow::Result<Vec<ChapterResult>> {
        let chapters = self.provider.list_chapters(manhwa_url).await?;
        Ok(chapters
            .into_iter()
            .map(|c| ChapterResult {
                title: c.title,
                url: c.url,
            })
            .collect())
    }

    async fn chapter(&self, chapter_url: &str) -> anyhow::Result<ChapterImages> {
        let chapter = self.provider.chapter(chapter_url).await?;
        Ok(ChapterImages {
            title: chapter.title,
            pages: chapter.pages,
        })
    }
}
