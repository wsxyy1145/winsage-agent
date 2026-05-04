use clap::{Parser, Subcommand};
use console::style;
use winsage::agent::Agent;
use winsage::cli::{chat_command, config_command, sandbox_command};
use winsage::config::AppConfig;
use winsage::memory::{ConversationManager, MemoryStorage};
use winsage::models::{
    anthropic::AnthropicProvider, azure::AzureProvider, deepseek::DeepSeekProvider,
    doubao::DoubaoProvider, moonshot::MoonshotProvider, ollama::OllamaProvider,
    openai::OpenAIProvider, qwen::QwenProvider, ModelProvider,
};
use winsage::sandbox::{CommandExecutor, WindowsSandbox};
use winsage::utils::{init_logging, print_welcome};

#[derive(Parser)]
#[command(name = "winsage")]
#[command(about = "Windows Terminal AI Agent", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动交互式聊天
    Chat,
    /// 管理配置
    Config,
    /// 查看沙箱状态
    Sandbox,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    init_logging();

    // 打印欢迎信息
    print_welcome();

    // 加载配置
    let config = AppConfig::load()?;

    // 解析命令行参数
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Chat) => {
            // 创建模型提供者
            let provider = create_provider(&config)?;

            // 初始化记忆系统（如果配置了）
            let conversation_manager = if !config.memory.database_url.is_empty() {
                match MemoryStorage::new(&config.memory.database_url).await {
                    Ok(storage) => {
                        // 运行迁移
                        if let Err(e) = storage.migrate().await {
                            eprintln!("数据库迁移失败: {}", e);
                        }
                        Some(ConversationManager::new(storage))
                    }
                    Err(e) => {
                        eprintln!("连接数据库失败: {}", e);
                        None
                    }
                }
            } else {
                None
            };

            // 初始化沙箱
            let sandbox = WindowsSandbox::new(
                config.sandbox.enabled,
                config.sandbox.timeout_seconds,
                config.sandbox.shared_folder.clone(),
            );

            let command_executor = if config.sandbox.enabled {
                Some(CommandExecutor::new(sandbox.clone()))
            } else {
                None
            };

            // 创建 Agent
            let mut agent = Agent::new(provider, conversation_manager, command_executor);

            // 启动聊天
            chat_command(&mut agent).await?;
        }
        Some(Commands::Config) => {
            config_command().await?;
        }
        Some(Commands::Sandbox) => {
            let sandbox = WindowsSandbox::new(
                config.sandbox.enabled,
                config.sandbox.timeout_seconds,
                config.sandbox.shared_folder.clone(),
            );
            sandbox_command(&sandbox)?;
        }
        None => {
            // 默认启动聊天
            println!("{}", style("未指定命令，默认启动聊天模式").dim());
            println!();

            let provider = create_provider(&config)?;

            let sandbox = WindowsSandbox::new(
                config.sandbox.enabled,
                config.sandbox.timeout_seconds,
                config.sandbox.shared_folder.clone(),
            );

            let command_executor = if config.sandbox.enabled {
                Some(CommandExecutor::new(sandbox.clone()))
            } else {
                None
            };

            let mut agent = Agent::new(provider, None, command_executor);
            chat_command(&mut agent).await?;
        }
    }

    Ok(())
}

/// 根据配置创建模型提供者
fn create_provider(
    config: &AppConfig,
) -> Result<Box<dyn ModelProvider>, Box<dyn std::error::Error>> {
    let default_model = &config.general.default_model;

    // 优先使用配置的默认模型
    if default_model.starts_with("gpt") && !config.models.openai.api_key.is_empty() {
        Ok(Box::new(OpenAIProvider::new(
            config.models.openai.api_key.clone(),
            config.models.openai.base_url.clone(),
            config.models.openai.default_model.clone(),
        )))
    } else if default_model.starts_with("claude") && !config.models.anthropic.api_key.is_empty() {
        Ok(Box::new(AnthropicProvider::new(
            config.models.anthropic.api_key.clone(),
            config.models.anthropic.base_url.clone(),
            config.models.anthropic.default_model.clone(),
        )))
    } else if default_model.starts_with("deepseek") && !config.models.deepseek.api_key.is_empty() {
        Ok(Box::new(DeepSeekProvider::new(
            config.models.deepseek.api_key.clone(),
            config.models.deepseek.base_url.clone(),
            config.models.deepseek.default_model.clone(),
        )))
    } else if (default_model.starts_with("moonshot") || default_model.starts_with("kimi"))
        && !config.models.moonshot.api_key.is_empty()
    {
        Ok(Box::new(MoonshotProvider::new(
            config.models.moonshot.api_key.clone(),
            config.models.moonshot.base_url.clone(),
            config.models.moonshot.default_model.clone(),
        )))
    } else if (default_model.starts_with("qwen") || default_model.starts_with("qianwen"))
        && !config.models.qwen.api_key.is_empty()
    {
        Ok(Box::new(QwenProvider::new(
            config.models.qwen.api_key.clone(),
            config.models.qwen.base_url.clone(),
            config.models.qwen.default_model.clone(),
        )))
    } else if (default_model.starts_with("doubao") || default_model.starts_with("ep-"))
        && !config.models.doubao.api_key.is_empty()
    {
        Ok(Box::new(DoubaoProvider::new(
            config.models.doubao.api_key.clone(),
            config.models.doubao.base_url.clone(),
            config.models.doubao.default_model.clone(),
        )))
    } else if !config.models.azure.api_key.is_empty() && !config.models.azure.endpoint.is_empty() {
        Ok(Box::new(AzureProvider::new(
            config.models.azure.api_key.clone(),
            config.models.azure.endpoint.clone(),
            config.models.azure.deployment.clone(),
            config.models.azure.api_version.clone(),
        )))
    } else {
        // 默认使用 Ollama
        Ok(Box::new(OllamaProvider::new(
            config.models.ollama.base_url.clone(),
            config.models.ollama.model.clone(),
        )))
    }
}
