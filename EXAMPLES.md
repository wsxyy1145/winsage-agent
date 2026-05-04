# WinSage 使用示例

## 快速开始

### 1. 首次配置

```bash
# 启动配置向导
winsage config
```

按照提示输入API密钥和数据库连接信息。

### 2. 开始聊天

```bash
# 启动交互式聊天
winsage chat

# 或直接运行（默认进入聊天模式）
winsage
```

### 3. 与AI对话

```
你> 你好！请介绍一下你自己

助手思考中...

助手
你好！我是WinSage AI助手，一个运行在Windows终端的智能助手。我可以帮助你：
- 回答问题和提供信息
- 执行安全的系统命令
- 管理文件和目录
- 编写代码和脚本
- 以及其他各种任务

有什么我可以帮助你的吗？
```

## 高级用法

### 使用不同的模型

编辑配置文件 `%APPDATA%\winsage\config.toml`:

```toml
[general]
# 切换到 Claude
default_model = "claude-3-opus-20240229"

# 或使用本地 Ollama
# default_model = "llama2"
```

### 使用环境变量

```powershell
# PowerShell
$env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-your-key"
winsage chat
```

```bash
# CMD
set WINSAGE_MODELS_OPENAI_API_KEY=sk-your-key
winsage chat
```

### 禁用沙箱（不推荐）

```toml
[sandbox]
enabled = false
```

## API 密钥获取

### OpenAI
1. 访问 https://platform.openai.com/api-keys
2. 创建新的API密钥
3. 复制到配置中

### Anthropic
1. 访问 https://console.anthropic.com/
2. 获取API密钥
3. 复制到配置中

### Azure OpenAI
1. 在Azure Portal创建OpenAI资源
2. 获取Endpoint和API密钥
3. 配置deployment名称

### Ollama（本地）
1. 安装 Ollama: https://ollama.ai
2. 拉取模型: `ollama pull llama2`
3. 无需API密钥

## 记忆系统

### 启动 PostgreSQL

```bash
# 使用 Docker
docker-compose up -d

# 查看日志
docker logs winsage-postgres
```

### 查询对话历史

直接连接到PostgreSQL查询：

```sql
-- 查看所有对话
SELECT id, title, created_at FROM conversations ORDER BY created_at DESC;

-- 查看特定对话的消息
SELECT role, content, timestamp
FROM messages
WHERE conversation_id = 'your-conversation-id'
ORDER BY timestamp;
```

## 沙箱示例

### 安全命令（直接执行）
```
你> 列出当前目录的文件

助手
dir
```

### 危险命令（沙箱执行）
```
你> 删除所有临时文件

助手
[自动在沙箱中执行] del /q %TEMP%\*.*
```

## 故障排除

### 问题: 无法连接到OpenAI

检查:
1. API密钥是否正确
2. 网络连接是否正常
3. 是否有代理设置

```toml
# 如需使用代理
[models.openai]
base_url = "https://api.openai.com/v1"
# 设置环境变量 http_proxy 和 https_proxy
```

### 问题: 数据库连接失败

检查:
1. PostgreSQL是否运行
2. 连接字符串是否正确
3. 数据库是否已创建

```bash
# 测试连接
psql postgresql://postgres:postgres@localhost/winsage
```

### 问题: Windows Sandbox不可用

检查:
1. Windows版本是否为专业版/企业版
2. 是否启用了虚拟化
3. 在"启用或关闭Windows功能"中启用Windows Sandbox

## 自定义系统提示

编辑配置文件添加自定义提示：

```toml
[agent]
system_prompt = """
你是一个专业的Python开发工程师。
擅长编写清晰、高效的Python代码。
总是包含详细的注释和文档字符串。
"""
```

## 批量处理

可以通过管道输入：

```bash
echo "解释量子计算" | winsage
```

## 日志调试

```powershell
# 设置详细日志
$env:RUST_LOG = "winsage=debug"
winsage chat
```

日志输出包含：
- API请求详情
- 数据库操作
- 沙箱执行状态
- 错误堆栈跟踪
