#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// 获取项目根目录
const projectRoot = path.join(__dirname, '..');

// 查找编译后的可执行文件
function findExecutable() {
  const releasePath = path.join(projectRoot, 'target', 'release', 'winsage.exe');
  const debugPath = path.join(projectRoot, 'target', 'debug', 'winsage.exe');

  if (fs.existsSync(releasePath)) {
    return releasePath;
  } else if (fs.existsSync(debugPath)) {
    return debugPath;
  } else {
    console.error('错误: 未找到 WinSage 可执行文件');
    console.error('请先运行安装: npm install -g winsage-agent');
    process.exit(1);
  }
}

// 启动 WinSage
function startWinSage(args) {
  const executable = findExecutable();
  const child = spawn(executable, args, {
    stdio: 'inherit',
    cwd: process.cwd()
  });

  child.on('error', (error) => {
    console.error('启动失败:', error.message);
    process.exit(1);
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

// 显示帮助信息
function showHelp() {
  console.log(`
╔════════════════════════════════════════╗
║   WinSage - Windows Terminal AI Agent  ║
╚════════════════════════════════════════╝

使用方法:
  winsage [command]

命令:
  chat        启动交互式聊天（默认）
  setup       运行初始配置向导
  config      管理配置
  sandbox     查看沙箱状态
  help        显示此帮助信息

示例:
  winsage              # 启动聊天
  winsage chat         # 启动聊天
  winsage setup        # 运行配置向导
  winsage config       # 打开配置
  winsage sandbox      # 检查沙箱

配置文件位置:
  %APPDATA%\\winsage\\config.toml

环境变量:
  WINSAGE_MODELS_OPENAI_API_KEY     OpenAI API Key
  WINSAGE_MODELS_ANTHROPIC_API_KEY  Anthropic API Key
  RUST_LOG                          日志级别 (debug/info/warn/error)
`);
}

// 运行配置向导
function runSetupWizard() {
  const wizardPath = path.join(projectRoot, 'scripts', 'setup-wizard.js');
  const child = spawn('node', [wizardPath], {
    stdio: 'inherit',
    cwd: process.cwd()
  });

  child.on('error', (error) => {
    console.error('启动配置向导失败:', error.message);
    process.exit(1);
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

// 主逻辑
function main() {
  const args = process.argv.slice(2);

  // 如果没有参数或第一个参数是chat，直接启动聊天
  if (args.length === 0 || args[0] === 'chat') {
    startWinSage(['chat']);
    return;
  }

  // 处理help命令
  if (args[0] === 'help' || args[0] === '--help' || args[0] === '-h') {
    showHelp();
    return;
  }

  // 处理setup命令 - 运行配置向导
  if (args[0] === 'setup') {
    runSetupWizard();
    return;
  }

  // 其他命令直接传递给Rust程序
  startWinSage(args);
}

main();
