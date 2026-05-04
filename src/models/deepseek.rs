use crate::models::provider::{Message, ModelError, ModelProvider, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// DeepSeek 提供商
/// API文档: https://platform.deepseek.com/api-docs/
pub struct DeepSeekProvider {
    api_key: String,
    base_url: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
    usage: Option<DeepSeekUsage>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekMessage,
}

#[derive(Debug, Deserialize)]
struct DeepSeekUsage {
    total_tokens: Option<u32>,
}

impl DeepSeekProvider {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            client: Client::new(),
        }
    }

    fn convert_messages(&self, messages: Vec<Message>) -> Vec<DeepSeekMessage> {
        messages
            .into_iter()
            .map(|msg| DeepSeekMessage {
                role: msg.role.to_string(),
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for DeepSeekProvider {
    fn name(&self) -> &str {
        "DeepSeek"
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError> {
        let url = format!("{}/chat/completions", self.base_url);

        let request = DeepSeekRequest {
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
                "DeepSeek API 请求失败: {} - {}",
                status,
                error_text
            )));
        }

        let deepseek_response: DeepSeekResponse = response
            .json()
            .await
            .map_err(|e| ModelError::ParseError(e.to_string()))?;

        let choice = deepseek_response
            .choices
            .first()
            .ok_or_else(|| ModelError::ParseError("响应中没有选择".to_string()))?;

        Ok(ModelResponse {
            content: choice.message.content.clone(),
            model: self.model.clone(),
            tokens_used: deepseek_response.usage.and_then(|u| u.total_tokens),
        })
    }
}
