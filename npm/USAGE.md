# nuwax-grok 使用指南

`nuwax-grok` 是 Grok Build CLI（Rust 实现的 agentic 编程工具）的 npm 分发版。本指南介绍如何通过 npm 安装，并接入你自己的大模型（自定义 OpenAI / Anthropic 兼容模型），在编辑器里使用。

> 命令名是 `nuwax-grok`，与官方 `grok` 互不冲突，两者可共存。

---

## 一、安装

要求：Node.js ≥ 18。

```bash
npm install -g nuwax-grok-build
nuwax-grok -V          # 验证，输出例如 grok 0.2.107 (...)
```

**支持平台**：macOS（Apple Silicon / Intel）、Linux（x64 / arm64）。Windows 暂不支持。

---

## 二、支持的 agent 协议

`nuwax-grok` 底层是 `agent` 命令，提供 4 种运行模式：

| 命令 | 协议 | 用途 |
|------|------|------|
| `nuwax-grok agent stdio` | **ACP**（JSON-RPC over stdio） | **编辑器集成（Zed / VS Code / Cursor 等）** ← 主要用这个 |
| `nuwax-grok agent serve` | WebSocket 服务器（默认 `127.0.0.1:2419`，带 secret 鉴权） | 本地多客户端共享一个 agent |
| `nuwax-grok agent headless` | Grok WebSocket relay（云端中转） | 远程 / 无人值守 |
| `nuwax-grok agent leader` | 共享 leader 后端进程 | 多个 TUI/IDE 客户端共用一个后端 |

**编辑器集成统一用 `agent stdio`**：编辑器自己 spawn 这个进程，通过 stdin/stdout 收发 ACP 消息。

---

## 三、接入自定义大模型

通过环境变量配置。核心变量：

| 变量 | 作用 | 必填 |
|------|------|------|
| `GROK_MODEL_BASE_URL` | 模型 API 基地址 | ✅ |
| `GROK_MODEL_API_KEY` | API 密钥（Bearer 鉴权） | ✅ |
| `GROK_DEFAULT_MODEL` | 默认模型（不设 `GROK_MODEL_ID` 时同时作为模型 id） | ✅ |
| `GROK_MODEL_ID` | 模型 id，作为 `model` 字段发给 API（不设则用 `GROK_DEFAULT_MODEL`） | 否 |
| `GROK_MODEL_API_BACKEND` | API 协议格式（**枚举**）：`chat_completions`=OpenAI `/v1/chat/completions`（**默认**，所有 OpenAI 兼容服务）/ `responses`=OpenAI `/v1/responses` / `messages`=Anthropic `/v1/messages` | 否 |
| `GROK_MODEL_CONTEXT_WINDOW` | 上下文窗口大小（tokens） | 否 |
| `GROK_MODEL_DISPLAY_NAME` | 在 UI 中显示的名字 | 否 |

**最小配置（3 个变量即可跑起来）：**

```bash
GROK_DEFAULT_MODEL=deepseek-v4-flash
GROK_MODEL_BASE_URL=https://api.deepseek.com/v1
GROK_MODEL_API_KEY=sk-xxx
```

---

## 四、编辑器集成（以 Zed 为例）

在 Zed 的 `settings.json`（macOS：`~/.config/zed/settings.json`）中，添加如下 provider 配置（放到你已有的 agent provider 结构里）：

```jsonc
"nuwax-grok-deepseek": {
  "default_config_options": { "mode": "auto" },
  "type": "custom",
  "command": "nuwax-grok",
  "args": ["agent", "stdio"],
  "env": {
    "GROK_DEFAULT_MODEL": "deepseek-v4-flash",
    "GROK_MODEL_ID": "deepseek-v4-flash",
    "GROK_MODEL_BASE_URL": "https://api.deepseek.com/v1",
    "GROK_MODEL_API_KEY": "sk-换成你自己的key",
    "GROK_MODEL_API_BACKEND": "chat_completions",
    "GROK_MODEL_CONTEXT_WINDOW": "1000000",
    "GROK_MODEL_DISPLAY_NAME": "DeepSeek V4 Flash"
  }
}
```

> VS Code / Cursor 等：把上面的 `command`/`args`/`env` 填到对应 AI agent 扩展的「custom command provider」配置里即可，参数含义一致。

