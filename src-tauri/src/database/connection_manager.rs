// 统一的数据库连接管理器

use crate::database::{DatabaseConfig, DatabaseType, SqlxDatabaseConnection, DatabaseManager, RedisConfig, RedisConnectionManager, RedisManager};
use std::sync::Arc;
use std::str::FromStr;
use tokio::sync::RwLock;

// 统一的数据库连接枚举
#[derive(Clone)]
pub enum UnifiedConnection {
    Sqlx(SqlxDatabaseConnection),
    Redis(RedisConnectionManager),
}

impl UnifiedConnection {
    pub async fn test_connection(&self) -> Result<(), String> {
        match self {
            UnifiedConnection::Sqlx(conn) => conn.test_connection().await,
            UnifiedConnection::Redis(conn) => conn.test_connection().await,
        }
    }

    pub fn get_type(&self) -> DatabaseType {
        match self {
            UnifiedConnection::Sqlx(_) => {
                // 由于SqlxDatabaseConnection现在是枚举，我们需要其他方式来确定类型
                // 暂时返回一个默认值，或者我们可以从配置中获取
                DatabaseType::SQLite // 临时默认值
            },
            UnifiedConnection::Redis(_) => DatabaseType::Redis,
        }
    }
}

// 统一的数据库连接管理器
pub struct UnifiedConnectionManager {
    sqlx_manager: Arc<RwLock<DatabaseManager>>,
    redis_manager: Arc<RwLock<RedisManager>>,
}

impl UnifiedConnectionManager {
    pub fn new() -> Self {
        Self {
            sqlx_manager: Arc::new(RwLock::new(DatabaseManager::new())),
            redis_manager: Arc::new(RwLock::new(RedisManager::new())),
        }
    }

    // 添加数据库连接
    pub async fn add_connection(&self, config: DatabaseConfig) -> Result<String, String> {
        let db_type = DatabaseType::from_str(&config.r#type)?;
        let connection_id = config.id.clone();

        match db_type {
            DatabaseType::SQLite | DatabaseType::MySQL | DatabaseType::PostgreSQL => {
                let connection = SqlxDatabaseConnection::new(config).await?;
                let mut manager = self.sqlx_manager.write().await;
                manager.add_connection(connection_id.clone(), connection);
            },
            DatabaseType::Redis => {
                // 对于Redis，我们直接使用host字段作为完整的URL，因为它已经包含了redis://前缀
                let redis_url = config.host.unwrap_or_else(|| "redis://localhost:6379".to_string());
                
                // 验证URL格式
                if !redis_url.starts_with("redis://") {
                    return Err(format!("Invalid Redis URL format: {}", redis_url));
                }
                
                let redis_config = RedisConfig {
                    url: redis_url,
                    db: config.database.map(|db| db.parse::<i64>().unwrap_or(0)),
                };
                let connection = RedisConnectionManager::new(redis_config)?;
                let mut manager = self.redis_manager.write().await;
                manager.add_connection(connection_id.clone(), connection);
            }
        }

        Ok(connection_id)
    }

    // 获取数据库连接
    pub async fn get_connection(&self, id: &str) -> Option<UnifiedConnection> {
        // 首先尝试从 SQLx 管理器获取
        let sqlx_manager = self.sqlx_manager.read().await;
        if let Some(conn) = sqlx_manager.get_connection(id) {
            return Some(UnifiedConnection::Sqlx(conn.clone()));
        }
        drop(sqlx_manager);

        // 然后尝试从 Redis 管理器获取
        let redis_manager = self.redis_manager.read().await;
        if let Some(conn) = redis_manager.get_connection(id) {
            return Some(UnifiedConnection::Redis(conn.clone()));
        }

        None
    }

    // 移除数据库连接
    pub async fn remove_connection(&self, id: &str) -> Result<(), String> {
        // 尝试从 SQLx 管理器移除
        let mut sqlx_manager = self.sqlx_manager.write().await;
        if sqlx_manager.remove_connection(id).is_some() {
            return Ok(());
        }
        drop(sqlx_manager);

        // 尝试从 Redis 管理器移除
        let mut redis_manager = self.redis_manager.write().await;
        if redis_manager.remove_connection(id).is_some() {
            return Ok(());
        }

        Err("Connection not found".to_string())
    }

    // 测试数据库连接
    pub async fn test_connection(&self, id: &str) -> Result<(), String> {
        if let Some(conn) = self.get_connection(id).await {
            conn.test_connection().await
        } else {
            Err("Connection not found".to_string())
        }
    }

    // 获取所有连接ID
    pub async fn get_all_connection_ids(&self) -> Vec<String> {
        let mut all_ids = Vec::new();

        let sqlx_manager = self.sqlx_manager.read().await;
        all_ids.extend(sqlx_manager.get_all_connection_ids());
        drop(sqlx_manager);

        let redis_manager = self.redis_manager.read().await;
        all_ids.extend(redis_manager.get_all_connection_ids());

        all_ids
    }

    // 获取连接类型
    pub async fn get_connection_type(&self, id: &str) -> Option<DatabaseType> {
        self.get_connection(id).await.map(|conn| conn.get_type())
    }

    // 获取 SQLx 连接（如果存在）
    pub async fn get_sqlx_connection(&self, id: &str) -> Option<SqlxDatabaseConnection> {
        let manager = self.sqlx_manager.read().await;
        manager.get_connection(id).cloned()
    }

    // 获取 Redis 连接（如果存在）
    pub async fn get_redis_connection(&self, id: &str) -> Option<RedisConnectionManager> {
        let manager = self.redis_manager.read().await;
        manager.get_connection(id).cloned()
    }
}

impl Default for UnifiedConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}