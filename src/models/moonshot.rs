use crate::models::provider::{Message, ModelError, ModelProvider, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Kimi (Moonshot AI) 提供商
/// API文档: https://platform.moonshot.cn/docs/
pub struct MoonshotProvider {
    api_key: String,
    base_url: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct MoonshotRequest {
    model: String,
    messages: Vec<MoonshotMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MoonshotMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct MoonshotResponse {
    choices: Vec<MoonshotChoice>,
    usage: Option<MoonshotUsage>,
}

#[derive(Debug, Deserialize)]
struct MoonshotChoice {
    message: MoonshotMessage,
}

#[derive(Debug, Deserialize)]
struct MoonshotUsage {
    total_tokens: Option<u32>,
}

impl MoonshotProvider {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            client: Client::new(),
        }
    }

    fn convert_messages(&self, messages: Vec<Message>) -> Vec<MoonshotMessage> {
        messages
            .into_iter()
            .map(|msg| MoonshotMessage {
                role: msg.role.to_string(),
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for MoonshotProvider {
    fn name(&self) -> &str {
        "Kimi (Moonshot)"
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError> {
        let url = format!("{}/v1/chat/completions", self.base_url);

        let request = MoonshotRequest {
            model: self.model.clone(),
            messages: self.convert_messages(messages),
            temperature: 0.7,
            max_tokens: 4096,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ModelError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ModelError::ApiError(format!(
                "Kimi API 请求失败: {} - {}",
                status,
                error_text
            )));
        }

        let moonshot_response: MoonshotResponse = response
            .json()
            .await
            .map_err(|e| ModelError::ParseError(e.to_string()))?;

        let choice = moonshot_response
            .choices
            .first()
            .ok_or_else(|| ModelError::ParseError("响应中没有选择".to_string()))?;

        Ok(ModelResponse {
            content: choice.message.content.clone(),
            model: self.model.clone(),
            tokens_used: moonshot_response.usage.and_then(|u| u.total_tokens),
        })
    }
}
