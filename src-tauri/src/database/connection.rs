// 数据库连接管理模块

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// 数据库连接类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DatabaseConnectionType {
    SQLite,
    MySQL,
    Redis,
}

// 导入具体的数据库连接实现
use super::mysql::MySQLConnection;
use super::sqlite::SQLiteConnection;
use super::redis::RedisConnection;

// 数据库连接特性
#[async_trait::async_trait]
pub trait DatabaseConnection: Send + Sync {
    // 获取连接类型
    fn get_type(&self) -> DatabaseConnectionType;
    
    // 测试连接是否有效
    async fn test_connection(&self) -> Result<(), String>;
    
    // 关闭连接
    async fn close(&self) -> Result<(), String>;
}

// 数据库连接枚举，用于替代dyn DatabaseConnection
#[derive(Debug, Clone)]
pub enum DatabaseConnectionEnum {
    SQLite(SQLiteConnection),
    MySQL(MySQLConnection),
    Redis(RedisConnection),
}

// 为DatabaseConnectionEnum添加直接的方法
impl DatabaseConnectionEnum {
    // 获取连接类型
    pub fn get_type(&self) -> DatabaseConnectionType {
        match self {
            DatabaseConnectionEnum::SQLite(_) => DatabaseConnectionType::SQLite,
            DatabaseConnectionEnum::MySQL(_) => DatabaseConnectionType::MySQL,
            DatabaseConnectionEnum::Redis(_) => DatabaseConnectionType::Redis,
        }
    }
    
    // 测试连接是否有效
    pub async fn test_connection(&self) -> Result<(), String> {
        match self {
            DatabaseConnectionEnum::SQLite(conn) => conn.test_connection().await,
            DatabaseConnectionEnum::MySQL(conn) => conn.test_connection().await,
            DatabaseConnectionEnum::Redis(conn) => conn.test_connection().await,
        }
    }
    
    // 关闭连接
    pub async fn close(&self) -> Result<(), String> {
        match self {
            DatabaseConnectionEnum::SQLite(conn) => conn.close().await,
            DatabaseConnectionEnum::MySQL(conn) => conn.close().await,
            DatabaseConnectionEnum::Redis(conn) => conn.close().await,
        }
    }
}

// 数据库连接管理器
pub struct DatabaseConnectionManager {
    connections: Arc<RwLock<HashMap<String, DatabaseConnectionEnum>>>,
}

impl DatabaseConnectionManager {
    // 创建新的连接管理器
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    // 添加数据库连接
    pub async fn add_connection(
        &self,
        connection: DatabaseConnectionEnum,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let mut connections = self.connections.write().await;
        connections.insert(id.clone(), connection);
        id
    }
    
    // 获取数据库连接
    pub async fn get_connection(
        &self,
        id: &str,
    ) -> Option<DatabaseConnectionEnum> {
        let connections = self.connections.read().await;
        connections.get(id).cloned()
    }
    
    // 移除数据库连接
    pub async fn remove_connection(&self, id: &str) -> Option<DatabaseConnectionEnum> {
        let mut connections = self.connections.write().await;
        connections.remove(id)
    }
    
    // 测试连接是否有效
    pub async fn test_connection(&self, id: &str) -> Result<(), String> {
        let connections = self.connections.read().await;
        if let Some(connection) = connections.get(id) {
            connection.test_connection().await
        } else {
            Err("Connection not found".to_string())
        }
    }
}