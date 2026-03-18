# Technology Stack

**Project:** ProjectFlow - LLM Document Assistant
**Researched:** 2026-03-17
**Confidence:** HIGH

## Recommended Stack Additions

### Core LLM Integration

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **genai** | 0.5.3 | Multi-provider LLM client | Supports OpenAI, Anthropic, Gemini, Ollama, Groq, DeepSeek with unified API. Best for multi-model requirement. |
| **text-splitter** | 0.29.3 | Text chunking | Splits documents into semantic chunks for context. Essential for RAG/document analysis. |
| **tiktoken-rs** | 0.9.1 | Token counting | Accurate token estimation for LLM context limits. Used by many Rust AI projects. |

### API Key Security

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **keyring** | 3.6.3 | Credential storage | Cross-platform (Windows Credential Manager on Windows). 8.9M downloads, well-maintained. |
| **windows-sys** | (existing) | Windows API access | Already part of Windows ecosystem. Use for enhanced credential access if needed. |

### HTTP Client

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **reqwest** | Add as explicit dep | HTTP client | Already pulled transitively by Tauri/tokio. Add explicitly for type safety and explicit feature flags. |

## Existing Stack to Leverage

| Component | Current Version | LLM Integration Point |
|-----------|-----------------|----------------------|
| rusqlite | 0.31 | Store conversations, API keys (encrypted), chat history |
| tokio | 1 (full) | Already supports async LLM calls |
| serde | 1 | Serialize/deserialize LLM request/response |
| chrono | 0.4 | Timestamps for conversation history |

## What NOT to Add

| Crate | Why Avoid | Alternative |
|-------|-----------|-------------|
| langchain-rust | Less maintained (last update Oct 2024), fewer downloads than genai | Use **genai** for multi-provider support |
| llm (local models) | Requires GGML/GGUF model files, heavy for desktop | Use API-based LLM via **genai** |
| swiql | Overkill for simple document Q&A | Start with simple text-splitter + genai |

## Database Schema Additions

For LLM integration, add these tables to existing SQLite:

```sql
-- API Keys (encrypted storage)
CREATE TABLE llm_settings (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,  -- 'openai', 'anthropic', 'gemini', etc.
    api_key_encrypted TEXT NOT NULL,
    default_model TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Conversation history
CREATE TABLE llm_conversations (
    id TEXT PRIMARY KEY,
    project_id TEXT,
    title TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
);

-- Individual messages
CREATE TABLE llm_messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL,  -- 'user', 'assistant'
    content TEXT NOT NULL,
    tokens_used INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES llm_conversations(id) ON DELETE CASCADE
);

-- Document embeddings (for semantic search)
CREATE TABLE document_embeddings (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    chunk_text TEXT NOT NULL,
    embedding BLOB NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
```

## Installation

```bash
# Add to src-tauri/Cargo.toml

[dependencies]
# LLM clients
genai = "0.5.3"
text-splitter = "0.29"
tiktoken-rs = "0.9"

# Secure storage
keyring = "3"

# HTTP (explicit for type safety)
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
```

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Vue 3 Frontend                        │
│  (Chat UI, Document Editor, Settings Panel)                │
└────────────────────────┬────────────────────────────────────┘
                         │ invoke()
┌────────────────────────▼────────────────────────────────────┐
│                    Tauri Commands (Rust)                     │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │ LLM Service  │  │ Chat Service │  │ Embedding Service│   │
│  │  (genai)     │  │ (history)    │  │ (tiktoken+vector)│   │
│  └──────┬───────┘  └──────┬───────┘  └────────┬─────────┘   │
│         │                 │                    │              │
│  ┌──────▼─────────────────▼────────────────────▼─────────┐  │
│  │              Database Layer (rusqlite)                 │  │
│  │   documents | llm_settings | llm_conversations | etc.  │  │
│  └────────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │           Keyring (Windows Credential Manager)       │   │
│  │              API Key Encryption/Storage               │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
              ┌─────────────────────────┐
              │  External LLM APIs       │
              │  (OpenAI, Anthropic,     │
              │   Gemini, Ollama, etc.) │
              └─────────────────────────┘
```

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use **genai** over langchain-rust | Better multi-provider support, more active development, simpler API | Unified interface for OpenAI, Anthropic, Gemini, etc. |
| Use **keyring** for API keys | Native OS credential storage (Windows Credential Manager), not plaintext | Security requirement met |
| Keep LLM calls in Rust backend | Avoid exposing API keys to frontend, better control | Secure architecture |
| Use **text-splitter** for RAG | Handles both character and token-based splitting | Document analysis support |

## Sources

- crates.io search results (2026-03-17)
- genai repository: https://github.com/jeremychone/rust-genai
- keyring crate: https://github.com/open-source-coordinate/keyring-rs
- text-splitter: https://github.com/benbrandt/text-splitter
