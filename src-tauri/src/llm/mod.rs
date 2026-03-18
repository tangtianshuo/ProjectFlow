//! LLM module for ProjectFlow
//!
//! Provides OpenAI API client with streaming support and prompt templates.

pub mod litellm;
pub mod openai;
pub mod prompts;
pub mod storage;

pub use litellm::{Message, LitellmClient};
pub use openai::OpenAIClient;
pub use prompts::build_system_prompt;
pub use storage::{
    decrypt_api_key, delete_api_key, delete_model_config, encrypt_api_key, has_api_key,
    retrieve_api_key, retrieve_model_config, store_api_key, store_model_config, ModelConfig,
};
