use serde_json::{json, Value};

/// Flat list layout — e.g. search results or a favorites list. Sent as a
/// single reply via `SlackClient::respond`.
pub fn list_view(title: &str, items: &[(String, String)]) -> Value {
    if items.is_empty() {
        return json!({ "response_type": "ephemeral", "text": title });
    }

    let lines: Vec<String> = items
        .iter()
        .map(|(item_title, url)| format!("\u{2022} <{url}|{item_title}>"))
        .collect();

    json!({
        "response_type": "ephemeral",
        "text": format!("{title}\n{}", lines.join("\n")),
    })
}

/// One item, meant to be posted individually as a thread reply via
/// `SlackClient::post_message(channel, Some(parent_ts), ..)` — e.g. one
/// message per chapter under a "here are the chapters" parent message.
pub fn thread_item_view(title: &str, url: &str) -> Value {
    json!({
        "blocks": [{
            "type": "section",
            "text": { "type": "mrkdwn", "text": format!("*<{url}|{title}>*") }
        }]
    })
}

/// Slack caps a message at 50 blocks total, so leave headroom for the title
/// block on the first chunk.
const MAX_IMAGE_BLOCKS_PER_MESSAGE: usize = 45;

/// Chapter page images, chunked into one or more full message payloads (each
/// with up to `MAX_IMAGE_BLOCKS_PER_MESSAGE` image blocks). Send every
/// payload in order via `SlackClient::respond` (or `post_message`).
pub fn chapter_images_view(title: &str, image_urls: &[String]) -> Vec<Value> {
    if image_urls.is_empty() {
        return vec![json!({
            "response_type": "ephemeral",
            "text": format!("{title}\n(no pages found)"),
        })];
    }

    image_urls
        .chunks(MAX_IMAGE_BLOCKS_PER_MESSAGE)
        .enumerate()
        .map(|(chunk_index, chunk)| {
            let mut blocks = Vec::new();

            if chunk_index == 0 {
                blocks.push(json!({
                    "type": "section",
                    "text": { "type": "mrkdwn", "text": format!("*{title}*") }
                }));
            }

            for (i, url) in chunk.iter().enumerate() {
                let page_num = chunk_index * MAX_IMAGE_BLOCKS_PER_MESSAGE + i + 1;
                blocks.push(json!({
                    "type": "image",
                    "image_url": url,
                    "alt_text": format!("page {page_num}"),
                }));
            }

            json!({ "response_type": "ephemeral", "blocks": blocks })
        })
        .collect()
}