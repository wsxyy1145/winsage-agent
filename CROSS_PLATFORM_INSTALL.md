# WinSage 跨平台安装指南

## 🎯 推荐安装方式

### 方式 1: Cargo 安装（跨平台，推荐）

```bash
# Windows / macOS / Linux 通用
cargo install --git https://github.com/wsxyy1145/winsage-agent.git
```

**优点**:
- ✅ 跨平台支持（Windows, macOS, Linux）
- ✅ 自动编译最新版本
- ✅ 无需手动下载
- ✅ 一行命令完成

**缺点**:
- ⏱️ 首次安装需要编译（5-10分钟）

---

### 方式 2: GitHub Releases 下载（最快）

访问最新 Release 页面下载预编译二进制：

```
https://github.com/wsxyy1145/winsage-agent/releases/latest
```

#### Windows
```powershell
# 下载并解压 winsage.exe
# 添加到 PATH 或使用
.\winsage.exe chat
```

#### macOS
```bash
# 下载并解压
chmod +x winsage
./winsage chat
```

#### Linux
```bash
# 下载并解压
chmod +x winsage
./winsage chat
```

---

### 方式 3: Git Clone + 本地编译（开发用）

```bash
# 克隆仓库
git clone https://github.com/wsxyy1145/winsage-agent.git
cd winsage-agent

# 编译
cargo build --release

# 运行
./target/release/winsage chat
```

---

## 📦 未来计划：包管理器支持

### Homebrew (macOS/Linux)

```bash
# 添加 tap
brew tap wsxyy1145/winsage

# 安装
brew install winsage
```

### Scoop (Windows)

```bash
# 添加 bucket
scoop bucket add winsage https://github.com/wsxyy1145/winsage-scoop.git

# 安装
scoop install winsage
```

### Chocolatey (Windows)

```bash
choco install winsage
```

---

## 🔧 系统要求

### Windows
- Windows 10/11 专业版或企业版
- Rust 工具链（使用 cargo install 时）
- Visual Studio Build Tools

### macOS
- macOS 10.15+
- Rust 工具链（使用 cargo install 时）
- Xcode Command Line Tools

### Linux
- Ubuntu 18.04+ / CentOS 7+ / 其他主流发行版
- Rust 工具链
- OpenSSL 开发库

---

## 🚀 快速开始

安装完成后：

```bash
# 运行配置向导
winsage setup

# 开始聊天
winsage chat
```

---

## 📝 版本更新

### Cargo 安装更新
```bash
cargo install --git https://github.com/wsxyy1145/winsage-agent.git --force
```

### Binary 更新
下载最新 Release 替换旧文件即可。
