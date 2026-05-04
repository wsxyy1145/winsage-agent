# WinSage macOS 支持计划

## 🎯 当前状态

WinSage 目前主要针对 Windows 平台开发，但代码架构已为跨平台支持做好准备。

## ✅ 已完成的跨平台准备

### 1. 代码层面

- ✅ 使用 Rust 标准库（天然跨平台）
- ✅ 异步运行时 tokio（跨平台）
- ✅ HTTP 客户端 reqwest（跨平台）
- ✅ 数据库 sqlx（跨平台）
- ✅ 配置文件 TOML（跨平台）

### 2. 依赖检查

所有依赖都支持 macOS：

| 依赖 | Windows | macOS | Linux |
|------|---------|-------|-------|
| tokio | ✅ | ✅ | ✅ |
| clap | ✅ | ✅ | ✅ |
| reqwest | ✅ | ✅ | ✅ |
| serde | ✅ | ✅ | ✅ |
| sqlx | ✅ | ✅ | ✅ |
| tracing | ✅ | ✅ | ✅ |
| config | ✅ | ✅ | ✅ |

### 3. 平台特定代码

需要适配的部分：

#### Windows Sandbox (src/sandbox/)
```rust
// 当前仅 Windows
#[cfg(target_os = "windows")]
pub mod windows_sandbox;

// macOS 需要实现
#[cfg(target_os = "macos")]
pub mod macos_sandbox;

// Linux 需要实现
#[cfg(target_os = "linux")]
pub mod linux_sandbox;
```

---

## 📋 macOS 实现清单

### 阶段 1: 基础功能（高优先级）

- [ ] 移除 Windows 特定依赖
- [ ] 条件编译沙箱模块
- [ ] 实现 macOS 配置路径
- [ ] 测试基本聊天功能
- [ ] 更新 README 添加 macOS 说明

### 阶段 2: 沙箱支持（中优先级）

- [ ] 研究 macOS Sandbox API
- [ ] 实现 macOS 沙箱隔离
- [ ] 或使用 Docker 作为替代方案
- [ ] 安全策略配置

### 阶段 3: 发布支持（低优先级）

- [ ] 配置 GitHub Actions CI/CD
- [ ] 自动构建 macOS binary
- [ ] 创建 Homebrew formula
- [ ] 发布预编译包

---

## 🔧 快速启用 macOS 支持

### 1. 修改 Cargo.toml

添加平台特定依赖：

```toml
[target.'cfg(windows)'.dependencies]
# Windows 特定依赖（如果有）

[target.'cfg(target_os = "macos")'.dependencies]
# macOS 特定依赖（如果需要）

[target.'cfg(target_os = "linux")'.dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```

### 2. 条件编译沙箱

```rust
// src/sandbox/mod.rs
#[cfg(target_os = "windows")]
pub use windows_sandbox::WindowsSandbox;

#[cfg(target_os = "macos")]
pub use macos_sandbox::MacOSSandbox;

#[cfg(target_os = "linux")]
pub use linux_sandbox::LinuxSandbox;
```

### 3. 配置目录适配

```rust
// src/config/settings.rs
fn get_config_dir() -> Result<PathBuf, ConfigError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| ConfigError::DirectoryError("无法获取配置目录".to_string()))?
        .join("winsage");

    // macOS: ~/Library/Application Support/winsage
    // Linux: ~/.config/winsage
    // Windows: %APPDATA%\winsage

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
}
```

---

## 🚀 在 macOS 上测试

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 克隆并编译

```bash
git clone https://github.com/wsxyy1145/winsage-agent.git
cd winsage-agent
cargo build --release
```

### 运行

```bash
./target/release/winsage chat
```

---

## 📦 macOS 分发方式

### 1. Homebrew (推荐)

创建 `winsage.rb` formula:

```ruby
class Winsage < Formula
  desc "Windows Terminal AI Agent (cross-platform)"
  homepage "https://github.com/wsxyy1145/winsage-agent"
  url "https://github.com/wsxyy1145/winsage-agent/archive/v0.1.0.tar.gz"
  sha256 "..."

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end
end
```

用户安装：
```bash
brew install wsxyy1145/tap/winsage
```

### 2. 预编译 Binary

GitHub Actions 自动构建：

```yaml
name: Build macOS
on: [push]
jobs:
  build:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
        with:
          name: winsage-macos
          path: target/release/winsage
```

---

## 🐛 已知问题

### Windows 特有问题

1. **Windows Sandbox**: 仅 Windows 专业版/企业版支持
   - macOS 替代方案: App Sandbox 或 Docker

2. **Visual Studio Build Tools**: 仅 Windows 需要
   - macOS 使用 Xcode Command Line Tools

3. **注册表访问**: 如果使用
   - macOS 使用 plist 文件

---

## 📊 时间估算

| 阶段 | 工作量 | 预计时间 |
|------|--------|---------|
| 基础功能适配 | 小 | 1-2 天 |
| 沙箱实现 | 中 | 3-5 天 |
| 测试和优化 | 中 | 2-3 天 |
| 发布准备 | 小 | 1 天 |
| **总计** | | **1-2 周** |

---

## 🎯 下一步行动

1. **立即可以做**:
   - 在 macOS 上测试编译
   - 修复编译错误
   - 验证基本功能

2. **短期计划**:
   - 条件编译 Windows 特定代码
   - 添加 CI/CD macOS 构建
   - 发布 macOS binary

3. **长期计划**:
   - 完整沙箱支持
   - Homebrew 发布
   - 官方 macOS 支持声明

---

**结论**: WinSage 的代码架构已经为跨平台做好准备，启用 macOS 支持相对简单，主要工作是条件编译和沙箱适配。
