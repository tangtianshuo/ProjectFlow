# Phase 4: 国产LLM模型支持 - Research

**Researched:** 2026-03-18
**Domain:** Chinese LLM integration (Kimi, DeepSeek, MiniMax) with OpenAI-compatible APIs
**Confidence:** MEDIUM

## Summary

This phase extends the existing LLM assistant to support Chinese large language models (Kimi, DeepSeek, MiniMax). The implementation leverages the existing OpenAI-compatible API structure already in place (reqwest-based client), with modifications to support configurable base URLs per model.

**Primary recommendation:** Modify the existing `OpenAIClient` to accept a configurable `base_url` parameter, extend storage to save base_url per model, and add UI for model selection. No new crates required - existing reqwest implementation is sufficient.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Support Kimi (moonshot.ai), DeepSeek (deepseek.com), MiniMax (minimax.io)
- Each model has independent configuration: base_url, api_key, model
- Use OpenAI-compatible interface (openai-compat or similar crate)
- UI dropdown for model selection
- Streaming response support

### Claude's Discretion
- Specific UI detail design
- Error handling logic
- Default baseurl for each model

### Deferred Ideas (OUT OF SCOPE)
- Claude and other international models - future phase
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| LLM-07 | Support Chinese LLM models (Kimi, DeepSeek, MiniMax) | API endpoints identified, OpenAI-compatible format confirmed |
| LLM-08 | Configurable base_url per model | Existing client architecture supports this modification |
| LLM-09 | Model selection UI | Existing LlmSettings.vue can be extended |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| reqwest | 0.12 | HTTP client for API calls | Already in use, supports streaming |
| keyring | 3 | Secure API key storage | Already in use |
| aes-gcm | 0.10 | Additional encryption layer | Already in use |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| futures-util | 0.3 | Stream handling | Already in use for SSE parsing |

### No New Crates Required
The existing reqwest-based implementation is sufficient. The openai-compat crate mentioned in decisions is optional - direct reqwest usage provides better control and avoids additional dependencies.

## Architecture Patterns

### Recommended Project Structure
```
src-tauri/src/
├── llm/
│   ├── mod.rs        # Already exports needed functions
│   ├── openai.rs     # MODIFY: Add base_url support
│   ├── storage.rs    # EXTEND: Store base_url per model
│   └── prompts.rs    # Keep as-is
└── commands/
    └── llm.rs        # EXTEND: Add model config commands

src/
├── stores/
│   └── llmStore.ts   # EXTEND: Add model config state
├── lib/
│   └── api.ts        # EXTEND: Add config API calls
└── components/
    └── features/
        └── llm/
            └── LlmSettings.vue  # EXTEND: Model selection UI
```

### Pattern: Configurable Base URL Client
**What:** Extend OpenAIClient to accept custom base_url
**When to use:** When connecting to any OpenAI-compatible API
**Example:**
```rust
// Source: Modified from existing openai.rs
pub struct OpenAIClient {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,  // NEW: configurable base URL
}

impl OpenAIClient {
    pub fn new(api_key: String, model: Option<String>, base_url: Option<String>) -> Self {
        let client = Client::new();
        let model = model.unwrap_or_else(|| "gpt-4o".to_string());
        let base_url = base_url.unwrap_or_else(|| "https://api.openai.com".to_string());
        Self {
            client,
            api_key,
            model,
            base_url,
        }
    }

    pub async fn stream_chat(&self, messages: Vec<Message>) -> Result<...> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        // ... rest of implementation
    }
}
```

### Pattern: Model Configuration Storage
**What:** Store model configuration (base_url, model_name) alongside api_key
**When to use:** When supporting multiple LLM providers with different endpoints
**Example:**
```rust
// Extend storage.rs to store model config
#[derive(Serialize, Deserialize)]
pub struct ModelConfig {
    pub base_url: String,
    pub model_name: String,
}

pub fn store_model_config(model_id: &str, config: &ModelConfig) -> Result<(), String> { ... }
pub fn retrieve_model_config(model_id: &str) -> Result<Option<ModelConfig>, String> { ... }
```

### Anti-Patterns to Avoid
- **Hardcoding API endpoints:** Each model should have configurable base_url
- **Single API key storage:** Support separate keys per model (already implemented)
- **Blocking on API calls:** Use async/streaming as already implemented

## Common Pitfalls

### Pitfall 1: Incorrect API Endpoints
**What goes wrong:** Using wrong base URL causes all API calls to fail
**Why it happens:** Each provider has different endpoint patterns
**How to avoid:** Use correct default endpoints:
- Kimi: `https://api.moonshot.cn`
- DeepSeek: `https://api.deepseek.com`
- MiniMax: `https://api.minimax.chat`
**Warning signs:** "404 Not Found" or "invalid endpoint" errors

