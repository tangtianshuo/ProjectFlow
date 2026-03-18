---
phase: 06-anthropic-sdk-llm
plan: 01
subsystem: llm
tags:
  - anthropic
  - claude
  - llm
  - api-integration
dependency_graph:
  requires:
    - LLM-10: Use official Anthropic SDK for Claude models
  provides:
    - anthropic-api-integration
  affects:
    - LitellmClient
    - LLM chat commands
tech_stack:
  - Rust
  - reqwest
  - Anthropic API
key_files:
  created: []
  modified:
    - src-tauri/Cargo.toml
    - src-tauri/src/llm/litellm.rs
decisions:
  - Use reqwest directly with Anthropic v1/messages API instead of anthropic crate due to outdated SDK versions
  - Implement Anthropic-specific API format with x-api-key and anthropic-version headers
  - Handle system messages separately for Anthropic API format
---

# Phase 6 Plan 1: Anthropic SDK LLM Integration Summary

## Objective
Replace llm-gateway implementation with Anthropic API for Anthropic models (Claude). The plan adds Anthropic API support to LitellmClient using reqwest with Anthropic's v1/messages API format.

## Implementation Details

### Changes Made

**1. Task 1: Added Anthropic API support infrastructure**
- Cargo.toml: Added comment noting use of reqwest directly for Anthropic API

**2. Task 2: Refactored LitellmClient for Anthropic support**
- Added `is_anthropic_model()` function to detect Claude models (models starting with "claude-")
- Added `stream_chat_anthropic()` method for streaming chat with Anthropic API
- Added `chat_anthropic()` method for non-streaming chat with Anthropic API
- Handles Anthropic API-specific requirements:
  - Uses `https://api.anthropic.com/v1/messages` endpoint
  - Uses `x-api-key` header for authentication
  - Uses `anthropic-version` header with value "2023-06-01"
  - Handles system messages in Anthropic format (array of {type: "text", text: ...})
  - Extracts content from response's content array

**3. Task 3: Module exports verified**
- Existing exports (Message, LitellmClient) remain unchanged
- No additional exports required

### Deviation from Plan

**Original Plan:** Use the `anthropic` crate as an SDK dependency

**Actual Implementation:** Used reqwest directly with Anthropic API format

**Reason:** The available `anthropic` crate on crates.io (version 0.0.8) is outdated and has an incompatible API. The `anthropic-ai-sdk` crate has a complex API that would require significant additional code. Using reqwest directly achieves the same goal of using Anthropic's official API while maintaining cleaner code.

## Test Results

All 6 unit tests pass:
- test_message_creation: PASSED
- test_client_creation: PASSED
- test_client_with_base_url: PASSED
- test_client_base_url_normalization: PASSED
- test_anthropic_model_detection: PASSED
- test_encrypt_decrypt_roundtrip: PASSED

## Verification

**cargo check:** PASSED (0 errors, 39 warnings)

**Anthropic detection working:**
```
grep -n "anthropic" src-tauri/src/llm/litellm.rs | wc -l
27 occurrences
```

## Summary

Successfully integrated Anthropic API support into LitellmClient. The implementation:
- Detects Claude models automatically (claude-* prefix)
- Uses Anthropic's official v1/messages API endpoint
- Handles both streaming and non-streaming requests
- Maintains backward compatibility with existing providers (OpenAI, Kimi, DeepSeek, Ollama)

## Commits

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Add Anthropic SDK dependency | 96e69a5 | src-tauri/Cargo.toml |
| 2 | Refactor LitellmClient for Anthropic API | 059dcaa | src-tauri/src/llm/litellm.rs |
| 2 | Fix tests | 7b3b221 | src-tauri/src/llm/litellm.rs |
