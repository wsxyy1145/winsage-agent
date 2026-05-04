# 🚀 WinSage 快速参考卡

## 一行命令安装

```bash
npm install -g winsage-agent
```

## 立即使用

```bash
winsage chat
```

## 常用命令速查

| 命令 | 功能 |
|------|------|
| `winsage` | 启动聊天（默认） |
| `winsage chat` | 启动聊天 |
| `winsage config` | 管理配置 |
| `winsage sandbox` | 查看沙箱状态 |
| `winsage help` | 显示帮助 |

## 配置文件位置

```
%APPDATA%\winsage\config.toml
```

## 环境变量

```powershell
$env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-your-key"
$env:RUST_LOG = "debug"  # 调试模式
```

## 故障排除

### 重新安装
```bash
npm uninstall -g winsage-agent
npm install -g winsage-agent
```

### 清理配置
```powershell
Remove-Item -Recurse -Force $env:APPDATA\winsage
```

### 查看日志
```bash
$env:RUST_LOG = "debug"
winsage chat
```

## 系统要求

- ✅ Windows 10/11 专业版或企业版
- ✅ Node.js >= 14.0.0
- ✅ 约 500MB 磁盘空间

## 文档链接

- 📖 [README.md](README.md) - 完整文档
- ⚡ [ONE_CLICK_INSTALL.md](ONE_CLICK_INSTALL.md) - 安装指南
- 💡 [EXAMPLES.md](EXAMPLES.md) - 使用示例
- 🔧 [INSTALLATION_GUIDE.md](INSTALLATION_GUIDE.md) - 技术说明

---

**就是这么简单！** 🎉
