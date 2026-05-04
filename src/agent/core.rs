use crate::models::provider::{Message, ModelProvider, ModelResponse};
use crate::memory::conversation::ConversationManager;
use crate::sandbox::executor::CommandExecutor;
use sqlx::types::Uuid;
use tracing::info;

/// AI Agent 核心
pub struct Agent {
    provider: Box<dyn ModelProvider>,
    conversation_manager: Option<ConversationManager>,
    command_executor: Option<CommandExecutor>,
    current_conversation_id: Option<Uuid>,
    system_prompt: String,
}

impl Agent {
    pub fn new(
        provider: Box<dyn ModelProvider>,
        conversation_manager: Option<ConversationManager>,
        command_executor: Option<CommandExecutor>,
    ) -> Self {
        Self {
            provider,
            conversation_manager,
            command_executor,
            current_conversation_id: None,
            system_prompt: "你是一个有用的AI助手。".to_string(),
        }
    }

    /// 设置系统提示
    pub fn set_system_prompt(&mut self, prompt: String) {
        self.system_prompt = prompt;
    }

    /// 开始新对话
    pub async fn start_conversation(&mut self, title: Option<String>) -> Result<Uuid, AgentError> {
        if let Some(ref manager) = self.conversation_manager {
            let id = manager
                .create_conversation(title)
                .await
                .map_err(|e| AgentError::MemoryError(e.to_string()))?;
            self.current_conversation_id = Some(id);
            Ok(id)
        } else {
            Err(AgentError::NoMemorySystem)
        }
    }

    /// 发送消息并获取回复
    pub async fn send_message(&self, user_input: &str) -> Result<ModelResponse, AgentError> {
        // 构建消息列表
        let mut messages = vec![Message::system(self.system_prompt.clone())];

        // 如果有历史对话，加载历史消息
        if let (Some(ref manager), Some(conv_id)) =
            (&self.conversation_manager, self.current_conversation_id)
        {
            let history = manager
                .get_messages(conv_id, Some(50))
                .await
                .map_err(|e| AgentError::MemoryError(e.to_string()))?;
            messages.extend(history);
        }

        // 添加用户消息
        messages.push(Message::user(user_input));

        // 调用模型
        info!("调用模型: {}", self.provider.name());
        let response = self
            .provider
            .chat(messages.clone())
            .await
            .map_err(|e| AgentError::ModelError(e.to_string()))?;

        // 保存消息到数据库
        if let (Some(ref manager), Some(conv_id)) =
            (&self.conversation_manager, self.current_conversation_id)
        {
            manager
                .add_message(
                    conv_id,
                    crate::models::provider::MessageRole::User,
                    user_input.to_string(),
                    None,
                    None,
                )
                .await
                .map_err(|e| AgentError::MemoryError(e.to_string()))?;

            manager
                .add_message(
                    conv_id,
                    crate::models::provider::MessageRole::Assistant,
                    response.content.clone(),
                    Some(response.model.clone()),
                    response.tokens_used,
                )
                .await
                .map_err(|e| AgentError::MemoryError(e.to_string()))?;
        }

        Ok(response)
    }

    /// 执行命令（使用沙箱）
    pub async fn execute_command(&self, command: &str) -> Result<String, AgentError> {
        if let Some(ref executor) = self.command_executor {
            let result = executor
                .execute(command)
                .await
                .map_err(|e| AgentError::ExecutionError(e.to_string()))?;

            if result.success {
                Ok(result.stdout)
            } else {
                Err(AgentError::ExecutionError(result.stderr))
            }
        } else {
            Err(AgentError::NoSandbox)
        }
    }

    /// 获取当前对话ID
    pub fn current_conversation_id(&self) -> Option<Uuid> {
        self.current_conversation_id
    }
}

/// Agent 错误类型
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("模型错误: {0}")]
    ModelError(String),

    #[error("记忆系统错误: {0}")]
    MemoryError(String),

    #[error("执行错误: {0}")]
    ExecutionError(String),

    #[error("未启用记忆系统")]
    NoMemorySystem,

    #[error("未启用沙箱")]
    NoSandbox,
}
