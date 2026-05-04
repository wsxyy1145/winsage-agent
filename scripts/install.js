#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const os = require('os');

// 颜色输出
const colors = {
  reset: '\x1b[0m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  red: '\x1b[31m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function checkWindows() {
  if (process.platform !== 'win32') {
    log('错误: WinSage 仅支持 Windows 系统', 'red');
    process.exit(1);
  }
}

function checkRust() {
  try {
    execSync('rustc --version', { stdio: 'ignore' });
    log('✓ Rust 已安装', 'green');
    return true;
  } catch (error) {
    log('⚠ 未检测到 Rust，正在安装...', 'yellow');
    return false;
  }
}

function installRust() {
  log('正在安装 Rust...', 'cyan');
  try {
    // 使用 winget 安装 Rust
    execSync('winget install Rustlang.Rust.MSVC --silent --accept-package-agreements --accept-source-agreements', {
      stdio: 'inherit'
    });
    log('✓ Rust 安装成功', 'green');
    return true;
  } catch (error) {
    log('错误: Rust 安装失败。请手动从 https://rustup.rs 安装', 'red');
    process.exit(1);
  }
}

function checkVisualStudioBuildTools() {
  const programFiles = process.env['ProgramFiles(x86)'] || 'C:\\Program Files (x86)';
  const vsPath = path.join(programFiles, 'Microsoft Visual Studio');

  if (fs.existsSync(vsPath)) {
    log('✓ Visual Studio Build Tools 已安装', 'green');
    return true;
  } else {
    log('⚠ 未检测到 Visual Studio Build Tools', 'yellow');
    return false;
  }
}

function installVisualStudioBuildTools() {
  log('提示: 需要安装 Visual Studio Build Tools', 'cyan');
  log('请访问: https://visualstudio.microsoft.com/visual-cpp-build-tools/', 'blue');
  log('选择 "C++ build tools" 工作负载进行安装', 'blue');

  const readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
  });

  return new Promise((resolve) => {
    readline.question('\n安装完成后按回车继续... ', () => {
      readline.close();
      resolve();
    });
  });
}

function checkPostgreSQL() {
  try {
    execSync('docker --version', { stdio: 'ignore' });
    log('✓ Docker 已安装（可用于 PostgreSQL）', 'green');
    return true;
  } catch (error) {
    log('ℹ 未检测到 Docker（PostgreSQL 为可选依赖）', 'yellow');
    return false;
  }
}

function setupPostgreSQL() {
  // 检查是否通过环境变量或参数指定了非交互模式
  const nonInteractive = process.env.CI || process.argv.includes('--non-interactive');

  if (nonInteractive) {
    log('跳过 PostgreSQL 安装（非交互模式）', 'yellow');
    return Promise.resolve();
  }

  const readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
  });

  return new Promise((resolve) => {
    readline.question('是否使用 Docker 快速部署 PostgreSQL? (y/n) ', (answer) => {
      readline.close();
      if (answer.toLowerCase() === 'y') {
        try {
          log('正在启动 PostgreSQL Docker 容器...', 'cyan');
          execSync('docker run --name winsage-postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=winsage -p 5432:5432 -d postgres:16', {
            stdio: 'inherit'
          });
          log('✓ PostgreSQL 启动成功', 'green');
        } catch (error) {
          log('⚠ PostgreSQL 启动失败，可以稍后手动配置', 'yellow');
        }
      } else {
        log('跳过 PostgreSQL 安装', 'yellow');
      }
      resolve();
    });
  });
}

function buildProject() {
  log('\n正在编译 WinSage...', 'cyan');
  try {
    execSync('cargo build --release', {
      stdio: 'inherit',
      cwd: __dirname + '/..'
    });
    log('✓ 编译成功', 'green');
    return true;
  } catch (error) {
    log('错误: 编译失败', 'red');
    process.exit(1);
  }
}

