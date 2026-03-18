# Phase 4: 国产LLM模型支持 - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

扩展 LLM 助手以支持国产大语言模型（Kimi、DeepSeek、MiniMax）。用户可配置每个模型的 baseurl、api_key、model_name，通过 OpenAI 兼容接口实现。

**本阶段范围：**
- 支持 Kimi (月之暗面)、DeepSeek (深度求索)、MiniMax
- 每个模型独立配置（base_url + api_key + model）
- OpenAI 兼容客户端实现
- UI 下拉选择切换模型

**后续阶段可扩展：**
- 更多国产模型
- Claude等其他国际模型

</domain>

<decisions>
## Implementation Decisions

### 支持的模型
- **Kimi** (moonshot.ai)
- **DeepSeek** (deepseek.com)
- **MiniMax** (minimax.io)

### 配置参数（每个模型独立）
- `base_url`: API 端点地址
- `api_key`: API 密钥
- `model`: 模型名称

### 技术方案
- **OpenAI 兼容接口**：使用 openai-compat 或类似 crate
- 所有支持 OpenAI API 格式的模型都可接入
- 流式响应处理

### UI 交互
- **下拉选择**：在设置页面显示模型列表，用户下拉选择
- 切换模型后自动使用对应配置

### 第三方库
- 使用 **openai-compat** 或类似 Rust crate
- 不重复造轮子，利用成熟的开源库

### Claude's Discretion
- 具体 UI 细节设计
- 错误处理逻辑
- 各模型默认 baseurl

</decisions>

<specifics>
## Specific Ideas

- 类似于 ChatGPT 的模型切换体验
- 每个模型的 API 分别存储
- 预设常用模型的默认 baseurl，用户可修改

</specifics>

<canonical_refs>
## Canonical References

### Project
- `.planning/PROJECT.md` — 项目定义
- `.planning/STATE.md` — 当前进度状态
- `.planning/ROADMAP.md` — 阶段规划

### Existing Code (Phase 3)
- `.planning/phases/03-llm/03-CONTEXT.md` — Phase 3 决策
- `src-tauri/src/llm/` — 现有 LLM 模块
- `src/stores/llmStore.ts` — LLM 状态管理
- `src/components/features/llm/` — LLM UI 组件

### 技术参考
- OpenAI API 兼容格式文档
- 各模型 API 文档（Kimi、DeepSeek、MiniMax）

</canonical_refs>

.Codebase
## Existing Code Insights

### Reusable Assets
- `src-tauri/src/llm/openai.rs` — 可复用的 API 调用逻辑
- `src-tauri/src/llm/storage.rs` — API Key 加密存储
- `src/stores/llmStore.ts` — 状态管理
- `src/components/features/llm/LlmSettings.vue` — 设置模态框

### Established Patterns
- Tauri 命令调用方式
- API Key 加密存储（AES-GCM + keyring）
- 流式响应处理（llm-token 事件）

### Integration Points
- 扩展现有 llmStore 支持多模型
- 修改 LlmSettings.vue 添加模型选择下拉
- 复用现有 streaming 逻辑

</code_context>

<deferred>
## Deferred Ideas

- Claude 等国际模型支持 — 后续阶段

</deferred>

---

*Phase: 04-llm-chinese-models*
*Context gathered: 2026-03-18*
