# 发布 WinSage 到 NPM

## 为什么出现 404 错误？

`winsage-agent` 还没有发布到 NPM 官方仓库，所以执行 `npm install -g winsage-agent` 会找不到包。

## 解决方案

### 方案 A: 使用本地安装（推荐用于开发/测试）

```bash
cd e:\WinSage-agent
npm install -g .
```

这样可以直接使用本地的代码，无需发布到 NPM。

### 方案 B: 发布到 NPM（供所有人使用）

如果您想让全世界的人都能通过 `npm install -g winsage-agent` 安装，需要发布到 NPM。

## 发布到 NPM 的步骤

### 1. 注册 NPM 账号

访问 [https://www.npmjs.com/signup](https://www.npmjs.com/signup) 注册账号

### 2. 登录 NPM

```bash
npm login
```

输入您的：
- Username
- Password
- Email

### 3. 检查包名可用性

访问 [https://www.npmjs.com/package/winsage-agent](https://www.npmjs.com/package/winsage-agent) 确认包名未被占用

### 4. 更新 package.json

确保以下字段正确：

```json
{
  "name": "winsage-agent",
  "version": "0.1.0",
  "description": "Windows Terminal AI Agent",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/your-username/winsage-agent.git"
  }
}
```

### 5. 发布

```bash
# 首次发布
npm publish --access public

# 后续更新版本
npm version patch  # 0.1.0 -> 0.1.1
npm publish
```

### 6. 验证发布

```bash
# 查看包信息
npm info winsage-agent

# 全局安装测试
npm install -g winsage-agent
```

## 注意事项

### 包名冲突

如果 `winsage-agent` 已被占用，需要修改包名：

```json
{
  "name": "@your-username/winsage-agent"
}
```

然后使用 scoped 包名安装：

```bash
npm install -g @your-username/winsage-agent
```

### 文件大小

NPM 包有大小限制，建议：
- 不包含编译产物（target/ 目录）
- 在 `.npmignore` 中排除不必要的文件

创建 `.npmignore` 文件：

```
# 忽略编译产物
target/
*.exe

# 忽略开发文件
.git/
.vscode/
.env
*.log

# 忽略文档（可选）
DELIVERY.md
PROJECT_SUMMARY.md
```

### 预编译二进制

由于 WinSage 是 Rust 项目，需要考虑：

**选项 1**: 包含预编译的 .exe 文件
- 优点: 用户无需编译，安装快速
- 缺点: 包体积大，需要为不同平台编译

**选项 2**: 让用户自己编译
- 优点: 包体积小，自动适配平台
- 缺点: 安装时间长，需要 Rust 环境

**推荐**: 选项 2（当前实现），因为：
- Rust 生态的标准做法
- 自动适配用户的系统
- 包体积小，分发方便

## 替代方案：使用 GitHub Releases

如果不想发布到 NPM，可以使用 GitHub Releases：

### 1. 创建 Release

```bash
# 编译
cargo build --release

# 打包
tar -czf winsage-agent-v0.1.0-windows-x64.tar.gz target/release/winsage.exe

# 上传到 GitHub Releases
```

### 2. 用户使用

```bash
# 从 GitHub 下载
# 或使用 scoop/chocolatey 等 Windows 包管理器
```

## 当前推荐做法

由于 WinSage 还在开发阶段，建议使用：

```bash
# 开发者/测试者使用
cd e:\WinSage-agent
npm install -g .

# 或直接运行
cargo run -- chat
```

等项目稳定后再发布到 NPM。

## 快速测试

验证当前安装是否正常：

```bash
# 检查是否已安装
npm list -g winsage-agent

# 运行帮助
winsage help

# 运行配置向导
winsage setup

# 开始聊天
winsage chat
```

---

**总结**: 目前请使用 `npm install -g .` 进行本地安装，等项目成熟后再发布到 NPM。
