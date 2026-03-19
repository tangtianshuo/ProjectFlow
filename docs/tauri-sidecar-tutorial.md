# Tauri Sidecar 使用教程

## 什么是 Tauri Sidecar？

Tauri Sidecar 是 Tauri v2 引入的一项强大功能，允许你在 Tauri 应用中嵌入和运行外部可执行文件。这些外部程序可以是 Python 脚本、Node.js 服务、C++ 编译的程序等，使得 Tauri 应用能够轻松集成其他语言编写的模块。

## 为什么要使用 Sidecar？

1. **语言灵活性**：无需学习 Rust 即可使用其他语言编写业务逻辑
2. **生态复用**：可以直接使用 Python、Node.js 等语言的丰富生态系统
3. **渐进迁移**：可以逐步将现有项目迁移到 Tauri
4. **独立运行**：Sidecar 作为独立进程运行，不影响主应用性能

## 项目结构

```
src-tauri/
├── sidecar/                  # Python Sidecar 目录
│   ├── main.py              # 入口文件
│   ├── api.py               # FastAPI 服务
│   ├── config.py            # 数据模型
│   ├── openai_client.py     # OpenAI SDK 封装
│   ├── anthropic_client.py  # Anthropic SDK 封装
│   └── requirements.txt     # Python 依赖
├── src/
│   ├── lib.rs               # Rust 后端（调用 Sidecar）
│   └── ...
├── Cargo.toml               # Rust 依赖
└── tauri.conf.json          # Tauri 配置
```

## 快速开始

### 1. 安装 Python 依赖

在 `sidecar` 目录下安装依赖：

```bash
cd src-tauri/sidecar
pip install -r requirements.txt
```

依赖包括：
- `fastapi` - Web 框架
- `uvicorn` - ASGI 服务器
- `openai` - OpenAI SDK
- `anthropic` - Anthropic SDK
- `pydantic` - 数据验证
- `httpx` - HTTP 客户端

### 2. 配置 Tauri 权限

在 `capabilities/default.json` 中添加必要的权限：

```json
{
  "permissions": [
    "core:default",
    "shell:allow-spawn",
    "shell:allow-execute",
    "shell:allow-open",
    "http:default"
  ]
}
```

### 3. 启动 Sidecar

在 Rust 中启动 Sidecar：

```rust
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn start_sidecar(app: AppHandle) -> Result<u16, String> {
    let python_path = if cfg!(windows) { "python" } else { "python3" };
    
    let child = tauri_plugin_shhell::shell(&app)
        .command(python_path)
        .args(["--port", "8765"])
        .current_dir(...)
        .spawn()
        .map_err(|e| format!("Failed to start sidecar: {}", e))?;
    
    Ok(8765)
}
```

### 4. 调用 Sidecar API

通过 HTTP 请求与 Sidecar 通信：

```rust
use tauri_plugin_http::reqwest;

#[tauri::command]
async fn call_sidecar_api(
    messages: Vec<serde_json::Value>,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:8765/chat")
        .json(&serde_json::json!({ "messages": messages }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let result = response.json::<serde_json::Value>().await.map_err(|e| e.to_string())?;
    Ok(result)
}
```

## Sidecar API 端点

本项目实现的 Sidecar API 包括：

### 健康检查
```
GET /health
```

### 获取配置
```
GET /config
```

### 更新配置
```
POST /config
Body: {
    "provider": "openai" | "anthropic" | "openai-compatible",
    "api_key": "your-api-key",
    "api_url": "https://api.openai.com/v1",
    "model": "gpt-4o-mini"
}
```

### 聊天完成
```
POST /chat
Body: {
    "messages": [
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "Hello!"}
    ],
    "model": "gpt-4o-mini",
    "temperature": 0.7,
    "max_tokens": 1024,
    "stream": false
}
```

### 模型列表
```
GET /models
```

## 前端集成

在 Vue/TypeScript 前端中调用：

```typescript
import { llmApi } from "./lib/api";

// 启动 Sidecar
const port = await llmApi.startSidecar();

// 更新配置
await llmApi.updateConfig(
    "openai",
    "your-api-key", 
    "https://api.openai.com/v1",
    "gpt-4o-mini"
);

// 发送消息
const response = await llmApi.chatWithLlm(
    [{ role: "user", content: "Hello!" }],
    0.7,
    1024
);
```

## 注意事项

1. **端口冲突**：确保 Sidecar 使用的端口（8765）不被其他程序占用
2. **跨域问题**：开发环境可能需要配置 CORS
3. **错误处理**：网络请求可能失败，需要做好异常处理
4. **安全性**：API Key 不应硬编码，建议使用环境变量或用户输入

## 常见问题

### Q: Sidecar 启动失败怎么办？
A: 检查 Python 环境是否正确安装，端口是否被占用，路径是否正确。

### Q: 如何调试 Sidecar？
A: 可以直接运行 `python main.py --port 8765` 启动 Sidecar，然后用浏览器访问 `http://127.0.0.1:8765/docs` 查看 API 文档。

### Q: 生产环境如何打包？
A: 需要将 Python 文件和依赖一起打包到应用资源中，详见 Tauri 官方文档。

## 参考资料

- [Tauri Sidecar 官方文档](https://tauri.app/zh-cn/guides/sidecar/)
- [FastAPI 官方文档](https://fastapi.tiangolo.com/)
- [OpenAI SDK Python](https://github.com/openai/openai-python)
- [Anthropic SDK Python](https://github.com/anthropics/anthropic-sdk-python)
