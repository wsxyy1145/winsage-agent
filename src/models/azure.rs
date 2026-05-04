use crate::models::provider::{Message, ModelError, ModelProvider, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Azure OpenAI 提供商
pub struct AzureProvider {
    api_key: String,
    endpoint: String,
    deployment: String,
    api_version: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct AzureRequest {
    messages: Vec<AzureMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct AzureMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AzureResponse {
    choices: Vec<AzureChoice>,
    usage: Option<AzureUsage>,
}

#[derive(Debug, Deserialize)]
struct AzureChoice {
    message: AzureMessage,
}

#[derive(Debug, Deserialize)]
struct AzureUsage {
    total_tokens: Option<u32>,
}

impl AzureProvider {
    pub fn new(
        api_key: String,
        endpoint: String,
        deployment: String,
        api_version: String,
    ) -> Self {
        Self {
            api_key,
            endpoint,
            deployment,
            api_version,
            client: Client::new(),
        }
    }

    fn convert_messages(&self, messages: Vec<Message>) -> Vec<AzureMessage> {
        messages
            .into_iter()
            .map(|msg| AzureMessage {
                role: msg.role.to_string(),
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for AzureProvider {
    fn name(&self) -> &str {
        "Azure OpenAI"
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError> {
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint.trim_end_matches('/'),
            self.deployment,
            self.api_version
        );

        let request = AzureRequest {
            messages: self.convert_messages(messages),
            temperature: 0.7,
            max_tokens: 4096,
        };

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
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

        let azure_response: AzureResponse = response
            .json()
            .await
            .map_err(|e| ModelError::ParseError(e.to_string()))?;

        let choice = azure_response
            .choices
            .first()
            .ok_or_else(|| ModelError::ParseError("响应中没有选择".to_string()))?;

        Ok(ModelResponse {
            content: choice.message.content.clone(),
            model: self.deployment.clone(),
            tokens_used: azure_response.usage.and_then(|u| u.total_tokens),
        })
    }
}
