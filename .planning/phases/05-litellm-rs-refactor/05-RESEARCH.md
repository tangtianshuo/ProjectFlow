# Phase 5: 使用 litellm_rs 重构 LLM 接入层 - Research

**Researched:** 2026-03-18
**Status:** Ready for planning

## Library Overview

**litellm_rs** 是一个 Rust 库，提供统一的接口来调用各种 LLM 提供商的 API。它支持：
- OpenAI (GPT-4, GPT-3.5)
- Anthropic (Claude)
- Azure OpenAI
- Ollama (本地模型)
- 任何 OpenAI 兼容的 API (如 Kimi, DeepSeek, MiniMax)

## Key Features

1. **统一接口** - 所有模型使用相同的 API 调用方式
2. **流式响应** - 支持 SSE (Server-Sent Events) 流式输出
3. **模型配置** - 灵活的配置选项
4. **错误处理** - 统一的错误类型

## Integration Approach

### Current Implementation (src-tauri/src/llm/)
- `openai.rs` - 自定义 OpenAI 客户端实现
- `storage.rs` - API key 存储
- `mod.rs` - 模块导出

### Migration Plan
1. 添加 `litellm_rs` 依赖到 `Cargo.toml`
2. 重构 `openai.rs` 使用 litellm_rs
3. 更新 `storage.rs` 适配新接口
4. 更新 `commands/llm.rs` 使用新的客户端

## Technical Details

Based on litellm_rs documentation pattern:
```rust
// 初始化客户端
let client = litellm_rs::Litellm::new("your-api-key");

// 发送聊天请求
let response = client.chat_completion("gpt-4", messages).await?;

// 流式响应
let stream = client.chat_completion_stream("gpt-4", messages).await?;
```

## Questions for Planning

1. 是否需要保留现有的 keyring 加密存储？
2. 是否需要支持所有 litellm_rs 支持的模型，还是只保留现有的？
3. 如何处理 model-specific 的配置 (如 temperature, max_tokens)？

---

*Phase: 05-litellm-rs-refactor*
*Research gathered: 2026-03-18*
