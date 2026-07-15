use crate::{
    models::chapter::Chapter,
    models::chapter_summary::ChapterSummary,
    models::search_result::SearchResult,
    scraper::traits::Provider,
};

pub struct AsuraProvider;

#[async_trait::async_trait]
impl Provider for AsuraProvider {
    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        // TODO: implement search scraping for asurascans.
        // - build the search URL from `query`
        // - fetch + parse the results page
        // - return one SearchResult { title, url } per hit (url = manhwa page,
        //   passed into `list_chapters` below)
        let _ = query;
        todo!("implement AsuraProvider::search")
    }

    async fn list_chapters(&self, manhwa_url: &str) -> anyhow::Result<Vec<ChapterSummary>> {
        // TODO: implement chapter-listing scraping for asurascans.
        // - fetch `manhwa_url` (the manhwa's page, from a SearchResult.url)
        // - parse out the chapter list (title + url per chapter)
        // - return one ChapterSummary { title, url } per chapter (url = chapter
        //   page, passed into `chapter` below)
        let _ = manhwa_url;
        todo!("implement AsuraProvider::list_chapters")
    }

    async fn chapter(&self, url: &str) -> anyhow::Result<Chapter> {
        // TODO: implement chapter-page scraping for asurascans.
        // - fetch `url` (a chapter page, from a ChapterSummary.url)
        // - parse out the title + ordered image URLs
        // - return Chapter { title, pages }
        let _ = url;
        todo!("implement AsuraProvider::chapter")
    }
}