function setupConfig() {
  const appData = process.env.APPDATA;
  const configDir = path.join(appData, 'winsage');
  const configFile = path.join(configDir, 'config.toml');

  if (!fs.existsSync(configDir)) {
    fs.mkdirSync(configDir, { recursive: true });
  }

  if (!fs.existsSync(configFile)) {
    const defaultConfig = `# WinSage 配置文件
# 首次使用时，请至少配置一个模型提供商的 API Key

[general]
default_model = "gpt-4"
temperature = 0.7
max_tokens = 4096

[models.openai]
api_key = ""  # 在此填入您的 OpenAI API Key
base_url = "https://api.openai.com/v1"
default_model = "gpt-4"

[models.anthropic]
api_key = ""  # 或使用 Anthropic

[models.azure]
api_key = ""  # 或使用 Azure OpenAI
endpoint = "https://your-resource.openai.azure.com/"
deployment = "gpt-4"

[models.ollama]
base_url = "http://localhost:11434"
model = "llama2"

[memory]
database_url = "postgresql://postgres:postgres@localhost/winsage"
enable_vector_search = false

[sandbox]
enabled = true
auto_cleanup = true
timeout_seconds = 300
`;
    fs.writeFileSync(configFile, defaultConfig, 'utf-8');
    log('✓ 配置文件已创建: ' + configFile, 'green');
  } else {
    log('✓ 配置文件已存在', 'green');
  }
}

function createGlobalLink() {
  const projectRoot = path.join(__dirname, '..');
  const binPath = path.join(projectRoot, 'bin', 'winsage.js');

  // npm link 会自动处理，这里只是提示用户
  log('\n✓ WinSage 已准备就绪！', 'green');
  log('\n使用方法:', 'cyan');
  log('  winsage chat     - 启动聊天', 'blue');
  log('  winsage config   - 管理配置', 'blue');
  log('  winsage sandbox  - 查看沙箱状态', 'blue');
  log('  winsage help     - 显示帮助', 'blue');
}

// 询问是否运行配置向导
async function promptSetupWizard() {
  const nonInteractive = process.env.CI || process.argv.includes('--non-interactive');

  if (nonInteractive) {
    log('\n非交互模式：跳过配置向导', 'yellow');
    return;
  }

  const readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
  });

  return new Promise((resolve) => {
    log('\n┌─────────────────────────────────────────────────┐', 'cyan');
    log('│  是否现在进行初始配置？                          │', 'cyan');
    log('└─────────────────────────────────────────────────┘', 'cyan');
    log('\n配置向导将帮助您：', 'blue');
    log('  ✓ 选择并配置 AI 模型（OpenAI/DeepSeek/Kimi/Qwen等）', 'blue');
    log('  ✓ 设置 API Key', 'blue');
    log('  ✓ 配置温度参数和最大 Token', 'blue');
    log('  ✓ 可选配置 PostgreSQL 记忆系统', 'blue');
    log('  ✓ 配置 Windows Sandbox 沙箱\n', 'blue');

    readline.question('是否启动配置向导? (y/n) ', (answer) => {
      readline.close();
      if (answer.toLowerCase() === 'y' || answer.toLowerCase() === 'yes') {
        log('\n正在启动配置向导...\n', 'cyan');
        try {
          const wizardPath = path.join(__dirname, 'setup-wizard.js');
          execSync(`node "${wizardPath}"`, {
            stdio: 'inherit',
            cwd: __dirname
          });
          log('\n✓ 配置完成！', 'green');
        } catch (error) {
          log('\n⚠ 配置向导执行失败，可以稍后手动配置', 'yellow');
        }
      } else {
        log('\n跳过配置向导', 'yellow');
        log('您可以稍后运行以下命令进行配置:', 'blue');
        log('  node scripts/setup-wizard.js\n', 'blue');
      }
      resolve();
    });
  });
}

async function main() {
  log('\n╔════════════════════════════════════════╗', 'cyan');
  log('║   WinSage - Windows Terminal AI Agent  ║', 'cyan');
  log('║         一键安装程序                    ║', 'cyan');
  log('╚════════════════════════════════════════╝\n', 'cyan');

  checkWindows();

  // 检查并安装 Rust
  if (!checkRust()) {
    installRust();
  }

  // 检查 Visual Studio Build Tools
  if (!checkVisualStudioBuildTools()) {
    await installVisualStudioBuildTools();
  }

  // 检查 PostgreSQL
  checkPostgreSQL();
  await setupPostgreSQL();

  // 编译项目
  buildProject();

  // 设置配置
  setupConfig();

  // 完成
  createGlobalLink();

  log('\n🎉 安装完成！', 'green');

  // 询问是否运行配置向导
  await promptSetupWizard();
}

main().catch((error) => {
  log('\n错误: ' + error.message, 'red');
  process.exit(1);
});
