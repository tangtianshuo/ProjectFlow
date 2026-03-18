//! Unified LLM client for multiple providers
//!
//! Provides a unified API for multiple LLM providers (OpenAI, Anthropic, Azure,
//! Ollama, and OpenAI-compatible APIs like Kimi/DeepSeek/MiniMax).
//!
//! Uses llm-gateway crate for Chinese providers (Kimi, DeepSeek) and reqwest
//! for standard providers (OpenAI, Anthropic, Ollama).

use futures_util::stream::{BoxStream, StreamExt};
use llm_gateway::types::ChatRequest;
use llm_gateway::Client as GatewayClient;
use llm_gateway::Config;
use reqwest::Client;

// Re-export Message from openai module to ensure type consistency
pub use crate::llm::openai::Message;

/// Unified LLM client supporting multiple providers
///
/// This client works with any OpenAI-compatible API endpoint, making it
/// compatible with a wide range of LLM providers.
///
/// Uses llm-gateway for Chinese providers (Kimi, DeepSeek) and reqwest for
/// standard providers (OpenAI, Anthropic, Ollama).
pub struct LitellmClient {
    /// HTTP client for making requests (used for standard providers)
    client: Client,
    /// Gateway client for Chinese providers (Kimi, DeepSeek)
    gateway_client: Option<GatewayClient>,
    /// API key for authentication
    api_key: String,
    /// Model name to use
    model: String,
    /// Base URL for the API (e.g., "https://api.openai.com", "https://api.moonshot.cn/v1")
    base_url: String,
    /// Provider identifier to detect Chinese providers
    provider_id: String,
}

/// Detect if a model/provider uses Chinese LLM services
fn is_chinese_provider(model: &str, base_url: &str) -> bool {
    let model_lower = model.to_lowercase();
    let url_lower = base_url.to_lowercase();

    // Kimi (moonshot.ai)
    model_lower.contains("moonshot") || url_lower.contains("moonshot.cn")
    // DeepSeek
    || model_lower.contains("deepseek") || url_lower.contains("deepseek.com")
}

impl LitellmClient {
    /// Create a new LitellmClient
    ///
    /// # Arguments
    /// * `api_key` - The API key for authentication
    /// * `model` - The model name (e.g., "gpt-4o", "claude-3-opus", "moonshot-v1-8k")
    /// * `base_url` - Optional custom base URL for OpenAI-compatible APIs
    ///
    /// # Example
    /// ```rust
    /// // OpenAI
    /// let client = LitellmClient::new(
    ///     "sk-xxx".to_string(),
    ///     "gpt-4o".to_string(),
    ///     None  // Uses default https://api.openai.com
    /// );
    ///
    /// // OpenAI-compatible API (Kimi/DeepSeek/MiniMax)
    /// let client = LitellmClient::new(
    ///     "sk-xxx".to_string(),
    ///     "moonshot-v1-8k".to_string(),
    ///     Some("https://api.moonshot.cn/v1".to_string())
    /// );
    ///
    /// // Ollama (local)
    /// let client = LitellmClient::new(
    ///     "".to_string(),  // No API key needed for local
    ///     "llama2".to_string(),
    ///     Some("http://localhost:11434".to_string())
    /// );
    /// ```
    pub fn new(api_key: String, model: String, base_url: Option<String>) -> Self {
        let client = Client::new();
        let base_url = base_url.unwrap_or_else(|| "https://api.openai.com".to_string());

        // Ensure base_url has the proper path for chat completions
        let base_url = if base_url.ends_with("/v1") || base_url.contains("/v1/") {
            base_url.clone()
        } else if base_url.ends_with("/") {
            format!("{}v1", base_url)
        } else {
            format!("{}/v1", base_url)
        };

        // Detect if this is a Chinese provider that should use llm-gateway
        let provider_id = if base_url.contains("moonshot.cn") {
            "kimi".to_string()
        } else if base_url.contains("deepseek.com") {
            "deepseek".to_string()
        } else {
            "".to_string()
        };

        // Initialize llm-gateway client for Chinese providers
        let gateway_client = if is_chinese_provider(&model, &base_url) {
            // Create Config with provider-specific API key and base_url
            let mut config = Config::default();
            if provider_id == "kimi" {
                config.kimi = llm_gateway::ProviderConfig {
                    base_url: Some(base_url.clone()),
                    api_key: Some(api_key.clone()),
                    ..Default::default()
                };
            } else if provider_id == "deepseek" {
                config.deepseek = llm_gateway::ProviderConfig {
                    base_url: Some(base_url.clone()),
                    api_key: Some(api_key.clone()),
                    ..Default::default()
                };
            }
            Some(GatewayClient::with_config(config))
        } else {
            None
        };

        Self {
            client,
            gateway_client,
            api_key,
            model,
            base_url,
            provider_id,
        }
    }

