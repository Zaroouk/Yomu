

use crate::{
    scraper::traits::Provider,
    models::chapter::Chapter,
    models::chapter_summary::ChapterSummary,
    models::search_result::SearchResult,
    utils::regex,
};
use reqwest;
pub struct YCBMProvider;

use scraper::{
    Html,
    Selector
};

#[async_trait::async_trait]
impl Provider for YCBMProvider {
    async fn chapter(&self, url: &str) -> anyhow::Result<Chapter> {
        let client = reqwest::Client::new();


        let html = client.get(url).send().await?.text().await?;

        let document = Html::parse_document(&html);

        // --- Title ---
        // Adjust this selector to match the actual page (e.g. "h1.chapter-title")
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        // --- Images ---
        // Adjust this selector to match the actual image containers
        // (e.g. "div.reading-content img" or "img.page-image")
        let image_selector = Selector::parse("img").unwrap();
        let mut images: Vec<String> = Vec::new();

        for image in document.select(&image_selector) {
            // Some sites lazy-load and put the real URL in data-src instead of src
            if let Some(src) = image
                .value()
                .attr("src")
                .or_else(|| image.value().attr("data-src"))
            {
                images.push(src.to_string());
                println!("LOOP {:?}",src);
                let s = regex::chapter_images(src);
                println!("{:?}",s);
            }
        }

        Ok(Chapter { title, pages:images })
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        // TODO: implement search scraping for this provider.
        // - build the provider's search URL from `query`
        // - fetch + parse the results page (see `chapter` above for the
        //   reqwest + scraper::Html pattern)
        // - return one SearchResult { title, url } per hit (url = manhwa page,
        //   passed into `list_chapters` below)
        let _ = query;
        todo!("implement YCBMProvider::search")
    }

    async fn list_chapters(&self, manhwa_url: &str) -> anyhow::Result<Vec<ChapterSummary>> {
        // TODO: implement chapter-listing scraping for this provider.
        // - fetch `manhwa_url` (the manhwa's page, from a SearchResult.url)
        // - parse out the chapter list (title + url per chapter)
        // - return one ChapterSummary { title, url } per chapter (url = chapter
        //   page, passed into `chapter` above)
        let _ = manhwa_url;
        todo!("implement YCBMProvider::list_chapters")
    }
}