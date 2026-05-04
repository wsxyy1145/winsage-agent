use sqlx::PgPool;
use tracing::info;

/// 记忆存储管理器
#[derive(Clone)]
pub struct MemoryStorage {
    pool: PgPool,
}

impl MemoryStorage {
    /// 创建新的记忆存储实例
    pub async fn new(database_url: &str) -> Result<Self, MemoryError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| MemoryError::ConnectionError(e.to_string()))?;

        info!("成功连接到 PostgreSQL 数据库");

        Ok(Self { pool })
    }

    /// 获取数据库连接池
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// 运行数据库迁移
    pub async fn migrate(&self) -> Result<(), MemoryError> {
        // SQLx 的迁移功能需要在编译时嵌入迁移文件
        // 这里我们手动执行迁移SQL
        let migration_sql = include_str!("../../migrations/001_init.sql");

        // 分割SQL语句并逐个执行
        for statement in migration_sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() && !statement.starts_with("--") {
                sqlx::query(statement)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| MemoryError::MigrationError(e.to_string()))?;
            }
        }

        info!("数据库迁移完成");
        Ok(())
    }
}

/// 记忆系统错误类型
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("数据库连接失败: {0}")]
    ConnectionError(String),

    #[error("查询失败: {0}")]
    QueryError(String),

    #[error("迁移失败: {0}")]
    MigrationError(String),

    #[error("序列化失败: {0}")]
    SerializationError(String),
}
