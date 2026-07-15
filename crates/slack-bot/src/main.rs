mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;
mod utils;

use std::sync::Arc;

use axum::{middleware::from_fn_with_state, Router};
use config::settings::Settings;
use engine::scraper::providers::ycbm::YCBMProvider;
use routes::slack_routes;
use services::engine_provider::EngineSearchProvider;
use services::slack_client::SlackClient;
use services::slack_service::SlackService;
use state::app_state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env")).ok();

    tracing_subscriber::fmt::init();

    let settings = Settings::from_env();
    let slack_client = SlackClient::new(reqwest::Client::new(), settings.slack_bot_token.clone());

    // `/manhwa chapter <url>` works today (YCBMProvider::chapter is
    // implemented). `search` and `chapters` will panic (todo!()) until
    // YCBMProvider::search / list_chapters are implemented.
    let state = AppState {
        slack_service: SlackService::new(
            Arc::new(EngineSearchProvider::new(YCBMProvider)),
            slack_client,
        ),
        settings,
    };

    let app = Router::new()
        .nest("/slack", slack_routes::router())
        .layer(from_fn_with_state(
            state.clone(),
            middleware::slack_signature::verify,
        ))
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    tracing::info!("slack-bot listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}