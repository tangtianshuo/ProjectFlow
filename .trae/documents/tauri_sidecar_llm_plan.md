# Tauri Sidecar Python LLM 实现计划

## 概述

本计划旨在通过 Tauri Sidecar 机制集成 Python 后端，使用 OpenAI SDK 和 Anthropic SDK 实现 LLM 对话功能。

## 实现步骤

### 第一阶段：Python Sidecar 环境搭建

1. **创建 Python 虚拟环境**
   - 在 `src-tauri/sidecar/` 目录下创建 Python 虚拟环境
   - 安装必要的依赖包

2. **安装 SDK 依赖**
   - `pip install openai anthropic`
   - 可选：`python-dotenv` 用于环境变量管理

3. **创建 Python 服务入口**
   - 创建 `src-tauri/sidecar/main.py` 作为主程序
   - 实现 HTTP 服务器（可使用 FastAPI 或 Flask）监听指定端口

### 第二阶段：Python LLM 模块实现

4. **实现 OpenAI 对话模块**
   - 创建 `src-tauri/sidecar/openai_client.py`
   - 封装 `chat_completions` 接口
   - 支持流式和非流式响应

5. **实现 Anthropic 对话模块**
   - 创建 `src-tauri/sidecar/anthropic_client.py`
   - 封装 Claude API 调用
   - 支持流式和非流式响应

6. **统一 API 入口**
   - 创建 `src-tauri/sidecar/api.py` 统一处理前端请求
   - 支持配置更新、聊天完成、模型列表等接口

### 第三阶段：Tauri 配置

7. **配置 Tauri Sidecar**
   - 修改 `src-tauri/tauri.conf.json`
   - 添加 sidecar 配置项
   - 指定 Python 解释器路径

8. **Rust 后端集成**
   - 修改现有 Rust 命令通过 sidecar 调用 Python 服务
   - 使用 HTTP 请求与 Python 服务通信
   - 或使用 Tauri 的子进程 API 直接调用

### 第四阶段：前端适配

9. **更新前端 API 调用**
   - 修改 `src/lib/api.ts` 指向新的 sidecar 端点
   - 或继续通过 Rust 命令间接调用

10. **测试与调试**
    - 端到端测试 LLM 对话功能
    - 处理跨域、认证等问题

## Tauri Sidecar 使用教程

### 什么是 Tauri Sidecar？

Tauri Sidecar 允许在 Tauri 应用中嵌入外部可执行文件（如 Python 脚本、Node.js 服务等），使其作为应用的一部分运行。

### 基础配置步骤

1. **在 `tauri.conf.json` 中配置：**
   ```json
   {
     "bundle": {
       "targets": "all"
     },
     "tauri": {
       "sidecar": {
         "name": "llm-server",
         "script": "src-tauri/sidecar/main.py",
         "args": ["--port", "8080"]
       }
     }
   }
   ```

2. **在 Rust 中启动/停止 Sidecar：**
   - 使用 `tauri::api::process::Command` 管理 sidecar 进程

### 完整示例

详见正式文档：https://tauri.app/zh-cn/guides/sidecar/

## 预期产出

1. 工作的 Python Sidecar 服务
2. 完整的 OpenAI 和 Anthropic 对话功能
3. 可运行的桌面应用
4. Tauri Sidecar 使用教程文档
