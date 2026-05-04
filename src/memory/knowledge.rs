use crate::memory::storage::MemoryStorage;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::types::Uuid;
use tracing::info;

/// 知识条目
#[derive(Debug, Clone)]
pub struct KnowledgeEntry {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 知识库管理器
pub struct KnowledgeManager {
    storage: MemoryStorage,
}

impl KnowledgeManager {
    pub fn new(storage: MemoryStorage) -> Self {
        Self { storage }
    }

    /// 添加知识条目
    pub async fn add_knowledge(
        &self,
        title: String,
        content: String,
        tags: Vec<String>,
    ) -> Result<Uuid, MemoryError> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO knowledge_base (id, title, content, tags) VALUES ($1, $2, $3, $4)",
            id,
            title,
            content,
            &tags
        )
        .execute(self.storage.pool())
        .await
        .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        info!("添加知识条目: {}", id);
        Ok(id)
    }

    /// 搜索知识条目（按标签）
    pub async fn search_by_tags(
        &self,
        tags: &[String],
    ) -> Result<Vec<KnowledgeEntry>, MemoryError> {
        let entries = sqlx::query_as!(
            KnowledgeEntry,
            r#"
            SELECT id, title, content, tags, 
                   metadata as "metadata: Value",
                   created_at, updated_at
            FROM knowledge_base
            WHERE tags && $1
            ORDER BY created_at DESC
            "#,
            tags
        )
        .fetch_all(self.storage.pool())
        .await
        .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        Ok(entries)
    }

    /// 获取所有知识条目
    pub async fn list_knowledge(&self) -> Result<Vec<KnowledgeEntry>, MemoryError> {
        let entries = sqlx::query_as!(
            KnowledgeEntry,
            r#"
            SELECT id, title, content, tags, 
                   metadata as "metadata: Value",
                   created_at, updated_at
            FROM knowledge_base
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.storage.pool())
        .await
        .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        Ok(entries)
    }

    /// 删除知识条目
    pub async fn delete_knowledge(&self, id: Uuid) -> Result<(), MemoryError> {
        sqlx::query!("DELETE FROM knowledge_base WHERE id = $1", id)
            .execute(self.storage.pool())
            .await
            .map_err(|e| MemoryError::QueryError(e.to_string()))?;

        info!("删除知识条目: {}", id);
        Ok(())
    }
}

use crate::memory::storage::MemoryError;
