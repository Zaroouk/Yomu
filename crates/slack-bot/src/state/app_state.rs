use crate::config::settings::Settings;
use crate::services::slack_service::SlackService;

#[derive(Clone)]
pub struct AppState {
    pub slack_service: SlackService,
    pub settings: Settings,
}