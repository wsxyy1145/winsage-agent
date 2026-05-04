# WinSage 一键安装实现说明

## 概述

本项目现已支持通过一条 npm 命令完成全部安装流程，大大降低了使用门槛。

## 实现架构

### 1. package.json - NPM 包配置

**文件位置**: `package.json`

**关键配置**:
```json
{
  "name": "winsage-agent",
  "version": "0.1.0",
  "bin": {
    "winsage": "./bin/winsage.js"
  },
  "scripts": {
    "postinstall": "node scripts/install.js --non-interactive"
  }
}
```

**功能说明**:
- `bin` 字段定义全局 CLI 命令 `winsage`
- `postinstall` 脚本在安装时自动执行安装程序
- `--non-interactive` 参数使安装过程无需用户交互

### 2. install.js - 自动化安装脚本

**文件位置**: `scripts/install.js`

**核心功能**:

#### a) 系统检测
```javascript
checkWindows()           // 检查是否为 Windows 系统
checkRust()              // 检查 Rust 是否已安装
checkVisualStudioBuildTools()  // 检查 VS Build Tools
checkPostgreSQL()        // 检查 Docker/PostgreSQL
```

#### b) 自动安装
```javascript
installRust()            // 使用 winget 安装 Rust
installVisualStudioBuildTools()  // 提示安装 VS Build Tools
setupPostgreSQL()        // 可选：使用 Docker 部署 PostgreSQL
```

#### c) 项目编译
```javascript
buildProject()           // 执行 cargo build --release
```

#### d) 配置生成
```javascript
setupConfig()            // 创建默认配置文件到 %APPDATA%\winsage
```

### 3. winsage.js - CLI 入口点

**文件位置**: `bin/winsage.js`

**功能**:
- 查找编译后的可执行文件（target/release/winsage.exe）
- 将 npm 命令参数传递给 Rust 程序
- 提供友好的帮助信息

**命令映射**:
```javascript
winsage              →  winsage.exe chat
winsage chat         →  winsage.exe chat
winsage config       →  winsage.exe config
winsage sandbox      →  winsage.exe sandbox
winsage help         →  显示帮助信息
```

## 安装流程

```
用户执行: npm install -g winsage-agent
                ↓
        NPM 下载包
                ↓
        执行 postinstall 脚本
                ↓
    ┌───────────────────────────┐
    │   install.js 开始运行     │
    └───────────────────────────┘
                ↓
    检查系统要求 (Windows, Rust, VS Build Tools)
                ↓
    自动安装缺失的依赖
                ↓
    编译 Rust 项目 (cargo build --release)
                ↓
    生成默认配置文件
                ↓
    注册全局 CLI 命令
                ↓
        安装完成！
                ↓
    用户可以立即使用: winsage chat
```

## 文件结构

```
WinSage-agent/
├── package.json              # NPM 包配置
├── scripts/
│   └── install.js           # 自动化安装脚本
├── bin/
│   └── winsage.js          # CLI 入口点
├── INSTALL.md               # 快速安装指南
├── ONE_CLICK_INSTALL.md     # 一行命令安装说明
└── README.md                # 完整文档（已更新）
```

## 使用方式对比

### 之前的安装方式（复杂）

```powershell
# 1. 安装 Rust
winget install Rustlang.Rust.MSVC

# 2. 下载安装 VS Build Tools
# 访问 https://visualstudio.microsoft.com/...

# 3. 克隆项目
git clone <url>
cd WinSage-agent

# 4. 编译
cargo build --release

# 5. 手动配置
# 创建配置文件...

# 6. 运行
.\target\release\winsage.exe chat
```

### 现在的安装方式（简单）

```bash
# 一条命令搞定！
npm install -g winsage-agent

# 立即使用
winsage chat
```

## 技术亮点

### 1. 智能依赖检测
- 自动检测已安装的组件
- 只安装缺失的依赖
- 避免重复安装

### 2. 非交互模式支持
- 支持 CI/CD 环境
- 可通过 `--non-interactive` 参数启用
- 适合自动化部署

### 3. 跨平台兼容
- 虽然仅支持 Windows，但使用了跨平台的 Node.js
- 彩色终端输出，提升用户体验
- 清晰的进度提示

### 4. 优雅的错误处理
- 检测失败时给出明确的修复建议
- 编译错误时提供详细反馈
- 友好的用户提示

## 发布到 NPM

要将此包发布到 NPM，执行：

```bash
# 1. 登录 NPM
npm login

# 2. 发布
npm publish

# 3. 用户即可使用
npm install -g winsage-agent
```

## 未来优化方向

1. **预编译二进制**: 提供预编译的 .exe 文件，避免本地编译
2. **增量更新**: 只更新变化的部分，减少更新时间
3. **版本管理**: 支持多版本共存和切换
4. **图形化安装向导**: 提供更直观的安装界面
5. **自动更新**: 内置自动更新机制

## 总结

通过 NPM 包管理器，我们将原本需要多个步骤的复杂安装过程简化为一条命令：

```bash
npm install -g winsage-agent
```

这大大降低了使用门槛，让用户能够快速获取和使用 WinSage 的强大功能！🎉
