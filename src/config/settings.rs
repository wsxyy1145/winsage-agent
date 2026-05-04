use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用主配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub models: ModelsConfig,
    pub memory: MemoryConfig,
    pub sandbox: SandboxConfig,
}

/// 通用配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub default_model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub streaming: bool,
}

/// 模型配置集合
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelsConfig {
    pub openai: ModelProviderConfig,
    pub anthropic: ModelProviderConfig,
    pub azure: AzureConfig,
    pub ollama: OllamaConfig,
    pub deepseek: ModelProviderConfig,
    pub moonshot: ModelProviderConfig,
    pub qwen: ModelProviderConfig,
    pub doubao: ModelProviderConfig,
}

/// 模型提供商配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelProviderConfig {
    pub api_key: String,
    pub base_url: String,
    pub default_model: String,
}

/// Azure AI 配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AzureConfig {
    pub api_key: String,
    pub endpoint: String,
    pub deployment: String,
    pub api_version: String,
}

/// Ollama 配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OllamaConfig {
    pub base_url: String,
    pub model: String,
}

/// 记忆系统配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryConfig {
    pub database_url: String,
    pub enable_vector_search: bool,
    pub max_history_length: usize,
}

/// 沙箱配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub auto_cleanup: bool,
    pub timeout_seconds: u64,
    pub shared_folder: Option<String>,
}

impl AppConfig {
    /// 从默认配置文件加载
    pub fn load() -> Result<Self, ConfigError> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        if !config_file.exists() {
            // 创建默认配置
            Self::create_default_config(&config_file)?;
        }

        let settings = config::Config::builder()
            .add_source(config::File::from(config_file.clone()))
            .add_source(config::Environment::with_prefix("WINSAGE").separator("_"))
            .build()
            .map_err(|e| ConfigError::LoadError(e.to_string()))?;

        settings
            .try_deserialize::<AppConfig>()
            .map_err(|e| ConfigError::ParseError(e.to_string()))
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), ConfigError> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        let toml_content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::SerializeError(e.to_string()))?;

        std::fs::write(config_file, toml_content)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;

        Ok(())
    }

    /// 获取配置目录
    fn get_config_dir() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ConfigError::DirectoryError("无法获取配置目录".to_string()))?
            .join("winsage");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)
                .map_err(|e| ConfigError::DirectoryError(format!("创建配置目录失败: {}", e)))?;
        }

        Ok(config_dir)
    }

    /// 创建默认配置文件
    fn create_default_config(path: &PathBuf) -> Result<(), ConfigError> {
        let default_config = include_str!("../../config/default.toml");
        std::fs::write(path, default_config)
            .map_err(|e| ConfigError::WriteError(format!("创建默认配置失败: {}", e)))?;
        Ok(())
    }
}

/// 配置错误类型
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("加载配置失败: {0}")]
    LoadError(String),

    #[error("解析配置失败: {0}")]
    ParseError(String),

    #[error("序列化配置失败: {0}")]
    SerializeError(String),

    #[error("写入配置失败: {0}")]
    WriteError(String),

    #[error("目录错误: {0}")]
    DirectoryError(String),
}
