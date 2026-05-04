use crate::models::provider::{Message, ModelError, ModelProvider, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Qwen (通义千问 / Alibaba Cloud) 提供商
/// API文档: https://help.aliyun.com/zh/dashscope/developer-reference/api-details
pub struct QwenProvider {
    api_key: String,
    base_url: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct QwenRequest {
    model: String,
    messages: Vec<QwenMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct QwenMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct QwenResponse {
    choices: Vec<QwenChoice>,
    usage: Option<QwenUsage>,
}

#[derive(Debug, Deserialize)]
struct QwenChoice {
    message: QwenMessage,
}

#[derive(Debug, Deserialize)]
struct QwenUsage {
    total_tokens: Option<u32>,
}

impl QwenProvider {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            client: Client::new(),
        }
    }

    fn convert_messages(&self, messages: Vec<Message>) -> Vec<QwenMessage> {
        messages
            .into_iter()
            .map(|msg| QwenMessage {
                role: msg.role.to_string(),
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for QwenProvider {
    fn name(&self) -> &str {
        "Qwen (通义千问)"
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError> {
        let url = format!("{}/api/v1/services/aigc/text-generation/generation", self.base_url);

        let request = QwenRequest {
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
                "Qwen API 请求失败: {} - {}",
                status,
                error_text
            )));
        }

        let qwen_response: QwenResponse = response
            .json()
            .await
            .map_err(|e| ModelError::ParseError(e.to_string()))?;

        let choice = qwen_response
            .choices
            .first()
            .ok_or_else(|| ModelError::ParseError("响应中没有选择".to_string()))?;

        Ok(ModelResponse {
            content: choice.message.content.clone(),
            model: self.model.clone(),
            tokens_used: qwen_response.usage.and_then(|u| u.total_tokens),
        })
    }
}
