# WinSage - Windows Terminal AI Agent

一个功能强大的Windows终端AI助手，具备安全沙箱隔离、多模型支持和长期记忆功能。

## 特性

- **多模型支持**: OpenAI GPT、Anthropic Claude、Azure AI、Ollama、DeepSeek、Kimi、Qwen、Doubao
- **安全沙箱**: 使用Windows Sandbox隔离危险命令执行
- **长期记忆**: PostgreSQL数据库存储对话历史和知识库
- **交互式CLI**: 友好的命令行界面
- **模块化架构**: 易于扩展新的模型提供商和功能

## 系统要求

### 必需
- Windows 10/11 专业版或企业版（用于Windows Sandbox）
- Rust工具链 (MSVC)
- Visual Studio Build Tools 2019或更高版本

### 可选
- PostgreSQL 14+（用于记忆系统）
- Docker Desktop（用于快速部署PostgreSQL）

## 安装

### 🚀 Cargo 安装（跨平台，推荐）

适用于 Windows、macOS、Linux：

```bash
cargo install --git https://github.com/wsxyy1145/winsage-agent.git
```

安装完成后即可使用：
```bash
winsage chat
```

**优点**: 
- ✅ 跨平台支持（Windows/macOS/Linux）
- ✅ 自动获取最新版本
- ✅ 一行命令完成

---

### 📦 GitHub Releases 下载（最快，无需编译）

访问最新 Release 页面下载预编译二进制：
https://github.com/wsxyy1145/winsage-agent/releases/latest

- **Windows**: 下载 `winsage-windows-x64.exe`
- **macOS**: 下载 `winsage-macos-x64`
- **Linux**: 下载 `winsage-linux-x64`

---

### 🔧 Git Clone + 本地编译（开发用）

```bash
git clone https://github.com/wsxyy1145/winsage-agent.git
cd winsage-agent
cargo build --release

# 运行
./target/release/winsage chat
```

---

### 系统要求

#### Windows
- Windows 10/11 专业版或企业版（用于 Windows Sandbox）
- Rust 工具链 (MSVC)
- Visual Studio Build Tools 2019+

#### macOS (未来支持)
- macOS 10.15+
- Rust 工具链
- Xcode Command Line Tools

#### Linux (未来支持)
- Ubuntu 18.04+ / CentOS 7+
- Rust 工具链
- OpenSSL 开发库

---

### 手动安装（高级用户）

#### 1. 安装Rust

```powershell
winget install Rustlang.Rust.MSVC
```

或从 [rustup.rs](https://rustup.rs) 下载

#### 2. 安装Visual Studio Build Tools

1. 下载 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. 运行安装程序
3. 选择 "C++ build tools" 工作负载
4. 确保包含以下组件：
   - MSVC v142 or later
   - Windows SDK
   - C++ CMake tools

#### 3. 安装PostgreSQL（可选）

使用Docker快速启动：
```bash
docker run --name winsage-postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=winsage -p 5432:5432 -d postgres:16
```

或直接安装 [PostgreSQL](https://www.postgresql.org/download/windows/)

#### 4. 克隆并编译

```bash
git clone <repository-url>
cd WinSage-agent
cargo build --release
```

编译后的可执行文件位于 `target/release/winsage.exe`

## 配置

首次运行时会自动创建配置文件：`%APPDATA%\winsage\config.toml`

### 基本配置示例

```toml
[general]
default_model = "gpt-4"
temperature = 0.7
max_tokens = 4096

[models.openai]
api_key = "sk-your-openai-key"
base_url = "https://api.openai.com/v1"
default_model = "gpt-4"

[models.anthropic]
api_key = "your-anthropic-key"

[models.azure]
api_key = "your-azure-key"
endpoint = "https://your-resource.openai.azure.com/"
deployment = "gpt-4"

[models.ollama]
base_url = "http://localhost:11434"
model = "llama2"

# DeepSeek 配置（可选）
[models.deepseek]
api_key = "sk-your-deepseek-key"  # DeepSeek API Key
base_url = "https://api.deepseek.com/v1"
default_model = "deepseek-chat"

# Kimi (Moonshot) 配置（可选）
[models.moonshot]
api_key = "sk-your-moonshot-key"  # Kimi API Key
base_url = "https://api.moonshot.cn"
default_model = "moonshot-v1-8k"

# Qwen (通义千问) 配置（可选）
[models.qwen]
api_key = "your-qwen-api-key"  # 阿里云 DashScope API Key
base_url = "https://dashscope.aliyuncs.com"
default_model = "qwen-turbo"

# Doubao (豆包) 配置（可选）
[models.doubao]
api_key = "your-doubao-api-key"  # 火山引擎 API Key
base_url = "https://ark.cn-beijing.volces.com"
default_model = "ep-xxxxx"  # 替换为您的 endpoint ID

[memory]
database_url = "postgresql://postgres:postgres@localhost/winsage"
enable_vector_search = false

[sandbox]
enabled = true
auto_cleanup = true
timeout_seconds = 300
```

也可以使用环境变量：
```powershell
$env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-your-key"
```

## 使用方法

### 🚀 快速开始

**1. 运行配置向导（推荐首次使用）**

```bash
winsage setup
```

配置向导将引导您：
- ✓ 选择 AI 模型（OpenAI/DeepSeek/Kimi/Qwen/Doubao等）
- ✓ 输入 API Key
- ✓ 设置温度参数和最大 Token
- ✓ 配置 PostgreSQL 记忆系统（可选）
- ✓ 配置 Windows Sandbox 沙箱（可选）

**2. 开始聊天**

```bash
winsage chat
```

或直接运行：
```bash
winsage
```

### 管理配置
```bash
winsage config
```

### 查看沙箱状态
```bash
winsage sandbox
```

### 聊天中的命令
- `quit` 或 `exit` - 退出聊天
- `help` - 显示帮助

## 架构

```
WinSage-agent/
├── src/
│   ├── agent/          # AI Agent核心逻辑
│   ├── cli/            # 命令行界面
│   ├── config/         # 配置管理
│   ├── memory/         # PostgreSQL记忆系统
│   ├── models/         # 模型提供商实现
│   ├── sandbox/        # Windows Sandbox集成
│   └── utils/          # 工具函数
├── migrations/         # 数据库迁移脚本
└── config/             # 默认配置模板
```

## 开发

### 添加新模型提供商

1. 在 `src/models/` 创建新文件
2. 实现 `ModelProvider` trait
3. 在 `src/models/mod.rs` 中导出

### 调试

设置日志级别：
```powershell
$env:RUST_LOG = "winsage=debug"
winsage chat
```

## 安全注意事项

1. **API密钥**: 不要将API密钥提交到版本控制
2. **沙箱**: 默认启用沙箱执行危险命令
3. **权限**: 以最小必要权限运行

## 故障排除

### 编译错误 "link.exe failed"

确保已安装Visual Studio Build Tools和C++工作负载。

### Windows Sandbox不可用

检查Windows版本是否为专业版或企业版，并在"启用或关闭Windows功能"中启用Windows Sandbox。

### 数据库连接失败

确保PostgreSQL正在运行，并且连接字符串正确。

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！
