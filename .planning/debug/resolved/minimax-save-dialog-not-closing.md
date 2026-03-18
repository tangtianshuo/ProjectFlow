---
status: resolved
trigger: "Configure MiniMax API key - after clicking save, the settings dialog doesn't close. After saving, chatting shows 'API key not configured'"
created: 2026-03-18T00:00:00.000Z
updated: 2026-03-18T00:00:00.000Z
---

## Current Focus
Human verified: 已修复 - user rebuilt and tested

## Symptoms
expected: 保存成功后弹窗应该关闭
actual: 弹窗已关闭，但聊天时显示 api key 未配置
errors: 聊天时显示 "API key not configured" 错误提示
reproduction: 首次配置 MiniMax API key，点击保存按钮后弹窗关闭，但聊天显示 apikey 未配置
started: 刚刚配置时出现

## Current Investigation

Dialog closing is now fixed. The remaining issue is that after saving the API key, chatting shows "API key not configured".

Added Rust debug logging to trace:
1. What model_id is received when saving
2. What model is received when chatting
3. Whether the API key is found in keyring

## Evidence

- timestamp: 2026-03-18
  checked: "LlmSettings.vue saveKey() function"
  found: "Function saves API key and updates status, but never calls emit('close') to close dialog"
  implication: "Dialog remains open after save - this explains issue 1"

- timestamp: 2026-03-18
  checked: "llm_chat Rust command for key retrieval"
  found: "Uses model parameter to retrieve API key. If selectedModelId is not properly set in store after save, defaults to 'gpt-4o'"
  implication: "This explains issue 2 - wrong model used when retrieving key"

- timestamp: 2026-03-18
  checked: "llmStore.selectedModelId initialization"
  found: "selectedModelId defaults to 'gpt-4o' but user selects 'minimax'. After save, selectedModelId is set to selectedModel.value (minimax), but the store's selectedModel (not selectedModelId) remains 'gpt-4o'"
  implication: "Potential confusion between selectedModel and selectedModelId - need to ensure selectedModelId is properly used"

- timestamp: 2026-03-18
  checked: "Fix verification - LlmSettings.vue code"
  found: "emit('close') is now on line 86, after saveModelConfig and store updates. Code looks correct."
  implication: "Dialog should close after save"

- timestamp: 2026-03-18
  checked: "Rust llm_save_model_config command"
  found: "Stores API key using keyring with model_id as the key. Retrieves using same model_id."
  implication: "Should work if keyring is functioning"

- timestamp: 2026-03-18
  checked: "Added debug logging to LlmSettings.vue and llmStore.ts"
  found: "Console logs added to trace: (1) what model is saved (2) what model is used when sending message"
  implication: "Will help identify if model is correctly passed"

- timestamp: 2026-03-18
  checked: "Added Rust debug logging to llm_save_model_config and llm_chat"
  found: "Logging added to show: (1) model_id received when saving (2) model received when chatting (3) whether key is found"
  implication: "Will help trace keyring storage/retrieval issues"

## Eliminated

- dialog not closing: FIXED - emit("close") added
- selectedModel not updated: FIXED - added llmStore.selectedModel = selectedModel.value

## Resolution

root_cause: "LlmSettings.vue missing emit('close') to close dialog after save, and llmStore.selectedModel not updated after saving model config"
fix: "Added emit('close') in saveKey() after successful save, and added llmStore.selectedModel = selectedModel.value to update store state"
verification: "User rebuilt and confirmed the dialog now closes and chat works with minimax API key"
files_changed:
- "src/components/features/llm/LlmSettings.vue"
- "src/stores/llmStore.ts"
