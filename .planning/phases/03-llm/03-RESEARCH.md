# Phase 3: LLM 文档助手 - Research

**Researched:** 2026-03-18
**Domain:** LLM Integration in Tauri Desktop Apps
**Confidence:** MEDIUM (based on training data, web search unavailable during research)

## Summary

This phase implements an AI assistant feature in ProjectFlow using LangChain Rust (`lgpt` crate) with OpenAI GPT models. The implementation requires: (1) Rust backend for LLM API calls with secure storage, (2) Vue 3 frontend with ChatGPT-like streaming conversation UI, (3) Tauri commands for AI interactions, and (4) automatic project context injection.

**Primary recommendation:** Use `lgpt` crate for LangChain abstractions with direct OpenAI API calls via `reqwest`, combined with `tauri-plugin-store` for encrypted API key storage. Implement streaming responses via Tauri events.

---

<user_constraints>

## User Constraints (from CONTEXT.md)

### Locked Decisions
- Use **LangChain Rust** framework
- Initial support **OpenAI GPT** model
- Sidebar menu item with expandable panel (not separate page)
- ChatGPT-like conversation UI
- Local encrypted file storage for API keys
- Auto-inject project context as system prompt

### Claude's Discretion
- Conversation UI details design
- Specific encryption algorithm selection
- Error handling logic
- Conversation history storage strategy

### Deferred Ideas (OUT OF SCOPE)
- Code assistance functionality - later phase
- Local model support (Ollama) - later phase
- Additional LLM models - later expansion

</user_constraints>

---

## Standard Stack

### Core Dependencies (Rust Backend)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `lgpt` | ^0.4 | LangChain Rust abstractions | LOCKED: User chose LangChain Rust |
| `reqwest` | ^0.12 | HTTP client for OpenAI API | Proven, async-native |
| `tokio` | ^1 | Async runtime | Already in project |
| `serde` | ^1 | Serialization | Already in project |
| `tauri-plugin-store` | ^2 | Encrypted key storage | Tauri official plugin |
| `aes-gcm` | ^0.10 | Encryption for API keys | Standard Rust crypto |

### Frontend Dependencies

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `@tauri-apps/api` | ^2 | Tauri invoke + events | Always |
| `marked` | ^17 | Markdown rendering | For AI response display |

### Alternative Approaches Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `lgpt` | Direct `reqwest` calls | Less abstraction, more control |
| `tauri-plugin-store` | Custom encrypted file | More code, same security |
| `aes-gcm` | `ring` crate | Both are standard, AES-GCM simpler |

---

## Architecture Patterns

### Recommended Project Structure

```
src-tauri/src/
├── commands/
│   ├── mod.rs           # Existing commands
│   └── llm.rs           # NEW: LLM commands
├── llm/                 # NEW: LLM module
│   ├── mod.rs           # LLM client setup
│   ├── openai.rs        # OpenAI API integration
│   └── prompts.rs       # Prompt templates
└── lib.rs               # Update with new commands

src/
├── components/
│   └── features/
│       └── llm/         # NEW: LLM UI components
│           ├── LlmPanel.vue
│           ├── ChatMessage.vue
│           └── LlmSettings.vue
├── stores/
│   └── llmStore.ts      # NEW: LLM state management
└── lib/
    └── api.ts           # Add LLM API functions
```

### Pattern 1: Streaming LLM Responses via Tauri Events

**What:** Send LLM tokens incrementally to frontend instead of waiting for complete response

**When to use:** ChatGPT-like experience with real-time token streaming

**Example:**
```rust
// Rust backend: emit events for each chunk
#[tauri::command]
async fn chat_stream(
    app: tauri::AppHandle,
    messages: Vec<Message>,
) -> Result<(), String> {
    let client = OpenAIClient::new();
    let mut stream = client.stream_chat(&messages).await?;

    while let Some(chunk) = stream.next().await {
        app.emit("llm-token", &chunk).map_err(|e| e.to_string())?;
    }
    app.emit("llm-done", ()).map_err(|e| e.to_string())?;
    Ok(())
}
```

