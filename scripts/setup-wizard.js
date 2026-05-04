#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const readline = require('readline');
const os = require('os');

// 颜色输出
const colors = {
  reset: '\x1b[0m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  red: '\x1b[31m',
  cyan: '\x1b[36m',
  magenta: '\x1b[35m',
  bold: '\x1b[1m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function clearScreen() {
  process.stdout.write('\x1bc');
}

// 创建 readline 接口
function createInterface() {
  return readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });
}

// 询问问题
function askQuestion(rl, question, defaultValue = '') {
  return new Promise((resolve) => {
    const defaultText = defaultValue ? ` (${defaultValue})` : '';
    rl.question(`${question}${defaultText}: `, (answer) => {
      resolve(answer.trim() || defaultValue);
    });
  });
}

// 显示欢迎界面
function showWelcome() {
  clearScreen();
  log('\n╔══════════════════════════════════════════════════╗', 'cyan');
  log('║                                                  ║', 'cyan');
  log('║     WinSage - Windows Terminal AI Agent          ║', 'cyan');
  log('║           初始配置向导                           ║', 'cyan');
  log('║                                                  ║', 'cyan');
  log('╚══════════════════════════════════════════════════╝', 'cyan');
  log('\n');
  log('本向导将帮助您快速配置 WinSage', 'bold');
  log('您可以随时跳过任何步骤，稍后手动编辑配置文件\n', 'yellow');
}

// 模型提供商选项
const MODEL_PROVIDERS = [
  {
    id: 'openai',
    name: 'OpenAI (GPT-4/GPT-3.5)',
    url: 'https://platform.openai.com/api-keys',
    baseUrl: 'https://api.openai.com/v1',
    defaultModel: 'gpt-4',
    popular: true
  },
  {
    id: 'deepseek',
    name: 'DeepSeek (深度求索)',
    url: 'https://platform.deepseek.com/',
    baseUrl: 'https://api.deepseek.com/v1',
    defaultModel: 'deepseek-chat',
    popular: true
  },
  {
    id: 'moonshot',
    name: 'Kimi (月之暗面)',
    url: 'https://platform.moonshot.cn/',
    baseUrl: 'https://api.moonshot.cn',
    defaultModel: 'moonshot-v1-8k',
    popular: true
  },
  {
    id: 'qwen',
    name: 'Qwen (通义千问/阿里云)',
    url: 'https://dashscope.aliyun.com/',
    baseUrl: 'https://dashscope.aliyuncs.com',
    defaultModel: 'qwen-turbo',
    popular: true
  },
  {
    id: 'doubao',
    name: 'Doubao (豆包/字节跳动)',
    url: 'https://www.volcengine.com/',
    baseUrl: 'https://ark.cn-beijing.volces.com',
    defaultModel: 'ep-xxxxx',
    popular: false
  },
  {
    id: 'anthropic',
    name: 'Anthropic (Claude)',
    url: 'https://console.anthropic.com/',
    baseUrl: 'https://api.anthropic.com/v1',
    defaultModel: 'claude-3-opus-20240229',
    popular: false
  },
  {
    id: 'ollama',
    name: 'Ollama (本地部署)',
    url: 'https://ollama.ai/',
    baseUrl: 'http://localhost:11434',
    defaultModel: 'llama2',
    popular: false,
    noApiKey: true
  }
];

// 显示模型选择菜单
function showModelMenu() {
  log('\n┌─────────────────────────────────────────────────┐', 'cyan');
  log('│  请选择您要配置的模型提供商                      │', 'cyan');
  log('└─────────────────────────────────────────────────┘', 'cyan');
  log('\n');

  MODEL_PROVIDERS.forEach((provider, index) => {
    const popularMark = provider.popular ? ' ⭐' : '';
    const freeMark = provider.noApiKey ? ' 🆓' : '';
    log(`  ${index + 1}. ${provider.name}${popularMark}${freeMark}`, 'blue');
  });

  log('\n  0. 跳过，稍后手动配置', 'yellow');
  log('\n提示: 标记 ⭐ 的为热门模型，🆓 表示无需 API Key（本地部署）\n', 'yellow');
}

// 配置单个模型提供商
async function configureProvider(rl, provider) {
  log(`\n┌─────────────────────────────────────────────────┐`, 'cyan');
  log(`│  配置 ${provider.name}`, 'cyan');
  log(`└─────────────────────────────────────────────────┘`, 'cyan');

  if (!provider.noApiKey) {
    log(`\n获取 API Key: ${provider.url}\n`, 'blue');

    const apiKey = await askQuestion(rl, '请输入 API Key');

    if (!apiKey) {
      log('⚠ 未输入 API Key，跳过此模型', 'yellow');
      return null;
    }

    return {
      api_key: apiKey,
      base_url: provider.baseUrl,
      default_model: provider.defaultModel
    };
  } else {
    // Ollama 等无需 API Key 的模型
    const baseUrl = await askQuestion(rl, '服务地址', provider.baseUrl);
    const model = await askQuestion(rl, '默认模型', provider.defaultModel);

    return {
      api_key: '',
      base_url: baseUrl,
      default_model: model
    };
  }
}

