use crate::models::provider::{Message, MessageRole, ModelError, ModelProvider, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Anthropic Claude 提供商
pub struct AnthropicProvider {
    api_key: String,
    base_url: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    model: String,
    usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

impl AnthropicProvider {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            client: Client::new(),
        }
    }

    fn convert_messages(&self, messages: Vec<Message>) -> Vec<AnthropicMessage> {
        messages
            .into_iter()
            .filter(|msg| msg.role != MessageRole::System) // Anthropic 单独处理 system
            .map(|msg| AnthropicMessage {
                role: match msg.role {
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                    MessageRole::System => "system".to_string(),
                },
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "Anthropic"
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError> {
        let url = format!("{}/messages", self.base_url);

        // 提取 system message
        let system_message = messages
            .iter()
            .find(|m| m.role == MessageRole::System)
            .map(|m| m.content.clone());

        let request = AnthropicRequest {
            model: self.model.clone(),
            messages: self.convert_messages(messages),
            max_tokens: 4096,
            temperature: 0.7,
        };

        let mut req_builder = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json");

        if let Some(_system) = system_message {
            req_builder = req_builder.header("anthropic-beta", "prompt-caching-2024-07-31");
        }

        let response = req_builder
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
                "API 请求失败: {} - {}",
                status,
                error_text
            )));
        }

        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| ModelError::ParseError(e.to_string()))?;

        let content = anthropic_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        let tokens_used = anthropic_response
            .usage
            .map(|u| u.input_tokens + u.output_tokens);

        Ok(ModelResponse {
            content,
            model: anthropic_response.model,
            tokens_used,
        })
    }
}
