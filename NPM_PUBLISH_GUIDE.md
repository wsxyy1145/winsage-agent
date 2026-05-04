# 📦 NPM 发布完整指南

## 🎯 前置要求

### 1. 注册 NPM 账号

**方式 A: 网页注册（推荐）**

1. 访问: https://www.npmjs.com/signup
2. 填写信息：
   - **Username**: 您的用户名（例如：wsxyy1145）
   - **Email**: 您的邮箱
   - **Password**: 设置密码
3. 验证邮箱（点击邮件中的确认链接）

**方式 B: 命令行注册**

```bash
npm adduser
```

按提示输入用户名、密码和邮箱。

---

## 📋 发布前准备

### 1. 检查 package.json

确保以下字段正确配置：

```json
{
  "name": "winsage-agent",
  "version": "0.1.0",
  "description": "Windows Terminal AI Agent - Multi-model support with sandbox isolation",
  "main": "index.js",
  "bin": {
    "winsage": "./bin/winsage.js"
  },
  "scripts": {
    "postinstall": "node scripts/install.js --non-interactive",
    "setup": "node scripts/setup-wizard.js"
  },
  "keywords": [
    "ai",
    "agent",
    "windows",
    "terminal",
    "cli",
    "chatbot",
    "sandbox",
    "rust",
    "openai",
    "deepseek",
    "kimi",
    "qwen",
    "doubao"
  ],
  "author": "wsxyy1145",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/wsxyy1145/winsage-agent.git"
  },
  "engines": {
    "node": ">=14.0.0",
    "npm": ">=6.0.0"
  },
  "os": [
    "win32"
  ]
}
```

### 2. 创建 .npmignore

确保已创建 `.npmignore` 文件，排除不必要的文件：

```
# 已在项目中创建
# 查看 .npmignore 文件内容
```

### 3. 检查包名可用性

访问: https://www.npmjs.com/package/winsage-agent

如果显示 "404 Not Found"，说明包名可用 ✅

---

## 🔐 登录 NPM

### 方式 1: 命令行登录（推荐）

```bash
npm login
```

按提示输入：
1. **Username**: wsxyy1145
2. **Password**: 您的密码
3. **Email**: 您的邮箱
4. **OTP Code**: 如果启用了双因素认证，输入验证码

成功会显示：
```
Logged in as wsxyy1145 on https://registry.npmjs.org/.
```

### 方式 2: 验证登录状态

```bash
npm whoami
```

应该显示您的用户名。

---

## 🚀 发布到 NPM

### 首次发布

```bash
# 确保在项目根目录
cd e:\WinSage-agent

# 发布包
npm publish --access public
```

**注意**: `--access public` 是必须的，因为这是公开包。

### 发布成功输出

```
npm notice
npm notice 📦  winsage-agent@0.1.0
npm notice === Tarball Contents ===
npm notice ...
npm notice === Tarball Details ===
npm notice name:          winsage-agent
npm notice version:       0.1.0
npm notice ...
npm notice unpacked size: xxx kB
npm notice total files:   xx
npm notice
+ winsage-agent@0.1.0
```

---

## ✅ 验证发布

### 1. 查看包信息

```bash
npm info winsage-agent
```

### 2. 在网页上查看

访问: https://www.npmjs.com/package/winsage-agent

### 3. 测试全局安装

```bash
# 在新目录测试
cd \temp
npm install -g winsage-agent
winsage help
```

---

## 🔄 更新版本

### 版本号规则

遵循语义化版本 (SemVer): `主版本.次版本.修订号`

- **修订号 (patch)**: Bug 修复 → `0.1.0` → `0.1.1`
- **次版本 (minor)**: 新功能（向后兼容）→ `0.1.0` → `0.2.0`
- **主版本 (major)**: 破坏性变更 → `0.1.0` → `1.0.0`

### 更新步骤

```bash
# 1. 更新版本号（自动）
npm version patch   # 0.1.0 -> 0.1.1
npm version minor   # 0.1.0 -> 0.2.0
npm version major   # 0.1.0 -> 1.0.0

# 2. 发布新版本
npm publish

# 3. 推送到 GitHub
git push
git push --tags
```

