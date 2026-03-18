---
phase: 04-llm-kimi-deepseek-minimax-baseurl-apikey
plan: '02'
subsystem: llm-frontend
tags: [llm, frontend, configuration, chinese-models]
dependency_graph:
  requires:
    - 04-01 (backend model config storage)
  provides:
    - Model config state in llmStore
    - API methods for model config CRUD
    - Settings UI for Chinese LLM providers
  affects:
    - src/stores/llmStore.ts
    - src/lib/api.ts
    - src/components/features/llm/LlmSettings.vue
tech_stack:
  added:
    - ModelConfig interface in api.ts
    - DEFAULT_MODEL_CONFIGS with Kimi, DeepSeek, MiniMax defaults
    - saveModelConfig and getModelConfig actions
    - Base URL and Model Name input fields in settings UI
  patterns:
    - Vue computed properties for model list
    - Config pre-filled with provider defaults
key_files:
  created: []
  modified:
    - src/stores/llmStore.ts
    - src/lib/api.ts
    - src/components/features/llm/LlmSettings.vue
decisions:
  - Use DEFAULT_MODEL_CONFIGS for both model list and default values
  - Store selectedModelId in llmStore for current model selection
metrics:
  duration: "~3 minutes"
  completed_date: "2026-03-18"
---

# Phase 04 Plan 02: Frontend Model Selection UI Summary

## Overview

Extended the Vue frontend to support model selection and configuration for Chinese LLM providers (Kimi, DeepSeek, MiniMax). Users can now select from all available providers and configure base_url and model_name for each.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Extend llmStore with model config state | 5c1d8ad | src/stores/llmStore.ts |
| 2 | Add model config API methods | 48d69fc | src/lib/api.ts |
| 3 | Update LlmSettings.vue with config UI | ca90928 | src/components/features/llm/LlmSettings.vue |

## Changes Made

### Task 1: llmStore Model Config State
- Added `modelConfigs` ref to store config for each model
- Added `selectedModelId` to track current selection
- Added `saveModelConfig(modelId, config)` action
- Added `getModelConfig(modelId)` action
- Added `loadAllModelConfigs()` action
- Added `DEFAULT_MODEL_CONFIGS` with provider defaults:
  - Kimi: base_url=https://api.moonshot.cn, model=moonshot-v1-8k
  - DeepSeek: base_url=https://api.deepseek.com, model=deepseek-chat
  - MiniMax: base_url=https://api.minimax.chat, model=abab6.5s-chat
- Updated `sendMessage` to use `selectedModelId` for config lookup

### Task 2: Model Config API Methods
- Added `ModelConfig` interface with model_id, base_url, model_name, api_key
- Added `saveModelConfig(config)` to call llm_save_model_config
- Added `getModelConfig(modelId)` to call llm_get_model_config

### Task 3: LlmSettings.vue Config UI
- Replaced static models array with `DEFAULT_MODEL_CONFIGS`
- Added base_url input field (pre-filled with defaults)
- Added model_name input field (pre-filled with defaults)
- Updated saveKey to call saveModelConfig with all fields
- Model dropdown shows provider name + model name
- Shows configured/not configured status per model

## Verification

Frontend compiles without TypeScript errors:
- npx vue-tsc --noEmit passes

## Success Criteria Status

- [x] Users can select Kimi, DeepSeek, MiniMax from dropdown
- [x] base_url is pre-filled with defaults, editable
- [x] model_name is pre-filled with defaults, editable
- [x] API key is saved with config
- [x] Chat uses selected model's configured endpoint

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check: PASSED

All files modified exist and commits verified:
- 5c1d8ad: feat(04-02): add model config state to llmStore
- 48d69fc: feat(04-02): add model config API methods
- ca90928: feat(04-02): update LlmSettings with Chinese LLM config UI
