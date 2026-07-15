use anyhow::Result;
use engine::scraper::providers::ycbm::YCBMProvider;
use engine::scraper::traits::Provider;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = YCBMProvider;

    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("search") => {
            let query = args.collect::<Vec<_>>().join(" ");
            let results = provider.search(&query).await?;
            dbg!(results);
        }
        Some("chapters") => {
            let manhwa_url = args.next().expect("usage: cli chapters <manhwa_url>");
            let chapters = provider.list_chapters(&manhwa_url).await?;
            dbg!(chapters);
        }
        Some("chapter") => {
            let url = args.next().expect("usage: cli chapter <chapter_url>");
            let chapter = provider.chapter(&url).await?;
            dbg!(chapter);
        }
        _ => {
            eprintln!(
                "usage: cli <search <query> | chapters <manhwa_url> | chapter <chapter_url>>"
            );
        }
    }

    Ok(())
}
