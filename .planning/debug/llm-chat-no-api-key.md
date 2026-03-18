---
status: investigating
trigger: "llm-chat-no-api-key"
created: 2026-03-18T00:00:00.000Z
updated: 2026-03-18T00:00:00.000Z
---

## Current Focus
hypothesis: Model mismatch - API key stored with "minimax" but chat defaults to "gpt-4o" after app restart
test: Run the app and check Tauri console logs
expecting: See logs showing which model ID is being used for API key retrieval
next_action: User needs to test with the added logging to confirm hypothesis

## Symptoms
expected: Chat should work with configured API key
actual: Returns "No API key found" error
errors: No API key found
reproduction: Configured API key via UI, then sent chat message
started: Recent - after phase 04 implementation

## Eliminated

## Evidence

- timestamp: 2026-03-18T00:00:00.000Z
  checked: Added logging to commands/llm.rs
  found: Logs model name used, checks for keys at "minimax", "gpt-4o", "kimi", and model configs
  implication: Will reveal if API key is stored under different model ID

- timestamp: 2026-03-18T00:00:00.000Z
  checked: Added logging to llm/litellm.rs
  found: Logs URL, headers, body before making HTTP request
  implication: Will show actual HTTP request details

- timestamp: 2026-03-18T00:00:00.000Z
  checked: Frontend store and settings
  found: Store has selectedModelId defaulting to "gpt-4o", which resets on app restart. LlmSettings.vue updates store after saving but no persistence.
  implication: If user restarts app after configuring MiniMax, selectedModelId resets to "gpt-4o"

## Resolution
root_cause:
fix:
verification:
files_changed: []