// 通用配置
async function configureGeneral(rl, defaultModel) {
  log('\n┌─────────────────────────────────────────────────┐', 'cyan');
  log('│  通用配置                                        │', 'cyan');
  log('└─────────────────────────────────────────────────┘', 'cyan');

  const temperature = await askQuestion(rl, '温度参数 (0.0-1.0，控制创造性)', '0.7');
  const maxTokens = await askQuestion(rl, '最大 Token 数', '4096');
  const streaming = await askQuestion(rl, '启用流式输出 (true/false)', 'true');

  return {
    default_model: defaultModel,
    temperature: parseFloat(temperature),
    max_tokens: parseInt(maxTokens),
    streaming: streaming.toLowerCase() === 'true'
  };
}

// PostgreSQL 配置
async function configureMemory(rl) {
  log('\n┌─────────────────────────────────────────────────┐', 'cyan');
  log('│  记忆系统配置 (PostgreSQL)                       │', 'cyan');
  log('└─────────────────────────────────────────────────┘', 'cyan');
  log('\n记忆系统用于存储对话历史，是可选功能\n', 'yellow');

  const setupDb = await askQuestion(rl, '是否现在配置 PostgreSQL? (y/n)', 'n');

  if (setupDb.toLowerCase() === 'y') {
    const dbUrl = await askQuestion(
      rl,
      '数据库连接 URL',
      'postgresql://postgres:postgres@localhost/winsage'
    );

    return {
      database_url: dbUrl,
      enable_vector_search: false,
      max_history_length: 100
    };
  } else {
    log('跳过 PostgreSQL 配置，可以稍后添加', 'yellow');
    return {
      database_url: '',
      enable_vector_search: false,
      max_history_length: 100
    };
  }
}

// 沙箱配置
async function configureSandbox(rl) {
  log('\n┌─────────────────────────────────────────────────┐', 'cyan');
  log('│  沙箱配置 (Windows Sandbox)                      │', 'cyan');
  log('└─────────────────────────────────────────────────┘', 'cyan');
  log('\n沙箱用于安全执行危险命令，需要 Windows 专业版或企业版\n', 'yellow');

  const enabled = await askQuestion(rl, '启用沙箱? (y/n)', 'y');

  return {
    enabled: enabled.toLowerCase() === 'y',
    auto_cleanup: true,
    timeout_seconds: 300,
    shared_folder: ''
  };
}

// 生成配置文件
function generateConfig(general, models, memory, sandbox) {
  const config = {
    general,
    models,
    memory,
    sandbox
  };

  return config;
}

// 将配置对象转换为 TOML 格式
function toToml(config) {
  let toml = '';

  // General
  toml += '[general]\n';
  toml += `default_model = "${config.general.default_model}"\n`;
  toml += `temperature = ${config.general.temperature}\n`;
  toml += `max_tokens = ${config.general.max_tokens}\n`;
  toml += `streaming = ${config.general.streaming}\n`;
  toml += '\n';

  // Models
  Object.entries(config.models).forEach(([key, value]) => {
    toml += `[models.${key}]\n`;
    if (value.api_key !== undefined) {
      toml += `api_key = "${value.api_key}"\n`;
    }
    if (value.base_url !== undefined) {
      toml += `base_url = "${value.base_url}"\n`;
    }
    if (value.default_model !== undefined) {
      toml += `default_model = "${value.default_model}"\n`;
    }
    if (value.model !== undefined) {
      toml += `model = "${value.model}"\n`;
    }
    if (value.endpoint !== undefined) {
      toml += `endpoint = "${value.endpoint}"\n`;
    }
    if (value.deployment !== undefined) {
      toml += `deployment = "${value.deployment}"\n`;
    }
    if (value.api_version !== undefined) {
      toml += `api_version = "${value.api_version}"\n`;
    }
    toml += '\n';
  });

  // Memory
  toml += '[memory]\n';
  toml += `database_url = "${config.memory.database_url}"\n`;
  toml += `enable_vector_search = ${config.memory.enable_vector_search}\n`;
  toml += `max_history_length = ${config.memory.max_history_length}\n`;
  toml += '\n';

  // Sandbox
  toml += '[sandbox]\n';
  toml += `enabled = ${config.sandbox.enabled}\n`;
  toml += `auto_cleanup = ${config.sandbox.auto_cleanup}\n`;
  toml += `timeout_seconds = ${config.sandbox.timeout_seconds}\n`;
  toml += `shared_folder = "${config.sandbox.shared_folder}"\n`;

  return toml;
}

