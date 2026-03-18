---
phase: 03-llm
plan: '01'
subsystem: llm-backend
tags:
  - llm
  - openai
  - backend
  - rust
dependency_graph:
  requires:
    - src-tauri/src/db/mod.rs
  provides:
    - src-tauri/src/llm/mod.rs
    - src-tauri/src/llm/openai.rs
    - src-tauri/src/llm/prompts.rs
    - src-tauri/src/llm/storage.rs
    - src-tauri/src/commands/llm.rs
  affects:
    - src-tauri/src/lib.rs
    - src-tauri/Cargo.toml
tech_stack:
  added:
    - reqwest (with stream, rustls-tls)
    - bytes
    - futures-util
    - keyring (3)
    - aes-gcm (0.10)
    - base64 (0.22)
  patterns:
    - Tauri commands with async streaming
    - Windows Credential Manager via keyring
    - AES-GCM encryption for API key backup
    - SSE (Server-Sent Events) parsing for streaming
key_files:
  created:
    - src-tauri/src/llm/mod.rs
    - src-tauri/src/llm/openai.rs
    - src-tauri/src/llm/prompts.rs
    - src-tauri/src/llm/storage.rs
    - src-tauri/src/commands/llm.rs
  modified:
    - src-tauri/Cargo.toml
    - src-tauri/src/lib.rs
    - src-tauri/src/commands/mod.rs
decisions:
  - Use reqwest directly instead of genai crate for better control over streaming
  - Use keyring for OS-native credential storage (Windows Credential Manager)
  - Use AES-GCM for additional encryption layer before keyring storage
  - Implement streaming via SSE parsing rather than using higher-level libs
---

# Phase 3 Plan 1: LLM Backend Integration Summary

Implemented Rust backend for LLM integration with OpenAI API, including secure API key storage, streaming response handling, and project context injection.

## What Was Built

### 1. LLM Module (src-tauri/src/llm/)

- **openai.rs**: OpenAI API client using reqwest with streaming support
  - `Message` struct for chat messages
  - `OpenAIClient` with `stream_chat()` and `chat()` methods
  - SSE parsing for streaming token responses

- **prompts.rs**: Prompt templates with project context injection
  - `build_system_prompt(project, tasks)` - creates context-aware system prompt
  - Includes project name, description, status, dates, and task list
  - `build_document_prompt()` for document analysis

- **storage.rs**: Encrypted API key storage
  - `encrypt_api_key()` / `decrypt_api_key()` using AES-GCM
  - `store_api_key()` / `retrieve_api_key()` using Windows Credential Manager (keyring)
  - `delete_api_key()` / `has_api_key()` for key management

### 2. Tauri Commands (src-tauri/src/commands/llm.rs)

- `llm_save_key(model, api_key)` - Encrypts and stores API key
- `llm_get_key_status(model)` - Checks if key exists
- `llm_delete_key(model)` - Removes stored key
- `llm_chat(messages, project_id, model)` - Streams chat response
  - Gets project context if project_id provided
  - Emits `llm-token` event for each chunk
  - Emits `llm-done` on completion
  - Emits `llm-error` on failure
- `llm_get_models()` - Returns available models

### 3. Command Registration (src-tauri/src/lib.rs)

All LLM commands registered in Tauri invoke handler.

## Deviation from Original Plan

The original plan specified:
- `lgpt = "0.4"` - Changed to direct reqwest implementation due to package availability
- `tauri-plugin-store = "2"` - Changed to `keyring = "3"` for OS-native credential storage
- `genai = "0.5"` - Changed to direct reqwest due to API complexity

These changes provide better control and use more mature, well-maintained dependencies.

## Verification

- [x] cargo check passes for src-tauri
- [x] All new modules compile without errors
- [x] LLM commands registered in lib.rs
- [x] API key can be stored and retrieved encrypted (via keyring)
- [x] Streaming responses work via Tauri events (llm-token, llm-done, llm-error)

## Auth Gate

No authentication gates - this is a backend-only implementation. Users need to:
1. Get OpenAI API key from https://platform.openai.com/api-keys
2. Frontend will call `llm_save_key` to store it

## Files Modified

| File | Change |
|------|--------|
| src-tauri/Cargo.toml | Added reqwest, bytes, futures-util, keyring, aes-gcm, base64 |
| src-tauri/src/lib.rs | Added llm module, registered commands |
| src-tauri/src/commands/mod.rs | Added llm submodule |
| src-tauri/src/llm/* | New files |

## Commits

- 4c23330 chore(03-01): add LLM dependencies to Cargo.toml
- 22b0fe0 feat(03-01): implement Rust backend for LLM integration
