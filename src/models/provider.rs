use async_trait::async_trait;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};

/// 消息角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
            MessageRole::System => write!(f, "system"),
        }
    }
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
        }
    }
}

/// 模型响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub content: String,
    pub model: String,
    pub tokens_used: Option<u32>,
}

/// 模型错误类型
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("网络请求失败: {0}")]
    NetworkError(String),

    #[error("API 调用失败: {0}")]
    ApiError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("解析响应失败: {0}")]
    ParseError(String),
}

/// 模型提供者 trait
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// 获取提供商名称
    fn name(&self) -> &str;

    /// 发送聊天请求
    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError>;

    /// 流式聊天（可选实现）
    async fn stream_chat(
        &self,
        _messages: Vec<Message>,
    ) -> Result<BoxStream<'static, Result<String, ModelError>>, ModelError> {
        Err(ModelError::ApiError("流式聊天未实现".to_string()))
    }
}
