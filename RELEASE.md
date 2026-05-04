# WinSage v0.1.0 发布说明

## 🎉 首个正式版本

WinSage v0.1.0 是第一个正式发布的版本，为 Windows 用户提供了一个功能完整的终端 AI 助手。

## ✨ 主要特性

### 🤖 多模型支持（8个提供商）

- **OpenAI** - GPT-4, GPT-3.5 Turbo
- **Anthropic** - Claude 3 Opus/Sonnet/Haiku
- **Azure OpenAI** - Azure 部署的 GPT 模型
- **Ollama** - 本地部署的开源模型
- **DeepSeek** ⭐ - 深度求索的高质量模型
- **Kimi (Moonshot)** ⭐ - 月之暗面的超长上下文模型
- **Qwen (通义千问)** ⭐ - 阿里云的强大中文模型
- **Doubao (豆包)** ⭐ - 字节跳动的自然对话模型

### 🔒 安全沙箱

- Windows Sandbox 隔离危险命令执行
- 可配置的超时和自动清理
- 共享文件夹支持

### 💾 长期记忆

- PostgreSQL 数据库存储对话历史
- 知识库管理系统
- 可选的向量搜索支持

### 🎯 交互式 CLI

- 友好的命令行界面
- 彩色输出和进度提示
- 内置帮助系统

### 🚀 简易安装

- 一行命令安装：`npm install -g .`
- 交互式配置向导：`winsage setup`
- 自动依赖检测和安装

## 📦 安装方法

### 前置要求

- Windows 10/11 专业版或企业版
- Rust 工具链（MSVC）
- Visual Studio Build Tools 2019+

### 安装步骤

```bash
# 克隆仓库
git clone https://github.com/your-username/winsage-agent.git
cd winsage-agent

# 全局安装
npm install -g .

# 运行配置向导
winsage setup

# 开始使用
winsage chat
```

## 📋 可用命令

| 命令 | 功能 |
|------|------|
| `winsage` | 启动聊天（默认） |
| `winsage chat` | 启动聊天 |
| `winsage setup` | 运行配置向导 |
| `winsage config` | 管理配置 |
| `winsage sandbox` | 查看沙箱状态 |
| `winsage help` | 显示帮助 |

## 🔧 配置示例

编辑 `%APPDATA%\winsage\config.toml`：

```toml
[general]
default_model = "deepseek-chat"
temperature = 0.7
max_tokens = 4096

[models.deepseek]
api_key = "sk-your-api-key"
base_url = "https://api.deepseek.com/v1"
default_model = "deepseek-chat"
```

## 📚 文档

- [README.md](README.md) - 完整项目文档
- [INSTALL.md](INSTALL.md) - 安装指南
- [SETUP_WIZARD.md](SETUP_WIZARD.md) - 配置向导说明
- [MODELS.md](MODELS.md) - 模型提供商配置
- [EXAMPLES.md](EXAMPLES.md) - 使用示例

## 🐛 已知问题

1. **编译时间较长**: 首次编译需要 5-10 分钟
2. **Windows Sandbox 限制**: 仅专业版和企业版支持
3. **PostgreSQL 需手动安装**: 或使用 Docker 快速部署

## 🔮 未来计划

- [ ] 预编译二进制文件
- [ ] 流式输出支持
- [ ] 更多模型提供商
- [ ] 图形化配置界面
- [ ] 插件系统
- [ ] 自动更新功能

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

感谢以下开源项目：

- Rust 生态系统
- reqwest - HTTP 客户端
- sqlx - 数据库访问
- tokio - 异步运行时
- clap - CLI 框架
- Element Plus - UI 组件库

## 📞 支持

- 提交 Issue: https://github.com/your-username/winsage-agent/issues
- 讨论区: https://github.com/your-username/winsage-agent/discussions

---

**祝您使用愉快！** 🚀