---

## 五、各家 provider 配置示例

换 provider 只需改 `GROK_MODEL_BASE_URL` / `GROK_MODEL_API_BACKEND` / `GROK_DEFAULT_MODEL`（以及 `API_KEY`）。

### DeepSeek
```
GROK_MODEL_BASE_URL=https://api.deepseek.com/v1
GROK_MODEL_API_BACKEND=chat_completions
GROK_DEFAULT_MODEL=deepseek-chat        # 或 deepseek-reasoner
```

### OpenAI
```
GROK_MODEL_BASE_URL=https://api.openai.com/v1
GROK_MODEL_API_BACKEND=chat_completions   # 或 responses
GROK_DEFAULT_MODEL=gpt-4o
```

### Anthropic Claude
```
GROK_MODEL_BASE_URL=https://api.anthropic.com/v1
GROK_MODEL_API_BACKEND=messages
GROK_DEFAULT_MODEL=claude-3-5-sonnet-20241022
```

### 通义千问（OpenAI 兼容）
```
GROK_MODEL_BASE_URL=https://dashscope.aliyuncs.com/compatible-mode/v1
GROK_MODEL_API_BACKEND=chat_completions
GROK_DEFAULT_MODEL=qwen-plus
```

### 智谱 GLM（OpenAI 兼容）
```
GROK_MODEL_BASE_URL=https://open.bigmodel.cn/api/paas/v4
GROK_MODEL_API_BACKEND=chat_completions
GROK_DEFAULT_MODEL=glm-4-plus
```

### 本地模型（Ollama / vLLM / LM Studio）
```
GROK_MODEL_BASE_URL=http://localhost:11434/v1     # Ollama；vLLM 用 http://localhost:8000/v1
GROK_MODEL_API_BACKEND=chat_completions
GROK_DEFAULT_MODEL=qwen2.5:32b
GROK_MODEL_API_KEY=ollama                          # 本地服务可随便填一个非空值
```

---

## 六、日志与调试

在编辑器配置的 `env` 里按需加入：

```jsonc
"RUST_LOG": "debug",                            // 日志级别：error < warn < info < debug < trace
"GROK_LOG_FILE": "/tmp/nuwax-grok.log",         // 写到文件（推荐，方便查看）
"GROK_HOOKS_LOG": "1"                           // 可选：hooks 执行日志
```

查看日志：

```bash
tail -f /tmp/nuwax-grok.log
```

> **说明**：
> - `RUST_LOG` 控制日志详细级别（同时影响 stderr 和 `GROK_LOG_FILE` 文件）。
> - `GROK_LOG_FILE` 是**单个文件路径**（不是目录），父目录会自动创建；默认 DEBUG 级。
> - `GROK_DEBUG_LOG=1` 是另一种方式（按会话分文件写到 `~/.grok/debug/`，带 `latest.txt` 软链）。**不要和 `GROK_LOG_FILE` 同时设**——同时设时 `GROK_LOG_FILE` 优先，`GROK_DEBUG_LOG` 被忽略。

---

## 七、常见问题

**Q：Zed 报 `nuwax-grok: command not found`？**
A：GUI 应用的 PATH 可能不全。把 `command` 改成绝对路径：
- Apple Silicon Mac：`/opt/homebrew/bin/nuwax-grok`
- Intel Mac：`/usr/local/bin/nuwax-grok`
- Linux：用 `which nuwax-grok` 的输出

**Q：和官方 `grok` 会冲突吗？**
A：不会。本包命令是 `nuwax-grok`，官方是 `grok`，互不影响、可共存。

**Q：怎么确认自定义模型配置生效了？**
A：开启 `GROK_LOG_FILE` 看日志（会看到模型加载、HTTP 请求等），或在编辑器里发一条消息看是否正常响应。

**Q：`agent stdio` 和直接运行二进制有什么区别？**
A：`nuwax-grok` 是个 Node 启动器，会自动选用对应平台的预编译二进制并 spawn 它，参数和环境变量原样透传。对 stdio 协议完全透明，只多约 30ms 一次性启动开销。

---

更多见仓库根目录 [README.md](../README.md)（项目总览）与 [npm/README.md](README.md)（npm 打包 / 发版说明）。
