# LLM WebSocket 流式对话实现计划

## 概述

基于当前项目的 LLM 实现，通过 WebSocket 实现流式对话功能，提供更流畅的用户体验。

## 当前实现分析

### Python Sidecar (已有流式支持)
- `api.py` 中的 `/chat` 端点支持 `stream: true` 参数
- 使用 Server-Sent Events (SSE) 返回流式数据
- OpenAI 和 Anthropic 客户端都支持流式响应

### 前端 (当前为非流式)
- `llmStore.ts` 使用非流式 `chatWithLlm` API
- `LlmPanel.vue` 显示静态响应，无实时流式更新

## 实现步骤

### 第一阶段：后端 WebSocket 支持

1. **修改 Python Sidecar**
   - 添加 WebSocket 端点 `/ws/chat`
   - 支持流式 JSON 推送（替代 SSE）
   - 保持向后兼容现有的 HTTP 端点

2. **修改 Rust 后端**
   - 添加 WebSocket 命令支持
   - 或者通过 HTTP 流式响应透传

### 第二阶段：前端 WebSocket 客户端

3. **更新 `src/lib/api.ts`**
   - 添加 WebSocket 连接函数
   - 添加流式聊天方法
   - 处理连接、消息、错误、断开事件

4. **更新 `src/stores/llmStore.ts`**
   - 添加 WebSocket 状态管理
   - 实现流式消息接收
   - 处理实时内容更新

### 第三阶段：UI 流式显示

5. **更新 `src/components/features/llm/LlmPanel.vue`**
   - 实时显示流式内容
   - 添加流式动画效果
   - 支持中断生成

## 技术方案

### 方案 A：HTTP SSE 流式 (推荐，简单)
- 继续使用 HTTP 长连接
- 使用 `fetch` 的 ReadableStream
- 后端无需大改

### 方案 B：WebSocket
- 建立持久连接
- 双向通信
- 适合需要中断/动态控制的场景

## 预期效果

- 用户发送消息后，内容实时逐字显示
- 显示加载动画指示流式传输
- 支持流式中断
- 更好的用户体验
