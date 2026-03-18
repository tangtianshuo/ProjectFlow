---
phase: 04-llm-kimi-deepseek-minimax-baseurl-apikey
plan: '01'
subsystem: llm-backend
tags: [llm, backend, configuration, chinese-models]
dependency_graph:
  requires: []
  provides:
    - OpenAIClient with configurable base_url
    - ModelConfig struct with storage functions
    - llm_save_model_config command
    - llm_get_model_config command
  affects:
    - src-tauri/src/llm/openai.rs
    - src-tauri/src/llm/storage.rs
    - src-tauri/src/commands/llm.rs
tech_stack:
  added:
    - ModelConfig struct for storing base_url and model_name
    - store_model_config() and retrieve_model_config() functions
    - llm_save_model_config and llm_get_model_config Tauri commands
  patterns:
    - Keyring storage with "_config" suffix for model configs
    - JSON serialization for model config storage
key_files:
  created: []
  modified:
    - src-tauri/src/llm/openai.rs
    - src-tauri/src/llm/storage.rs
    - src-tauri/src/llm/mod.rs
    - src-tauri/src/commands/llm.rs
decisions:
  - Use keyring service with "_config" suffix for model configuration storage
  - Serialize ModelConfig as JSON before AES-GCM encryption
metrics:
  duration: "~5 minutes"
  completed_date: "2026-03-18"
---

# Phase 04 Plan 01: Configurable base_url for Chinese LLM Providers Summary

## Overview

Extended the Rust backend to support configurable base_url and model_name for each LLM provider (Kimi, DeepSeek, MiniMax). This enables the OpenAI-compatible client to call different API endpoints.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Extend OpenAIClient with base_url parameter | 64293dd | src-tauri/src/llm/openai.rs, src-tauri/src/commands/llm.rs |
| 2 | Add model config storage functions | a39c9ab | src-tauri/src/llm/storage.rs, src-tauri/src/llm/mod.rs |
| 3 | Add Tauri commands for model config | eec0bae | src-tauri/src/commands/llm.rs |

## Changes Made

### Task 1: OpenAIClient base_url Support
- Added `base_url: String` field to `OpenAIClient` struct
- Updated `new()` to accept optional `base_url` parameter (defaults to "https://api.openai.com")
- Modified `stream_chat()` and `chat()` to use configured base_url

### Task 2: Model Config Storage
- Added `ModelConfig` struct with `base_url` and `model_name` fields
- Added `store_model_config()` for storing config in keyring
- Added `retrieve_model_config()` for retrieving config from keyring
- Added `delete_model_config()` for removing config
- Used "_config" suffix for config entries in keyring

### Task 3: Tauri Commands for Model Config
- Added `ModelConfigRequest` struct
- Added `llm_save_model_config` command (stores api_key + config)
- Added `llm_get_model_config` command (retrieves config without api_key)
- Updated `llm_chat` to retrieve base_url from model config
- Added Chinese LLM models to `llm_get_models`:
  - Kimi (Moonshot AI)
  - DeepSeek Chat/Coder
  - MiniMax

## Verification

Backend compiles successfully with all new functions:
- `OpenAIClient::new(api_key, model, base_url)` - now accepts base_url
- `llm::store_model_config()` / `llm::retrieve_model_config()` - storage functions
- `llm_save_model_config` / `llm_get_model_config` - Tauri commands

## Success Criteria Status

- [x] OpenAIClient accepts base_url parameter
- [x] Model config can be stored/retrieved via keyring
- [x] Tauri commands for config CRUD work
- [x] llm_chat uses configured base_url

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check: PASSED

All files modified exist and commits verified:
- 64293dd: feat(04-01): add base_url parameter to OpenAIClient
- a39c9ab: feat(04-01): add model config storage functions
- eec0bae: feat(04-01): add Tauri commands for model config