### Pitfall 2: Model Name Mismatch
**What goes wrong:** API rejects request due to invalid model name
**Why it happens:** Each provider uses different model identifiers
**How to avoid:** Document correct model names:
- Kimi: `moonshot-v1-8k`, `moonshot-v1-32k`, etc.
- DeepSeek: `deepseek-chat`, `deepseek-coder`
- MiniMax: `abab6.5s-chat`, `abab6.5g-chat`

### Pitfall 3: Streaming Format Differences
**What goes wrong:** SSE parsing fails for some providers
**Why it happens:** Some providers may have slight format variations
**How to avoid:** The existing SSE parser is robust - should work with all providers

## Code Examples

### Default Model Configurations
```typescript
// Frontend: Default configurations for Chinese models
export const DEFAULT_MODEL_CONFIGS = {
  'kimi': {
    name: 'Kimi',
    base_url: 'https://api.moonshot.cn',
    default_model: 'moonshot-v1-8k',
  },
  'deepseek': {
    name: 'DeepSeek',
    base_url: 'https://api.deepseek.com',
    default_model: 'deepseek-chat',
  },
  'minimax': {
    name: 'MiniMax',
    base_url: 'https://api.minimax.chat',
    default_model: 'abab6.5s-chat',
  },
};
```

### Rust Command Extension
```rust
// Source: Extend existing commands/llm.rs
#[derive(Debug, Deserialize)]
pub struct ModelConfigRequest {
    pub model_id: String,
    pub base_url: String,
    pub model_name: String,
    pub api_key: String,
}

#[tauri::command]
pub fn llm_save_model_config(config: ModelConfigRequest) -> Result<(), String> {
    // Store both API key and config
    llm::store_api_key(&config.model_id, &config.api_key)?;
    llm::store_model_config(&config.model_id, &ModelConfig {
        base_url: config.base_url,
        model_name: config.model_name,
    })
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Hardcoded OpenAI endpoint | Configurable base_url per model | Phase 4 | Enables multi-provider support |
| Single model selection | Multi-model dropdown with config | Phase 4 | Better UX for Chinese models |

**Deprecated/outdated:**
- None specific to this phase

## Open Questions

1. **MiniMax API Version**
   - What we know: MiniMax uses `/v1/text/chatcompletion_v2` (different from standard `/v1/chat/completions`)
   - What's unclear: Whether v2 endpoint is fully compatible with existing SSE parsing
   - Recommendation: Test with MiniMax after implementation, adjust if needed

2. **API Key Storage Keyring Entry**
   - What we know: Currently uses model name as keyring entry (e.g., "gpt-4o")
   - What's unclear: Should Chinese models use unique identifiers (e.g., "kimi", "deepseek")
   - Recommendation: Use consistent model IDs matching frontend model list

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Vitest (existing) |
| Config file | vitest.config.ts |
| Quick run command | `npm run test` |
| Full suite command | `npm run test:coverage` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| LLM-07 | Chinese LLM API calls work | unit/integration | `npm run test -- llm` | Yes - existing tests |
| LLM-08 | Configurable base_url | unit | Test client with custom URL | Need new test |
| LLM-09 | Model selection UI | e2e | Manual testing | N/A |

### Sampling Rate
- **Per task commit:** `npm run test -- llmStore`
- **Per wave merge:** `npm run test`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `tests/llm/client.test.ts` - tests for configurable base_url
- [ ] `tests/llm/storage.test.ts` - tests for model config storage

## Sources

### Primary (HIGH confidence)
- Existing implementation: `src-tauri/src/llm/openai.rs` - confirmed reqwest streaming approach
- Existing implementation: `src-tauri/src/llm/storage.rs` - confirmed keyring usage
- Existing implementation: `src-tauri/src/commands/llm.rs` - confirmed Tauri command pattern

### Secondary (MEDIUM confidence)
- OpenAI API format documentation (general knowledge) - SSE streaming format is standard
- Kimi API: `https://api.moonshot.cn/v1/chat/completions`
- DeepSeek API: `https://api.deepseek.com/v1/chat/completions`
- MiniMax API: `https://api.minimax.chat/v1/text/chatcompletion_v2`

### Tertiary (LOW confidence)
- Model names may vary - recommend adding validation or documentation

## Metadata

**Confidence breakdown:**
- Standard Stack: HIGH - using existing reqwest implementation
- Architecture: HIGH - extends existing patterns
- Pitfalls: MEDIUM - based on general knowledge of APIs, not verified against current docs

**Research date:** 2026-03-18
**Valid until:** 2026-04-18 (30 days for stable API)
