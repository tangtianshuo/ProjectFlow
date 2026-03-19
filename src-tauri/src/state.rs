use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub api_url: String,
    pub model: String,
}

pub struct AppState {
    pub llm_config: Mutex<LlmConfig>,
    pub http_client: Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            llm_config: Mutex::new(LlmConfig::default()),
            http_client: Client::new(),
        }
    }

    pub fn update_config(&self, provider: &str, api_key: &str, api_url: &str, model: &str) {
        let mut config = self.llm_config.lock().unwrap();
        config.provider = provider.to_string();
        config.api_key = api_key.to_string();
        config.api_url = api_url.to_string();
        config.model = model.to_string();
    }

    pub fn get_config(&self) -> LlmConfig {
        self.llm_config.lock().unwrap().clone()
    }
}
