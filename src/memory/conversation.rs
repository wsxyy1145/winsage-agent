use crate::memory::storage::MemoryStorage;
use crate::models::provider::{Message, MessageRole};
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use tracing::info;

/// 对话记录
#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub title: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 消息记录
#[derive(Debug, Clone)]
pub struct MessageRecord {
    pub id: Option<Uuid>,
    pub conversation_id: Option<Uuid>,
    pub role: String,
    pub content: String,
    pub model: Option<String>,
    pub tokens: Option<i32>,
    pub timestamp: Option<DateTime<Utc>>,
}

/// 对话管理器
pub struct ConversationManager {
    storage: MemoryStorage,
}

impl ConversationManager {
    pub fn new(storage: MemoryStorage) -> Self {
        Self { storage }
    }

    /// 创建新对话
    pub async fn create_conversation(&self, title: Option<String>) -> Result<Uuid, MemoryError> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO conversations (id, title) VALUES ($1, $2)",
            id,
            title
        )
        .execute(self.storage.pool())
        .await
        .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        info!("创建新对话: {}", id);
        Ok(id)
    }

    /// 添加消息到对话
    pub async fn add_message(
        &self,
        conversation_id: Uuid,
        role: MessageRole,
        content: String,
        model: Option<String>,
        tokens: Option<u32>,
    ) -> Result<Uuid, MemoryError> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO messages (id, conversation_id, role, content, model, tokens) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            id,
            conversation_id,
            role.to_string(),
            content,
            model,
            tokens.map(|t| t as i32)
        )
        .execute(self.storage.pool())
        .await
        .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        Ok(id)
    }

    /// 获取对话的所有消息
    pub async fn get_messages(
        &self,
        conversation_id: Uuid,
        limit: Option<usize>,
    ) -> Result<Vec<Message>, MemoryError> {
        let limit = limit.unwrap_or(100);

        let records = sqlx::query_as!(
            MessageRecord,
            r#"
            SELECT id, conversation_id, role, content, model, tokens, timestamp
            FROM messages
            WHERE conversation_id = $1
            ORDER BY timestamp ASC
            LIMIT $2
            "#,
            conversation_id,
            limit as i64
        )
        .fetch_all(self.storage.pool())
        .await
        .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        let messages = records
            .into_iter()
            .filter_map(|r| {
                // 过滤掉无法解析的消息
                let role = match r.role.as_str() {
                    "user" => MessageRole::User,
                    "assistant" => MessageRole::Assistant,
                    "system" => MessageRole::System,
                    _ => return None,
                };
                Some(Message {
                    role,
                    content: r.content,
                })
            })
            .collect();

        Ok(messages)
    }

    /// 获取最近的对话列表
    pub async fn list_conversations(
        &self,
        limit: usize,
    ) -> Result<Vec<Conversation>, MemoryError> {
        let conversations = sqlx::query_as!(
            Conversation,
            r#"
            SELECT id, title, created_at, updated_at
            FROM conversations
            ORDER BY updated_at DESC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(self.storage.pool())
        .await
        .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        Ok(conversations)
    }

    /// 删除对话及其所有消息
    pub async fn delete_conversation(&self, id: Uuid) -> Result<(), MemoryError> {
        sqlx::query!("DELETE FROM conversations WHERE id = $1", id)
            .execute(self.storage.pool())
            .await
            .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        info!("删除对话: {}", id);
        Ok(())
    }
}

use crate::memory::storage::MemoryError;