```typescript
// Frontend: listen for tokens
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<string>('llm-token', (event) => {
  responseText.value += event.payload;
});
```

### Pattern 2: Secure API Key Storage

**What:** Encrypt API keys at rest using AES-GCM, decrypt only when making API calls

**When to use:** Storing sensitive LLM API keys locally

**Example:**
```rust
// Encrypt
use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::AesGcm;

fn encrypt_key(key: &str, encryption_key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(encryption_key).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, key.as_bytes()).unwrap();
    [nonce.to_vec(), ciphertext].concat()
}
```

### Pattern 3: Project Context Auto-Injection

**What:** Automatically include project metadata in system prompt

**When to use:** When user asks project-related questions

**Example:**
```rust
fn build_system_prompt(project: &Project, tasks: &[Task]) -> String {
    format!(
        "You are a project management assistant for ProjectFlow. \
        Current project: {}\nDescription: {}\n\
        Tasks: {}\n\
        Provide helpful, concise responses.",
        project.name,
        project.description.unwrap_or_default(),
        tasks.iter().map(|t| t.title.as_str()).collect::<Vec<_>>().join(", ")
    )
}
```

### Anti-Patterns to Avoid

- **Hardcoding API keys in source code:** Always use encrypted storage
- **Blocking async calls on main thread:** Use tokio async runtime (already configured)
- **Storing full conversation history in memory:** Persist to database with size limits
- **No error handling for API rate limits:** Implement retry with exponential backoff

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| HTTP client | Custom reqwest wrapper | Direct `reqwest` | Already sufficient for OpenAI API |
| Encryption | Implement AES yourself | `aes-gcm` crate | Battle-tested, audited |
| Key storage | Plain text file | `tauri-plugin-store` | OS-level encryption integration |
| Markdown parsing | Custom parser | `marked` (already in project) | Already installed |

---

## Common Pitfalls

### Pitfall 1: Streaming Response Without Backpressure
**What goes wrong:** Frontend receives tokens faster than it can render, causing memory issues
**Why it happens:** No flow control in event emission
**How to avoid:** Implement token batching (emit every 10-50 tokens, not every token)
**Warning signs:** Memory growth during long conversations, UI lag

### Pitfall 2: API Key Exposure in Logs
**What goes wrong:** API key appears in debug logs
**Why it happens:** Logging request headers without filtering secrets
**How to avoid:** Use `tracing` with sensitive data filtering, never log request headers
**Warning signs:** Long string values in debug output

### Pitfall 3: Conversation Context Window Overflow
**What goes wrong:** Too many messages cause API errors (context length exceeded)
**Why it happens:** No limit on conversation history
**How to avoid:** Implement sliding window (keep last N messages or tokens)
**Warning signs:** API errors with "maximum context length" message

### Pitfall 4: Missing Error Recovery
**What goes wrong:** Single API failure crashes entire conversation
**Why it happens:** No error boundaries in LLM command handlers
**How to avoid:** Wrap API calls in try-catch, emit error events to frontend
**Warning signs:** App becomes unresponsive after API error

---

## Code Examples

### OpenAI API Call with Streaming

```rust
// Source: Standard OpenAI API pattern
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

pub async fn chat_completion(
    client: &Client,
    api_key: &str,
    messages: &[Message],
) -> Result<String, reqwest::Error> {
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&ChatRequest {
            model: "gpt-4o".to_string(),
            messages: messages.to_vec(),
            stream: false,
        })
        .send()
        .await?;

    let result: ChatResponse = response.json().await?;
    Ok(result.choices[0].message.content.clone())
}
```

### Tauri Command with Streaming Events

