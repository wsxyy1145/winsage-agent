# WinSage 快速安装指南

## 🚀 一行命令完成安装

```bash
npm install -g winsage-agent
```

就这么简单！安装程序会自动处理所有依赖和配置。

## 安装后首次使用

1. **配置 API Key**

   编辑配置文件：`%APPDATA%\winsage\config.toml`

   ```toml
   [models.openai]
   api_key = "sk-your-openai-key"  # 填入您的 OpenAI API Key
   ```

   或使用环境变量：
   ```powershell
   $env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-your-key"
   ```

2. **启动聊天**

   ```bash
   winsage chat
   ```

## 常用命令

```bash
winsage              # 启动聊天（默认）
winsage chat         # 启动聊天
winsage config       # 管理配置
winsage sandbox      # 查看沙箱状态
winsage help         # 显示帮助
```

## 系统要求

- **操作系统**: Windows 10/11 专业版或企业版
- **Node.js**: >= 14.0.0（用于安装器）
- **磁盘空间**: 约 500MB（包含编译产物）

安装程序会自动检查和安装：
- ✓ Rust 工具链
- ✓ Visual Studio Build Tools（如需要会提示手动安装）
- ✓ PostgreSQL（可选，可通过 Docker 部署）

## 故障排除

### 安装失败

如果 `npm install -g` 失败，请尝试：

```powershell
# 以管理员身份运行 PowerShell
npm install -g winsage-agent --force
```

### Rust 安装问题

手动安装 Rust：
```powershell
winget install Rustlang.Rust.MSVC
```

### Visual Studio Build Tools 缺失

访问 https://visualstudio.microsoft.com/visual-cpp-build-tools/ 下载并安装，选择 "C++ build tools" 工作负载。

### 编译缓慢

首次编译可能需要 5-10 分钟，这是正常的。后续更新会快很多。

## 卸载

```bash
npm uninstall -g winsage-agent
```

删除配置目录（可选）：
```powershell
Remove-Item -Recurse -Force $env:APPDATA\winsage
```

## 更多帮助

- 查看完整文档：[README.md](README.md)
- 示例用法：[EXAMPLES.md](EXAMPLES.md)
- 快速开始：[QUICKSTART.md](QUICKSTART.md)
