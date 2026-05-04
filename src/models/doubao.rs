use crate::models::provider::{Message, ModelError, ModelProvider, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Doubao (豆包 / ByteDance) 提供商
/// API文档: https://www.volcengine.com/docs/82379/1263482
pub struct DoubaoProvider {
    api_key: String,
    base_url: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct DoubaoRequest {
    model: String,
    messages: Vec<DoubaoMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DoubaoMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DoubaoResponse {
    choices: Vec<DoubaoChoice>,
    usage: Option<DoubaoUsage>,
}

#[derive(Debug, Deserialize)]
struct DoubaoChoice {
    message: DoubaoMessage,
}

#[derive(Debug, Deserialize)]
struct DoubaoUsage {
    total_tokens: Option<u32>,
}

impl DoubaoProvider {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            client: Client::new(),
        }
    }

    fn convert_messages(&self, messages: Vec<Message>) -> Vec<DoubaoMessage> {
        messages
            .into_iter()
            .map(|msg| DoubaoMessage {
                role: msg.role.to_string(),
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for DoubaoProvider {
    fn name(&self) -> &str {
        "Doubao (豆包)"
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError> {
        let url = format!("{}/api/v3/chat/completions", self.base_url);

        let request = DoubaoRequest {
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
                "Doubao API 请求失败: {} - {}",
                status,
                error_text
            )));
        }

        let doubao_response: DoubaoResponse = response
            .json()
            .await
            .map_err(|e| ModelError::ParseError(e.to_string()))?;

        let choice = doubao_response
            .choices
            .first()
            .ok_or_else(|| ModelError::ParseError("响应中没有选择".to_string()))?;

        Ok(ModelResponse {
            content: choice.message.content.clone(),
            model: self.model.clone(),
            tokens_used: doubao_response.usage.and_then(|u| u.total_tokens),
        })
    }
}
