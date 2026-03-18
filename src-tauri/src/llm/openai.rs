//! OpenAI API client with streaming support
//!
//! Uses reqwest directly to call OpenAI's API for more control.

use futures_util::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Message struct for LLM conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,  // "user", "assistant", "system"
    pub content: String,
}

/// OpenAI API client with streaming support
pub struct OpenAIClient {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAIClient {
    /// Create a new OpenAI client
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let client = Client::new();
        let model = model.unwrap_or_else(|| "gpt-4o".to_string());
        Self {
            client,
            api_key,
            model,
        }
    }

    /// Stream chat completion response
    ///
    /// Returns a stream of response chunks
    pub async fn stream_chat(
        &self,
        messages: Vec<Message>,
    ) -> Result<impl futures_util::stream::Stream<Item = Result<String, String>> + Send, String> {
        let url = format!(
            "https://api.openai.com/v1/chat/completions"
        );

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true,
        });

        let request = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let byte_stream = request.bytes_stream();

        let stream = byte_stream.map(|chunk_result: Result<bytes::Bytes, reqwest::Error>| {
            match chunk_result {
                Ok(bytes) => {
                    // Parse SSE response
                    let text = String::from_utf8_lossy(&bytes);
                    let mut content = String::new();

                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" {
                                continue;
                            }
                            // Try to parse JSON and extract content
                            if let Ok(resp) = serde_json::from_str::<serde_json::Value>(data) {
                                if let Some(choices) = resp.get("choices").and_then(|c| c.as_array()) {
                                    for choice in choices {
                                        if let Some(delta) = choice.get("delta").and_then(|d| d.get("content")) {
                                            if let Some(c) = delta.as_str() {
                                                content.push_str(c);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if content.is_empty() {
                        Ok(String::new())
                    } else {
                        Ok(content)
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        });

        Ok(stream)
    }

    /// Non-streaming chat completion
    pub async fn chat(&self, messages: Vec<Message>) -> Result<String, String> {
        let url = format!(
            "https://api.openai.com/v1/chat/completions"
        );

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": false,
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        if let Some(content) = json
            .get("choices")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
        {
            Ok(content.to_string())
        } else {
            Err("Failed to parse response".to_string())
        }
    }
}
