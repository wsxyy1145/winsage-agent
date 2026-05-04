# WinSage 初始配置向导

## 🚀 快速开始

安装完成后，运行配置向导进行初始设置：

```bash
winsage setup
```

或使用 npm 命令：

```bash
npm run setup
```

## 📋 配置向导功能

配置向导将引导您完成以下设置：

### 1. 选择 AI 模型

支持以下模型提供商：

| 模型 | 类型 | 特点 |
|------|------|------|
| **OpenAI** (GPT-4/GPT-3.5) | 国际 | 最强大的通用AI |
| **DeepSeek** | 中国 ⭐ | 高质量对话和代码生成 |
| **Kimi (Moonshot)** | 中国 ⭐ | 超长上下文(128K) |
| **Qwen (通义千问)** | 中国 ⭐ | 阿里云强大中文能力 |
| **Doubao (豆包)** | 中国 | 字节跳动自然对话 |
| **Anthropic** (Claude) | 国际 | 优秀的长文本理解 |
| **Ollama** | 本地 🆓 | 无需网络，隐私保护 |

⭐ 标记的为热门模型
🆓 标记的无需 API Key（本地部署）

### 2. 输入 API Key

对于云端模型，需要输入相应的 API Key：

- **OpenAI**: [https://platform.openai.com/api-keys](https://platform.openai.com/api-keys)
- **DeepSeek**: [https://platform.deepseek.com/](https://platform.deepseek.com/)
- **Kimi**: [https://platform.moonshot.cn/](https://platform.moonshot.cn/)
- **Qwen**: [https://dashscope.aliyun.com/](https://dashscope.aliyun.com/)
- **Doubao**: [https://www.volcengine.com/](https://www.volcengine.com/)

### 3. 通用配置

- **温度参数** (0.0-1.0): 控制回答的创造性
  - 0.0 - 完全确定性
  - 0.7 - 平衡（默认）
  - 1.0 - 最具创造性

- **最大 Token 数**: 限制单次回复的长度
  - 推荐: 4096

- **流式输出**: 是否逐字显示回复
  - 推荐: true

### 4. 记忆系统（可选）

配置 PostgreSQL 数据库以存储对话历史：

```
postgresql://postgres:postgres@localhost/winsage
```

如果跳过，可以稍后手动配置。

### 5. 沙箱配置（可选）

Windows Sandbox 用于安全执行危险命令：

- 需要 Windows 10/11 专业版或企业版
- 推荐启用以提高安全性

## 💡 使用示例

### 示例 1: 配置 OpenAI

```
请选择您要配置的模型提供商

  1. OpenAI (GPT-4/GPT-3.5) ⭐
  2. DeepSeek (深度求索) ⭐
  3. Kimi (月之暗面) ⭐
  ...

请选择 (输入编号): 1

请输入 API Key: sk-your-openai-key
```

### 示例 2: 配置 DeepSeek

```
请选择 (输入编号): 2

获取 API Key: https://platform.deepseek.com/

请输入 API Key: sk-your-deepseek-key
```

### 示例 3: 使用 Ollama（无需 API Key）

```
请选择 (输入编号): 7

服务地址 (http://localhost:11434):
默认模型 (llama2):
```

## 🔧 高级选项

### 非交互模式

在自动化环境中，可以跳过配置向导：

```bash
npm install -g winsage-agent --non-interactive
```

### 手动编辑配置

配置文件位置：

```
%APPDATA%\winsage\config.toml
```

可以使用任何文本编辑器修改：

```powershell
notepad $env:APPDATA\winsage\config.toml
```

### 使用环境变量

也可以通过环境变量覆盖配置：

```powershell
$env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-your-key"
$env:WINSAGE_GENERAL_DEFAULT_MODEL = "gpt-4"
$env:WINSAGE_GENERAL_TEMPERATURE = "0.8"
```

## ❓ 常见问题

### Q: 我可以配置多个模型吗？

是的！配置文件支持同时配置多个模型，通过修改 `default_model` 切换：

```toml
[general]
default_model = "deepseek-chat"  # 当前使用的模型

[models.openai]
api_key = "sk-openai-key"

[models.deepseek]
api_key = "sk-deepseek-key"
```

### Q: 如何更改默认模型？

编辑配置文件中的 `[general]` 部分：

```toml
[general]
default_model = "qwen-turbo"  # 改为 qwen-turbo
```

### Q: 配置向导可以重新运行吗？

当然！随时运行：

```bash
winsage setup
```

### Q: API Key 安全吗？

是的，API Key 仅存储在本地配置文件中，不会上传到任何服务器。

### Q: 没有配置任何模型怎么办？

默认会使用 Ollama（本地部署）。如果您没有安装 Ollama，需要先配置至少一个云端模型。

## 📚 相关文档

- [README.md](README.md) - 完整项目文档
- [MODELS.md](MODELS.md) - 模型提供商详细配置
- [INSTALL.md](INSTALL.md) - 安装指南
- [EXAMPLES.md](EXAMPLES.md) - 使用示例

---

**祝您使用愉快！** 🎉