```rust
// Source: Tauri v2 event system
#[tauri::command]
pub async fn llm_chat(
    app: tauri::AppHandle,
    db: State<'_, Database>,
    messages: Vec<Message>,
    project_id: Option<String>,
) -> Result<(), String> {
    // Get project context if provided
    let system_prompt = if let Some(pid) = project_id {
        if let Ok(Some(project)) = db.get_project(&pid) {
            let tasks = db.get_tasks_by_project(&pid).unwrap_or_default();
            build_system_prompt(&project, &tasks)
        } else {
            DEFAULT_SYSTEM_PROMPT.to_string()
        }
    } else {
        DEFAULT_SYSTEM_PROMPT.to_string()
    };

    // Add system prompt
    let mut full_messages = vec![Message {
        role: "system".to_string(),
        content: system_prompt,
    }];
    full_messages.extend(messages);

    // Stream response
    let client = OpenAIClient::new();
    match client.stream_chat(&full_messages).await {
        Ok(mut stream) => {
            while let Some(chunk) = stream.next().await {
                if let Ok(content) = chunk {
                    let _ = app.emit("llm-token", &content);
                }
            }
            let _ = app.emit("llm-done", ());
        }
        Err(e) => {
            let _ = app.emit("llm-error", &e.to_string());
        }
    }

    Ok(())
}
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Single prompt | LangChain chains with memory | 2023+ | Better conversation context |
| Blocking responses | Server-sent events / streaming | 2023+ | Real-time UX like ChatGPT |
| Plain text API keys | OS keychain / encrypted storage | 2022+ | Security compliance |
| Fixed models | Multi-model abstraction | 2024+ | Flexibility |

**Deprecated/outdated:**
- **Direct OpenAI SDK in Rust:** Use `reqwest` directly for more control
- **Blocking HTTP calls:** Use async `reqwest` with tokio (already in project)

---

## Open Questions

1. **Which encryption key to use for API key storage?**
   - What we know: User machine has OS keychain capabilities
   - What's unclear: Should we use OS keyring or derive key from user password?
   - Recommendation: Use `tauri-plugin-store` which handles OS-level encryption

2. **Conversation history persistence strategy?**
   - What we know: Messages stored in SQLite
   - What's unclear: How many messages to keep per conversation?
   - Recommendation: Keep last 50 messages, summarize older ones

3. **Rate limiting handling?**
   - What we know: OpenAI has rate limits
   - What's unclear: Should we implement client-side rate limiting?
   - Recommendation: Simple retry with exponential backoff, show error after 3 attempts

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Vitest (already in project) |
| Config file | vitest.config.ts |
| Quick run command | `npm run test` |
| Full suite command | `npm run test:coverage` |

### Phase Requirements (TBD - Need REQUIREMENTS.md update)

*Note: The main requirements are documented in CONTEXT.md but should be added to REQUIREMENTS.md with IDs like LLM-01, LLM-02, etc.*

### Wave 0 Gaps
- [ ] `tests/llmStore.test.ts` - covers LLM state management
- [ ] `tests/api/llm.test.ts` - covers LLM API calls
- [ ] Framework already installed: Vitest

---

## Sources

### Primary (HIGH confidence)
- Tauri v2 official documentation - commands, events, plugins
- Project existing code structure - patterns to follow

### Secondary (MEDIUM confidence)
- lgpt crate documentation (Rust LangChain implementation)
- OpenAI API documentation for chat completions

### Tertiary (LOW confidence - needs validation)
- Encryption algorithm specifics (AES-GCM vs ChaCha20Poly1305)
- Streaming implementation details

---

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM - Training data + project structure, web search unavailable
- Architecture: MEDIUM - Established patterns, specific implementation TBD
- Pitfalls: MEDIUM - Known issues from similar implementations

**Research date:** 2026-03-18
**Valid until:** 2026-04-18 (30 days - stable technology)

---

## Phase Requirements

*Requirements from this research to be added to REQUIREMENTS.md:*

| ID | Description | Research Support |
|----|-------------|-----------------|
| LLM-01 | LLM panel UI in sidebar | Use Vue 3 components with streaming events |
| LLM-02 | OpenAI API integration | lgpt + reqwest for API calls |
| LLM-03 | Encrypted API key storage | tauri-plugin-store with OS encryption |
| LLM-04 | Project context auto-injection | Build system prompt from project data |
| LLM-05 | Streaming response UI | Tauri events for token-by-token updates |
| LLM-06 | Conversation history | SQLite storage with size limits |
