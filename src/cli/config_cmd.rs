use crate::config::AppConfig;
use console::style;
use dialoguer::{Confirm, Input};

/// 配置管理命令
pub async fn config_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("=== 配置管理 ===").green().bold());

    // 加载当前配置
    let mut config = AppConfig::load()?;

    loop {
        println!("\n{}", style("请选择要配置的项目:").cyan());
        println!("1. OpenAI API 密钥");
        println!("2. Anthropic API 密钥");
        println!("3. Azure API 密钥");
        println!("4. Ollama 配置");
        println!("5. 数据库连接");
        println!("6. 沙箱设置");
        println!("7. 保存并退出");
        println!("8. 退出不保存");

        let choice: String = Input::new()
            .with_prompt("选择 (1-8)")
            .default("7".to_string())
            .interact_text()?;

        match choice.as_str() {
            "1" => configure_openai(&mut config)?,
            "2" => configure_anthropic(&mut config)?,
            "3" => configure_azure(&mut config)?,
            "4" => configure_ollama(&mut config)?,
            "5" => configure_memory(&mut config)?,
            "6" => configure_sandbox(&mut config)?,
            "7" => {
                config.save()?;
                println!("\n{}", style("配置已保存").green());
                break;
            }
            "8" => {
                println!("\n{}", style("配置未保存").yellow());
                break;
            }
            _ => println!("{}", style("无效选择").red()),
        }
    }

    Ok(())
}

fn configure_openai(config: &mut AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("OpenAI 配置").cyan().bold());

    let api_key: String = Input::new()
        .with_prompt("API 密钥")
        .default(config.models.openai.api_key.clone())
        .allow_empty(true)
        .interact_text()?;

    config.models.openai.api_key = api_key;

    let model: String = Input::new()
        .with_prompt("默认模型")
        .default(config.models.openai.default_model.clone())
        .interact_text()?;

    config.models.openai.default_model = model;

    Ok(())
}

fn configure_anthropic(config: &mut AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("Anthropic 配置").cyan().bold());

    let api_key: String = Input::new()
        .with_prompt("API 密钥")
        .default(config.models.anthropic.api_key.clone())
        .allow_empty(true)
        .interact_text()?;

    config.models.anthropic.api_key = api_key;

    Ok(())
}

fn configure_azure(config: &mut AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("Azure AI 配置").cyan().bold());

    let api_key: String = Input::new()
        .with_prompt("API 密钥")
        .default(config.models.azure.api_key.clone())
        .allow_empty(true)
        .interact_text()?;

    config.models.azure.api_key = api_key;

    let endpoint: String = Input::new()
        .with_prompt("Endpoint URL")
        .default(config.models.azure.endpoint.clone())
        .allow_empty(true)
        .interact_text()?;

    config.models.azure.endpoint = endpoint;

    Ok(())
}

fn configure_ollama(config: &mut AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("Ollama 配置").cyan().bold());

    let base_url: String = Input::new()
        .with_prompt("Base URL")
        .default(config.models.ollama.base_url.clone())
        .interact_text()?;

    config.models.ollama.base_url = base_url;

    let model: String = Input::new()
        .with_prompt("默认模型")
        .default(config.models.ollama.model.clone())
        .interact_text()?;

    config.models.ollama.model = model;

    Ok(())
}

fn configure_memory(config: &mut AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("记忆系统配置").cyan().bold());

    let database_url: String = Input::new()
        .with_prompt("PostgreSQL 连接字符串")
        .default(config.memory.database_url.clone())
        .interact_text()?;

    config.memory.database_url = database_url;

    Ok(())
}

fn configure_sandbox(config: &mut AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("沙箱配置").cyan().bold());

    let enabled = Confirm::new()
        .with_prompt("启用沙箱")
        .default(config.sandbox.enabled)
        .interact()?;

    config.sandbox.enabled = enabled;

    Ok(())
}
