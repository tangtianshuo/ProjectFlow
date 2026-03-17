# 技术栈

**分析日期:** 2026-03-17

## 编程语言

**主要:**
- TypeScript 5.6.2 - 前端核心语言
- Vue 3.5.13 - 前端框架
- Rust 2021 Edition - 后端核心语言

**次要:**
- HTML/CSS - 页面结构与样式

## 运行时

**环境:**
- Node.js 20 (CI/CD) - 前端开发与构建
- Rust stable - 后端编译

**包管理:**
- npm - 前端依赖管理
- Cargo - Rust 依赖管理
- Lockfile: `package-lock.json` (存在)

## 框架

**前端核心:**
- Vue 3.5.13 - 渐进式前端框架
- Pinia 3.0.4 - 状态管理

**前端构建:**
- Vite 6.0.3 - 开发服务器与构建工具
- @vitejs/plugin-vue 5.2.1 - Vue 插件
- vue-tsc 2.1.10 - TypeScript 类型检查

**UI 组件库:**
- @vue-flow/core 1.48.2 - 任务看板可视化
- vue-echarts 8.0.1 / echarts 6.0.0 - 图表展示
- lucide-vue-next 0.577.0 - 图标库
- @guolao/vue-monaco-editor 1.6.0 - 代码编辑器 (Monaco)
- marked 17.0.3 - Markdown 解析

**CSS 框架:**
- Tailwind CSS 4.2.1 - 原子化 CSS 框架
- PostCSS 8.5.8 - CSS 处理
- Autoprefixer 10.4.27 - 自动前缀

**测试:**
- Vitest 4.0.18 - 测试框架
- @vitest/coverage-v8 4.0.18 - 代码覆盖率
- @vue/test-utils 2.4.6 - Vue 组件测试
- @testing-library/vue 8.1.0 - Vue 测试库
- jsdom 24.1.3 - DOM 模拟

**后端:**
- Tauri 2 - 桌面应用框架
- rusqlite 0.31 - SQLite 数据库 (bundled)
- tokio 1 - 异步运行时
- serde 1 / serde_json 1 - 序列化/反序列化
- chrono 0.4 - 日期时间处理
- uuid 1 - UUID 生成
- thiserror 1 - 错误处理
- log 0.4 / env_logger 0.11 - 日志框架

**Tauri 插件:**
- @tauri-apps/api 2 - Tauri API 客户端
- tauri-plugin-opener 2 - 打开外部链接
- @tauri-apps/plugin-opener 2 - 前端插件

## 关键依赖

**核心功能:**
- `vue` 3.5.13 - Vue 核心库
- `pinia` 3.0.4 - 状态管理
- `@tauri-apps/api` 2 - Tauri IPC 通信

**数据可视化:**
- `@vue-flow/core` 1.48.2 - 任务看板流程图
- `echarts` 6.0.0 - 图表库

**编辑器:**
- `@guolao/vue-monaco-editor` 1.6.0 - Monaco 代码编辑器
- `marked` 17.0.3 - Markdown 渲染

**数据库:**
- `rusqlite` 0.31 - Rust SQLite 绑定 (bundled)

**工具库:**
- `@vueuse/core` 14.2.1 - Vue 组合式工具
- `@vueuse/motion` 3.0.3 - 动画库

## 配置

**前端构建:**
- `vite.config.ts` - Vite 配置
- `tsconfig.json` - TypeScript 配置
- `postcss.config.js` - PostCSS 配置
- `tailwind.config.js` - Tailwind CSS 配置

**后端配置:**
- `src-tauri/Cargo.toml` - Rust 依赖配置
- `src-tauri/tauri.conf.json` - Tauri 应用配置

**TypeScript 配置:**
```json
{
  "target": "ES2020",
  "module": "ESNext",
  "strict": true,
  "moduleResolution": "bundler"
}
```

**Tauri 配置:**
- 应用标识符: `com.projectflow.desktop`
- 窗口尺寸: 1280x800 (最小)
- 开发端口: 15173
- 构建目标: Windows/macOS

## 平台要求

**开发:**
- Node.js 20
- Rust stable
- Windows 10+ / macOS

**生产:**
- Windows 10+ (exe)
- macOS 11+ (dmg)

---

*技术栈分析: 2026-03-17*
