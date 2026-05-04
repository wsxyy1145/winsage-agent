# 📦 创建 GitHub Release 完整指南

## 🎯 方法选择

### 方法 1: 使用 Git Tag 自动发布（推荐）

通过创建 Git tag 触发自动化构建和发布。

### 方法 2: 手动创建 Release

在 GitHub 网页上手动上传文件。

---

## 🚀 方法 1: 自动发布（推荐）

### 步骤 1: 更新版本号

编辑 `Cargo.toml`:

```toml
[package]
name = "winsage"
version = "0.1.0"  # 修改这里
```

### 步骤 2: 提交更改

```bash
git add Cargo.toml
git commit -m "Bump version to v0.1.0"
git push
```

### 步骤 3: 创建并推送 Tag

```bash
# 创建 tag
git tag -a v0.1.0 -m "WinSage v0.1.0 - Initial Release"

# 推送 tag 到 GitHub（会触发自动构建）
git push origin v0.1.0
```

### 步骤 4: 等待构建完成

访问 Actions 页面查看进度：
```
https://github.com/wsxyy1145/winsage-agent/actions
```

构建完成后会自动创建 Release！

---

## 📝 方法 2: 手动创建 Release

### 步骤 1: 本地编译所有平台

#### Windows
```bash
cargo build --release
# 输出: target/release/winsage.exe
```

#### macOS (需要 macOS 机器)
```bash
cargo build --release
# 输出: target/release/winsage
```

#### Linux (需要 Linux 机器或 Docker)
```bash
docker run --rm -v "$(pwd)":/app -w /app rust:latest \
  cargo build --release
# 输出: target/release/winsage
```

### 步骤 2: 重命名文件

```
winsage.exe              → winsage-windows-x64.exe
winsage (macOS Intel)    → winsage-macos-x64
winsage (macOS ARM)      → winsage-macos-arm64
winsage (Linux)          → winsage-linux-x64
```

### 步骤 3: 在 GitHub 创建 Release

1. **访问 Releases 页面**:
   ```
   https://github.com/wsxyy1145/winsage-agent/releases/new
   ```

2. **填写发布信息**:
   - **Tag version**: `v0.1.0`
   - **Target**: `master` (或 main)
   - **Release title**: `WinSage v0.1.0 - Initial Release`

3. **编写描述** (复制以下模板):

```markdown
## 🎉 WinSage v0.1.0

首个正式发布的版本，为 Windows/macOS/Linux 用户提供功能完整的终端 AI 助手。

## ✨ 主要特性

- 8个AI模型提供商支持 (OpenAI, Claude, DeepSeek, Kimi, Qwen, Doubao, Azure, Ollama)
- Windows Sandbox 安全隔离
- PostgreSQL 长期记忆系统
- 交互式配置向导
- 跨平台支持 (Windows/macOS/Linux)

## 📦 安装

### 预编译二进制（推荐）

下载下方对应平台的文件：
- **Windows**: `winsage-windows-x64.exe`
- **macOS Intel**: `winsage-macos-x64`
- **macOS Apple Silicon**: `winsage-macos-arm64`
- **Linux**: `winsage-linux-x64`

### 使用方法

**Windows**:
```powershell
# 重命名为 winsage.exe
.\winsage.exe chat
```

**macOS/Linux**:
```bash
chmod +x winsage-*
./winsage-* chat
```

### 或使用 Cargo 安装

```bash
cargo install --git https://github.com/wsxyy1145/winsage-agent.git
winsage chat
```

## 🔧 系统要求

### Windows
- Windows 10/11 专业版或企业版
- Visual Studio Build Tools (仅源码安装需要)

### macOS
- macOS 10.15+

### Linux
- Ubuntu 18.04+ / CentOS 7+

## 📚 文档

- [README](https://github.com/wsxyy1145/winsage-agent/blob/master/README.md)
- [安装指南](https://github.com/wsxyy1145/winsage-agent/blob/master/CROSS_PLATFORM_INSTALL.md)
- [配置向导](https://github.com/wsxyy1145/winsage-agent/blob/master/SETUP_WIZARD.md)

## 🐛 已知问题

- Windows Sandbox 仅在 Windows 专业版/企业版可用
- macOS 和 Linux 版本的沙箱功能暂未实现

## 🙏 致谢

感谢所有贡献者和开源社区！
```

4. **上传文件**:
   - 拖拽或点击 "Attach binaries by dropping them here"
   - 上传所有平台的二进制文件

5. **发布**:
   - 勾选 ✅ "Set as the latest release"
   - 点击 "Publish release"

---

## 🔄 后续版本更新

### 小版本更新 (0.1.0 → 0.1.1)

```bash
# 修改 Cargo.toml version = "0.1.1"
git add Cargo.toml
git commit -m "Bump version to v0.1.1"
git tag -a v0.1.1 -m "Bug fixes and improvements"
git push origin v0.1.1
```

### 大版本更新 (0.1.x → 1.0.0)

```bash
# 修改 Cargo.toml version = "1.0.0"
git add Cargo.toml
git commit -m "Release v1.0.0"
git tag -a v1.0.0 -m "Major release with new features"
git push origin v1.0.0
```

---

## 📊 查看 Release 统计

访问：
```
https://github.com/wsxyy1145/winsage-agent/releases
```

可以看到：
- 下载次数统计
- 所有历史版本
- 用户反馈和 Issue

---

## 🎯 快速命令参考

```bash
# 查看当前 tag
git tag

# 创建新 tag
git tag -a v0.1.0 -m "Release message"

# 推送 tag
git push origin v0.1.0

# 删除 tag (如果需要)
git tag -d v0.1.0
git push origin :refs/tags/v0.1.0
```

---

**推荐使用 Tag 自动发布方式，省时省力！** 🚀