---

## ❌ 常见问题

### 问题 1: 包名已被占用

**错误信息**:
```
npm error 403 Forbidden - PUT https://registry.npmjs.org/winsage-agent
npm error 403 You cannot publish over the previously published versions
```

**解决方案**:

**选项 A**: 修改包名

```json
{
  "name": "@wsxyy1145/winsage-agent"
}
```

然后发布：
```bash
npm publish --access public
```

使用时：
```bash
npm install -g @wsxyy1145/winsage-agent
```

**选项 B**: 联系包所有者转让

### 问题 2: 需要双因素认证

**错误信息**:
```
npm error code E403
npm error 403 This package requires you to use a valid OTP
```

**解决方案**:

启用双因素认证后，每次发布需要提供 OTP：

```bash
npm publish --otp=123456
```

或者在登录时设置：
```bash
npm profile enable-2fa auth-and-writes
```

### 问题 3: 未登录

**错误信息**:
```
npm error 403 Forbidden - PUT https://registry.npmjs.org/winsage-agent
npm error 403 This operation requires a logged-in user
```

**解决方案**:

```bash
npm login
```

### 问题 4: 邮箱未验证

**错误信息**:
```
npm error 403 Forbidden
npm error 403 Please verify your email before publishing
```

**解决方案**:

1. 检查邮箱
2. 点击 NPM 发送的验证链接
3. 重新发布

### 问题 5: 文件太大

**警告**:
```
npm WARN tarball tarball data seems to be too large
```

**解决方案**:

检查 `.npmignore` 文件，确保排除了：
- `target/` 目录
- `.git/` 目录
- 大型文档文件

---

## 📊 发布后管理

### 查看下载统计

访问: https://www.npmjs.com/package/winsage-agent

或命令行：
```bash
npm view winsage-agent downloads
```

### 管理协作者

```bash
# 添加协作者
npm owner add username winsage-agent

# 查看协作者
npm owner ls winsage-agent

# 移除协作者
npm owner rm username winsage-agent
```

### 弃用版本

```bash
# 弃用特定版本
npm deprecate winsage-agent@0.1.0 "This version is deprecated"

# 弃用所有版本
npm deprecate winsage-agent "This package is deprecated"
```

---

## 🔧 高级选项

### 1. 发布 Beta 版本

```bash
# 设置 beta 标签
npm version prerelease --preid=beta
npm publish --tag beta
```

用户安装：
```bash
npm install -g winsage-agent@beta
```

### 2. 发布 Canary 版本

```bash
npm version prerelease --preid=canary
npm publish --tag canary
```

### 3. 使用 CI/CD 自动发布

创建 `.github/workflows/publish.yml`:

```yaml
name: Publish to NPM

on:
  release:
    types: [created]

jobs:
  publish:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        registry-url: 'https://registry.npmjs.org'

    - name: Install dependencies
      run: npm ci

    - name: Publish
      run: npm publish --access public
      env:
        NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

在 GitHub Settings 中添加 `NPM_TOKEN` secret。

---

## 📝 完整发布流程示例

```bash
# 1. 登录 NPM
npm login

# 2. 检查当前版本
cat package.json | grep version

# 3. 预发布测试（可选）
npm pack
tar -xzf winsage-agent-0.1.0.tgz
cd package
npm install -g .
winsage help
cd ..

# 4. 正式发布
npm publish --access public

# 5. 验证发布
npm info winsage-agent

# 6. 推送到 GitHub
git push
git push --tags

# 7. 创建 GitHub Release
# 访问: https://github.com/wsxyy1145/winsage-agent/releases/new
```

---

## 🎯 WinSage 专属发布命令

根据您的项目，执行以下命令即可发布：

```bash
# 确保在项目目录
cd e:\WinSage-agent

# 登录 NPM（首次）
npm login

# 发布
npm publish --access public
```

就这么简单！🚀

---

## 📚 相关资源

- NPM 官方文档: https://docs.npmjs.com/
- 发布指南: https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry
- 语义化版本: https://semver.org/
- NPM CLI: https://docs.npmjs.com/cli/v9/commands/npm

---

**祝您发布顺利！** 🎉
