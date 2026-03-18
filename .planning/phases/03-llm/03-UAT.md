---
status: testing
phase: 03-llm
source: 03-01-SUMMARY.md, 03-02-SUMMARY.md, 03-03-SUMMARY.md
started: 2026-03-18T10:00:00Z
updated: 2026-03-18T10:00:00Z
---

## Current Test

number: 1
name: Sidebar shows AI Assistant menu
expected: |
  打开应用后，侧边栏应该显示 "AI 助手" 菜单项（带机器人图标），位于回收站之前。
awaiting: user response

## Tests

### 1. Sidebar shows AI Assistant menu
expected: 打开应用后，侧边栏应该显示 "AI 助手" 菜单项（带机器人图标），位于回收站之前。
result: [pending]

### 2. Click AI Assistant opens chat panel
expected: 点击侧边栏的 "AI 助手" 菜单项后，主内容区域应显示聊天面板，包含消息输入框和发送按钮。
result: [pending]

### 3. Settings modal opens
expected: 点击聊天面板右上角的设置图标，应弹出设置模态框，显示 API Key 配置选项。
result: [pending]

### 4. Can save API key
expected: 在设置模态框中输入 OpenAI API Key 并保存，应该成功保存并显示确认信息。
result: [pending]

### 5. Send message and receive response
expected: 在输入框输入问题并发送，应能看到流式响应逐字显示在聊天区域。
result: [pending]

### 6. Markdown rendering works
expected: AI 返回的 Markdown 格式内容（如代码块、粗体、列表）应正确渲染显示。
result: [pending]

### 7. Project context is included
expected: 当选择特定项目后，AI 应能了解项目信息（名称、描述、任务列表）。
result: [pending]

## Summary

total: 7
passed: 0
issues: 0
pending: 7
skipped: 0

## Gaps

[none yet]