    /// Stream chat completion response
    ///
    /// Returns a stream of response content chunks
    pub async fn stream_chat(
        &self,
        messages: Vec<Message>,
    ) -> Result<futures_util::stream::BoxStream<'static, Result<String, String>>, String> {
        // Use llm-gateway for Chinese providers (Kimi, DeepSeek)
        if let Some(ref gateway_client) = self.gateway_client {
            // Convert messages to (role, content) tuples for llm-gateway
            let gateway_messages: Vec<(String, String)> = messages
                .into_iter()
                .map(|m| (m.role, m.content))
                .collect();

            let request = ChatRequest {
                model: self.model.clone(),
                messages: gateway_messages,
            };

            let stream = gateway_client
                .chat_stream(request)
                .await
                .map_err(|e| e.to_string())?;

            // Convert llm-gateway stream to our stream type
            // The gateway returns BoxStream<Result<String, LlmProxyError>>
            let stream: BoxStream<'static, Result<String, String>> = stream
                .map(|result| result.map_err(|e| e.to_string()))
                .boxed();

            return Ok(stream);
        }

        // Fall back to reqwest for standard providers (OpenAI, Anthropic, Ollama)
        let url = format!("{}/chat/completions", self.base_url);

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": true,
        });

        let request = self
            .client
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

        let stream: BoxStream<'static, Result<String, String>> = stream.boxed();
        Ok(stream)
    }

    /// Non-streaming chat completion
    ///
    /// Returns the complete response as a string
    pub async fn chat(&self, messages: Vec<Message>) -> Result<String, String> {
        // Use llm-gateway for Chinese providers (Kimi, DeepSeek)
        if let Some(ref gateway_client) = self.gateway_client {
            // Convert messages to (role, content) tuples for llm-gateway
            let gateway_messages: Vec<(String, String)> = messages
                .into_iter()
                .map(|m| (m.role, m.content))
                .collect();

            let request = ChatRequest {
                model: self.model.clone(),
                messages: gateway_messages,
            };

            let response = gateway_client
                .chat(request)
                .await
                .map_err(|e| e.to_string())?;

            return Ok(response.text);
        }

        // Fall back to reqwest for standard providers (OpenAI, Anthropic, Ollama)
        let url = format!("{}/chat/completions", self.base_url);

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": false,
        });

        let response = self
            .client
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

    /// Get the model name
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_client_creation() {
        let client = LitellmClient::new(
            "sk-test".to_string(),
            "gpt-4o".to_string(),
            None,
        );
        assert_eq!(client.api_key, "sk-test");
        assert_eq!(client.model, "gpt-4o");
        assert!(client.base_url.contains("api.openai.com"));
    }

    #[test]
    fn test_client_with_base_url() {
        let client = LitellmClient::new(
            "sk-test".to_string(),
            "moonshot-v1-8k".to_string(),
            Some("https://api.moonshot.cn/v1".to_string()),
        );
        assert!(client.base_url.contains("api.moonshot.cn"));
    }

    #[test]
    fn test_client_base_url_normalization() {
        // Test that base_url is properly normalized
        let client = LitellmClient::new(
            "sk-test".to_string(),
            "llama2".to_string(),
            Some("http://localhost:11434".to_string()),
        );
        assert!(client.base_url.contains("localhost:11434"));
        assert!(client.base_url.contains("/v1/chat/completions"));
    }
}
