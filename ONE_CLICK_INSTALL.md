# 🚀 WinSage 一行命令安装

## 超级简单的安装方式

```bash
npm install -g winsage-agent
```

**就这么简单！** 安装程序会自动完成：
- ✅ 检测和安装 Rust 工具链
- ✅ 检查 Visual Studio Build Tools
- ✅ 编译 WinSage 项目
- ✅ 创建全局 CLI 命令
- ✅ 生成默认配置文件

## 安装后立即使用

```bash
winsage chat
```

## 完整的安装流程说明

### 1️⃣ 安装前准备

确保您的系统满足以下要求：
- Windows 10/11 专业版或企业版
- Node.js >= 14.0.0（用于运行安装器）
- 约 500MB 磁盘空间

### 2️⃣ 执行安装

在 PowerShell 或 CMD 中运行：

```bash
npm install -g winsage-agent
```

安装过程大约需要 5-10 分钟（首次编译 Rust 代码）。

### 3️⃣ 配置 API Key

安装完成后，编辑配置文件：

```
%APPDATA%\winsage\config.toml
```

添加您的 OpenAI API Key：

```toml
[models.openai]
api_key = "sk-your-openai-key"
```

或使用环境变量：

```powershell
$env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-your-key"
```

### 4️⃣ 开始使用

```bash
winsage              # 启动聊天
winsage config       # 管理配置
winsage sandbox      # 查看沙箱状态
winsage help         # 显示帮助
```

## 常见问题

### Q: 安装很慢怎么办？
A: 首次安装需要编译 Rust 代码，大约需要 5-10 分钟，这是正常的。后续更新会快很多。

### Q: 需要手动安装 Rust 吗？
A: 不需要！安装程序会自动检测和安装 Rust。如果自动安装失败，可以手动从 https://rustup.rs 安装。

### Q: PostgreSQL 是必需的吗？
A: 不是必需的。PostgreSQL 用于长期记忆功能，您可以稍后根据需要再安装。

### Q: 如何卸载？
A: 运行 `npm uninstall -g winsage-agent` 即可。

## 高级选项

### 非交互模式安装

```bash
npm install -g winsage-agent --non-interactive
```

### 使用 Docker 部署 PostgreSQL

```bash
docker run --name winsage-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=winsage \
  -p 5432:5432 \
  -d postgres:16
```

### 自定义配置

配置文件位置：`%APPDATA%\winsage\config.toml`

完整配置示例请参考 [README.md](README.md)

## 技术支持

- 📖 完整文档：[README.md](README.md)
- 💡 使用示例：[EXAMPLES.md](EXAMPLES.md)
- ⚡ 快速开始：[QUICKSTART.md](QUICKSTART.md)
- 📝 安装指南：[INSTALL.md](INSTALL.md)

---

**就是这么简单！一条命令，即刻体验 WinSage 的强大功能！** 🎉
