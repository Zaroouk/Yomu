use regex::Regex;
use serde::{Deserialize,Serialize};

pub fn chapter_images(url:&str) -> ChapterPage {
let expression = r"https:\/\/cdn\.asurascans\.com\/asura-images\/chapters\/(?P<manwha>[^/]+)\/(?P<chapter>\d+)\/(?P<page>\d+)\.(?P<ext>\w+)\?v=(?P<version>\d+)";
let rx = Regex::new(expression).unwrap();

let mut page = ChapterPage::default();
if let Some(caps) = rx.captures(url) {
    // println!("Manhwa: {}", &caps["manwha"]);
    // println!("Chapter: {}", &caps["chapter"]);
    // println!("Page: {}", &caps["page"]);
    // println!("Version: {}", &caps["version"]);

    page.manwha = String::from(&caps["manwha"]);
    page.chapter = String::from(&caps["chapter"]);
    page.page = String::from(&caps["page"]);
    page.version = String::from(&caps["version"]);
}

page
}
#[derive(Debug, Default,Serialize,Deserialize, Clone)]
pub struct ChapterPage {
    manwha: String,
    chapter: String,
    page: String,
    version: String,
}
