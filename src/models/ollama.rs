use crate::models::provider::{Message, ModelError, ModelProvider, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Ollama 提供商
pub struct OllamaProvider {
    base_url: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
    #[allow(dead_code)]
    done: bool,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            model,
            client: Client::new(),
        }
    }

    fn convert_messages(&self, messages: Vec<Message>) -> Vec<OllamaMessage> {
        messages
            .into_iter()
            .map(|msg| OllamaMessage {
                role: msg.role.to_string(),
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError> {
        let url = format!("{}/api/chat", self.base_url.trim_end_matches('/'));

        let request = OllamaRequest {
            model: self.model.clone(),
            messages: self.convert_messages(messages),
            stream: false,
        };

        let response = self
            .client
            .post(&url)
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
                "API 请求失败: {} - {}",
                status,
                error_text
            )));
        }

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| ModelError::ParseError(e.to_string()))?;

        Ok(ModelResponse {
            content: ollama_response.message.content,
            model: self.model.clone(),
            tokens_used: None,
        })
    }
}