// 保存配置文件
function saveConfig(config) {
  const appData = process.env.APPDATA;
  const configDir = path.join(appData, 'winsage');
  const configFile = path.join(configDir, 'config.toml');

  // 创建目录
  if (!fs.existsSync(configDir)) {
    fs.mkdirSync(configDir, { recursive: true });
  }

  // 备份旧配置
  if (fs.existsSync(configFile)) {
    const backupFile = `${configFile}.backup.${Date.now()}`;
    fs.copyFileSync(configFile, backupFile);
    log(`✓ 已备份旧配置到: ${backupFile}`, 'green');
  }

  // 写入新配置
  const tomlContent = toToml(config);
  fs.writeFileSync(configFile, tomlContent, 'utf-8');

  return configFile;
}

// 显示完成信息
function showCompletion(configFile, configuredModels) {
  clearScreen();
  log('\n╔══════════════════════════════════════════════════╗', 'cyan');
  log('║                                                  ║', 'cyan');
  log('║              配置完成！                          ║', 'cyan');
  log('║                                                  ║', 'cyan');
  log('╚══════════════════════════════════════════════════╝', 'cyan');
  log('\n');

  log('✓ 配置文件已保存到:', 'green');
  log(`  ${configFile}\n`, 'blue');

  if (configuredModels.length > 0) {
    log('✓ 已配置的模型:', 'green');
    configuredModels.forEach(model => {
      log(`  - ${model}`, 'green');
    });
    log('');
  }

  log('下一步:', 'bold');
  log('  1. 运行 winsage chat 开始聊天', 'blue');
  log('  2. 如需修改配置，编辑配置文件或使用 winsage config', 'blue');
  log('  3. 查看帮助: winsage help\n', 'blue');

  log('提示: 您可以随时添加更多模型，只需编辑配置文件\n', 'yellow');
}

// 主函数
async function main() {
  const rl = createInterface();

  try {
    showWelcome();

    // 等待用户按回车继续
    await askQuestion(rl, '按回车键开始配置...');

    // 选择并配置模型
    const models = {
      openai: { api_key: '', base_url: 'https://api.openai.com/v1', default_model: 'gpt-4' },
      anthropic: { api_key: '', base_url: 'https://api.anthropic.com/v1', default_model: 'claude-3-opus-20240229' },
      azure: { api_key: '', endpoint: '', deployment: 'gpt-4', api_version: '2024-02-15-preview' },
      ollama: { base_url: 'http://localhost:11434', model: 'llama2' },
      deepseek: { api_key: '', base_url: 'https://api.deepseek.com/v1', default_model: 'deepseek-chat' },
      moonshot: { api_key: '', base_url: 'https://api.moonshot.cn', default_model: 'moonshot-v1-8k' },
      qwen: { api_key: '', base_url: 'https://dashscope.aliyuncs.com', default_model: 'qwen-turbo' },
      doubao: { api_key: '', base_url: 'https://ark.cn-beijing.volces.com', default_model: 'ep-xxxxx' }
    };

    const configuredModels = [];
    let firstModel = null;

    showModelMenu();
    const choice = await askQuestion(rl, '请选择 (输入编号)');

    const choiceNum = parseInt(choice);
    if (choiceNum > 0 && choiceNum <= MODEL_PROVIDERS.length) {
      const provider = MODEL_PROVIDERS[choiceNum - 1];
      const config = await configureProvider(rl, provider);

      if (config) {
        models[provider.id] = config;
        configuredModels.push(provider.name);
        firstModel = config.default_model || config.model;

        // 询问是否配置更多模型
        const more = await askQuestion(rl, '是否配置其他模型? (y/n)', 'n');
        if (more.toLowerCase() === 'y') {
          // 可以递归配置更多模型
          showModelMenu();
          log('提示: 输入 0 完成模型配置\n', 'yellow');
        }
      }
    }

    // 如果没有配置任何模型，使用默认的 Ollama
    if (configuredModels.length === 0) {
      log('\n未配置任何模型，将使用 Ollama (本地部署) 作为默认', 'yellow');
      firstModel = 'llama2';
    }

    // 配置通用设置
    const general = await configureGeneral(rl, firstModel || 'gpt-4');

    // 配置记忆系统
    const memory = await configureMemory(rl);

    // 配置沙箱
    const sandbox = await configureSandbox(rl);

    // 生成并保存配置
    const config = generateConfig(general, models, memory, sandbox);
    const configFile = saveConfig(config);

    // 显示完成信息
    showCompletion(configFile, configuredModels);

  } catch (error) {
    log(`\n错误: ${error.message}`, 'red');
  } finally {
    rl.close();
  }
}

// 检查是否在非交互模式
if (process.argv.includes('--non-interactive')) {
  log('非交互模式：跳过配置向导', 'yellow');
  log('您可以稍后运行 winsage setup 进行配置\n', 'yellow');
  process.exit(0);
} else {
  main();
}
