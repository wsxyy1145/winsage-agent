# 模型提供商配置指南

WinSage 支持多种 AI 模型提供商，您可以根据需要选择并配置。

## 支持的模型提供商

| 提供商 | 模型示例 | API 类型 | 地区 |
|--------|---------|---------|------|
| OpenAI | gpt-4, gpt-3.5-turbo | OpenAI兼容 | 国际 |
| Anthropic | claude-3-opus, claude-3-sonnet | 原生API | 国际 |
| Azure | gpt-4, gpt-35-turbo | Azure OpenAI | 国际 |
| Ollama | llama2, mistral | 本地部署 | 本地 |
| **DeepSeek** | deepseek-chat, deepseek-coder | OpenAI兼容 | 中国 |
| **Kimi (Moonshot)** | moonshot-v1-8k, moonshot-v1-32k | OpenAI兼容 | 中国 |
| **Qwen (通义千问)** | qwen-turbo, qwen-plus, qwen-max | DashScope | 中国 |
| **Doubao (豆包)** | ep-xxxxx | OpenAI兼容 | 中国 |

---

## DeepSeek 配置

DeepSeek 是一家专注于通用人工智能的中国公司，提供高质量的对话和代码生成模型。

### 获取 API Key

1. 访问 [DeepSeek 平台](https://platform.deepseek.com/)
2. 注册/登录账号
3. 在控制台获取 API Key

### 配置示例

```toml
[models.deepseek]
api_key = "sk-your-deepseek-key"
base_url = "https://api.deepseek.com/v1"
default_model = "deepseek-chat"
```

### 可用模型

- `deepseek-chat` - 通用对话模型
- `deepseek-coder` - 代码专用模型

### 使用环境变量

```powershell
$env:WINSAGE_MODELS_DEEPSEEK_API_KEY = "sk-your-key"
```

---

## Kimi (Moonshot) 配置

Kimi 是由月之暗面（Moonshot AI）开发的智能助手，擅长长文本处理和多轮对话。

### 获取 API Key

1. 访问 [Moonshot 开放平台](https://platform.moonshot.cn/)
2. 注册/登录账号
3. 在控制台创建 API Key

### 配置示例

```toml
[models.moonshot]
api_key = "sk-your-moonshot-key"
base_url = "https://api.moonshot.cn"
default_model = "moonshot-v1-8k"
```

### 可用模型

- `moonshot-v1-8k` - 支持 8K 上下文
- `moonshot-v1-32k` - 支持 32K 上下文
- `moonshot-v1-128k` - 支持 128K 超长上下文

### 使用环境变量

```powershell
$env:WINSAGE_MODELS_MOONSHOT_API_KEY = "sk-your-key"
```

---

## Qwen (通义千问) 配置

Qwen 是阿里云开发的大语言模型系列，提供强大的中文理解和生成能力。

### 获取 API Key

1. 访问 [阿里云 DashScope 平台](https://dashscope.aliyun.com/)
2. 注册/登录阿里云账号
3. 开通 DashScope 服务
4. 在控制台获取 API Key

### 配置示例

```toml
[models.qwen]
api_key = "your-qwen-api-key"
base_url = "https://dashscope.aliyuncs.com"
default_model = "qwen-turbo"
```

### 可用模型

- `qwen-turbo` - 快速响应模型
- `qwen-plus` - 均衡性能模型
- `qwen-max` - 最强性能模型
- `qwen-long` - 长文本处理模型

### 使用环境变量

```powershell
$env:WINSAGE_MODELS_QWEN_API_KEY = "your-api-key"
```

---

## Doubao (豆包) 配置

Doubao 是字节跳动开发的AI助手，提供自然流畅的对话体验。

### 获取 API Key

1. 访问 [火山引擎平台](https://www.volcengine.com/)
2. 注册/登录账号
3. 开通方舟大模型服务
4. 在控制台获取 API Key 和 Endpoint ID

### 配置示例

```toml
[models.doubao]
api_key = "your-doubao-api-key"
base_url = "https://ark.cn-beijing.volces.com"
default_model = "ep-xxxxx"  # 替换为您的 endpoint ID
```

### 注意事项

- `default_model` 需要替换为您在火山引擎控制台创建的 Endpoint ID
- Endpoint ID 格式通常为 `ep-xxxxx`

### 使用环境变量

```powershell
$env:WINSAGE_MODELS_DOUBAO_API_KEY = "your-api-key"
```

---

## 切换模型

要切换使用的模型，修改配置文件中的 `default_model` 字段：

```toml
[general]
default_model = "deepseek-chat"  # 切换到 DeepSeek
# default_model = "moonshot-v1-8k"  # 或使用 Kimi
# default_model = "qwen-turbo"      # 或使用 Qwen
# default_model = "ep-xxxxx"        # 或使用 Doubao
```

或者通过环境变量指定：

```powershell
$env:WINSAGE_GENERAL_DEFAULT_MODEL = "deepseek-chat"
```

---

## 多模型同时配置

您可以同时配置多个模型，WinSage 会根据 `default_model` 的设置自动选择：

```toml
[models.openai]
api_key = "sk-openai-key"
base_url = "https://api.openai.com/v1"
default_model = "gpt-4"

[models.deepseek]
api_key = "sk-deepseek-key"
base_url = "https://api.deepseek.com/v1"
default_model = "deepseek-chat"

[models.moonshot]
api_key = "sk-moonshot-key"
base_url = "https://api.moonshot.cn"
default_model = "moonshot-v1-8k"
```

当 `default_model` 设置为 `gpt-4` 时使用 OpenAI，设置为 `deepseek-chat` 时使用 DeepSeek。

---

## 常见问题

### Q: 如何选择最适合我的模型？

- **国际用户**: OpenAI GPT-4、Claude 3
- **中国用户**: DeepSeek、Kimi、Qwen、Doubao（访问速度更快）
- **本地部署**: Ollama（无需网络连接）
- **代码任务**: DeepSeek Coder、GPT-4
- **长文本**: Kimi (128K)、Qwen Long

### Q: API Key 安全吗？

API Key 存储在本地配置文件中，不会上传到任何地方。建议：
- 不要将配置文件提交到版本控制
- 使用环境变量管理敏感信息

### Q: 如何测试连接？

运行以下命令启动聊天，如果能正常回复说明配置正确：

```bash
winsage chat
```

### Q: 出现 "API 请求失败" 怎么办？

1. 检查 API Key 是否正确
2. 确认账户余额充足
3. 检查网络连接
4. 查看日志：`$env:RUST_LOG="debug"; winsage chat`
