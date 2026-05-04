# WinSage 快速开始指南

## 5分钟上手

### 步骤1: 安装依赖（首次使用）

```powershell
# 1. 安装Rust（如果未安装）
winget install Rustlang.Rust.MSVC

# 2. 下载Visual Studio Build Tools
# 访问: https://visualstudio.microsoft.com/visual-cpp-build-tools/
# 选择 "C++ build tools" 工作负载并安装
```

### 步骤2: 编译项目

```powershell
# 进入项目目录
cd WinSage-agent

# 编译
cargo build --release
```

### 步骤3: 配置API密钥

**方式A: 使用配置向导**
```bash
.\target\release\winsage.exe config
```
按提示输入API密钥

**方式B: 编辑配置文件**
```powershell
notepad $env:APPDATA\winsage\config.toml
```

添加你的API密钥：
```toml
[models.openai]
api_key = "sk-你的OpenAI密钥"
```

**方式C: 环境变量**
```powershell
$env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-你的密钥"
```

### 步骤4: 开始聊天！

```bash
.\target\release\winsage.exe chat
```

## 常用命令速查

| 命令 | 功能 |
|------|------|
| `winsage` | 启动聊天（默认） |
| `winsage chat` | 启动交互式聊天 |
| `winsage config` | 配置管理 |
| `winsage sandbox` | 查看沙箱状态 |
| `quit` / `exit` | 退出聊天 |
| `help` | 显示帮助 |

## 获取API密钥

### OpenAI（推荐）
1. 访问: https://platform.openai.com/api-keys
2. 点击 "Create new secret key"
3. 复制密钥到配置

### Anthropic Claude
1. 访问: https://console.anthropic.com/
2. 注册/登录账户
3. 获取API密钥

### Azure OpenAI
1. 在Azure Portal创建资源
2. 部署模型
3. 获取Endpoint和Key

### Ollama（免费本地）
1. 下载安装: https://ollama.ai
2. 运行: `ollama pull llama2`
3. 无需API密钥！

## 启用记忆功能（可选）

```bash
# 使用Docker快速启动PostgreSQL
docker-compose up -d

# 配置会自动连接到:
# postgresql://postgres:postgres@localhost/winsage
```

## 故障排除

### 编译错误？
```powershell
# 确保使用MSVC工具链
rustup default stable-x86_64-pc-windows-msvc

# 清理后重新编译
cargo clean
cargo build --release
```

### 找不到命令？
使用完整路径：
```powershell
.\target\release\winsage.exe
```

### API调用失败？
1. 检查网络连接
2. 验证API密钥
3. 查看详细日志：
   ```powershell
   $env:RUST_LOG = "debug"
   .\target\release\winsage.exe chat
   ```

## 下一步

- 📖 阅读 [README.md](README.md) 了解详细信息
- 💡 查看 [EXAMPLES.md](EXAMPLES.md) 学习高级用法
- 🔧 查看 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解架构设计

---

**有问题？** 查看完整文档或提交Issue！
