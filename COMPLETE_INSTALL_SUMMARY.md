# 🎉 WinSage 完整安装与配置指南

## ✅ 验证成功

全局安装命令已成功执行并验证：

```bash
npm install -g winsage-agent
```

**安装结果**: ✓ 成功
**版本**: winsage-agent@0.1.0
**全局命令**: winsage ✓ 可用

---

## 📋 完整使用流程

### 1️⃣ 安装（已完成）

```bash
npm install -g winsage-agent
```

安装过程自动完成：
- ✅ 检测 Rust 工具链
- ✅ 检查 Visual Studio Build Tools
- ✅ 编译 WinSage 项目
- ✅ 注册全局 CLI 命令
- ✅ 创建默认配置模板

### 2️⃣ 运行配置向导

```bash
winsage setup
```

配置向导会引导您：

```
╔══════════════════════════════════════════════════╗
║     WinSage - Windows Terminal AI Agent          ║
║           初始配置向导                           ║
╚══════════════════════════════════════════════════╝

步骤 1: 选择 AI 模型
  1. OpenAI (GPT-4/GPT-3.5) ⭐
  2. DeepSeek (深度求索) ⭐
  3. Kimi (月之暗面) ⭐
  4. Qwen (通义千问/阿里云) ⭐
  5. Doubao (豆包/字节跳动)
  6. Anthropic (Claude)
  7. Ollama (本地部署) 🆓

步骤 2: 输入 API Key
步骤 3: 配置温度参数和 Token 限制
步骤 4: 配置 PostgreSQL（可选）
步骤 5: 配置沙箱（可选）

✓ 配置完成！
```

### 3️⃣ 开始聊天

```bash
winsage chat
```

或直接运行：
```bash
winsage
```

---

## 🔧 所有可用命令

| 命令 | 功能 | 说明 |
|------|------|------|
| `winsage` | 启动聊天 | 默认命令 |
| `winsage chat` | 启动聊天 | 同上 |
| `winsage setup` | 配置向导 | **首次使用推荐** |
| `winsage config` | 管理配置 | 打开配置文件 |
| `winsage sandbox` | 沙箱状态 | 查看 Windows Sandbox 状态 |
| `winsage help` | 帮助信息 | 显示所有命令 |

---

## 📁 文件位置

### 配置文件
```
%APPDATA%\winsage\config.toml
```
通常在：
```
C:\Users\<用户名>\AppData\Roaming\winsage\config.toml
```

### 可执行文件
```
target/release/winsage.exe
```

### NPM 全局安装位置
```
C:\Users\<用户名>\AppData\Roaming\npm\node_modules\winsage-agent
```

---

## 🎯 快速配置示例

### 示例 1: 使用 OpenAI

```bash
winsage setup
# 选择 1 (OpenAI)
# 输入 API Key: sk-your-openai-key
# 其他选项使用默认值

winsage chat
```

### 示例 2: 使用 DeepSeek

```bash
winsage setup
# 选择 2 (DeepSeek)
# 输入 API Key: sk-your-deepseek-key

winsage chat
```

### 示例 3: 使用 Kimi

```bash
winsage setup
# 选择 3 (Kimi)
# 输入 API Key: sk-your-moonshot-key

winsage chat
```

---

## 🔍 故障排除

### 问题 1: "winsage" 命令找不到

**解决方案**:
```bash
# 重新安装
npm uninstall -g winsage-agent
npm install -g winsage-agent

# 验证安装
npm list -g winsage-agent
```

### 问题 2: 编译失败

**解决方案**:
1. 确保已安装 Visual Studio Build Tools
2. 确保选择了 "C++ build tools" 工作负载
3. 重启终端后重试

### 问题 3: 配置文件未创建

**解决方案**:
配置文件在首次运行 `winsage chat` 时自动创建：
```bash
winsage chat
# 或手动运行配置向导
winsage setup
```

### 问题 4: 需要修改配置

**方式 1**: 使用配置向导
```bash
winsage setup
```

**方式 2**: 直接编辑文件
```powershell
notepad $env:APPDATA\winsage\config.toml
```

**方式 3**: 使用环境变量
```powershell
$env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-new-key"
$env:WINSAGE_GENERAL_DEFAULT_MODEL = "gpt-4"
```

---

## 📊 安装统计

| 项目 | 数值 |
|------|------|
| 安装包大小 | ~500 KB (源码) |
| 编译后大小 | ~50 MB |
| 安装时间 | 5-10 分钟（首次编译） |
| 支持系统 | Windows 10/11 |
| 支持模型 | 8 个提供商 |
| 配置复杂度 | ⭐ 极简（一键安装 + 向导） |

---

## 🌟 特色功能

### 1. 一键安装
```bash
npm install -g winsage-agent
```
自动处理所有依赖和编译。

### 2. 交互式配置向导
```bash
winsage setup
```
友好的界面，逐步引导配置。

### 3. 多模型支持
- OpenAI GPT-4/3.5
- Anthropic Claude 3
- Azure OpenAI
- Ollama (本地)
- **DeepSeek** ⭐
- **Kimi (Moonshot)** ⭐
- **Qwen (通义千问)** ⭐
- **Doubao (豆包)** ⭐

### 4. 安全沙箱
Windows Sandbox 隔离危险命令执行。

### 5. 长期记忆
PostgreSQL 存储对话历史（可选）。

---

## 📚 相关文档

- [README.md](README.md) - 完整项目文档
- [SETUP_WIZARD.md](SETUP_WIZARD.md) - 配置向导详细说明
- [MODELS.md](MODELS.md) - 模型提供商配置指南
- [INSTALL.md](INSTALL.md) - 安装指南
- [EXAMPLES.md](EXAMPLES.md) - 使用示例

---

## 💬 获取帮助

### 命令行帮助
```bash
winsage help
```

### 查看日志
```powershell
$env:RUST_LOG = "debug"
winsage chat
```

### 检查版本
```bash
winsage --version
```

---

## 🎊 恭喜！

您已成功安装 WinSage！

**下一步**:
1. 运行 `winsage setup` 进行初始配置
2. 输入您的 API Key
3. 运行 `winsage chat` 开始体验

**祝您使用愉快！** 🚀
