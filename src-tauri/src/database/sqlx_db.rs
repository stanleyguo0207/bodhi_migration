// 使用sqlx统一数据库操作模块

use serde::{Deserialize, Serialize};
use sqlx::{any::AnyPoolOptions, any::AnyRow, Any, Pool, Sqlite};
use std::str::FromStr;

// 数据库类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DatabaseType {
    Sqlite,
    MySql,
    PostgreSql,
    Redis,
}

impl DatabaseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseType::Sqlite => "sqlite",
            DatabaseType::MySql => "mysql",
            DatabaseType::PostgreSql => "postgresql",
            DatabaseType::Redis => "redis",
        }
    }
}

impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sqlite" => Ok(DatabaseType::Sqlite),
            "mysql" => Ok(DatabaseType::MySql),
            "postgresql" | "postgres" => Ok(DatabaseType::PostgreSql),
            "redis" => Ok(DatabaseType::Redis),
            _ => Err(format!("Unknown database type: {}", s)),
        }
    }
}

// 统一的数据库配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub host: Option<String>,
    pub port: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub ssl: bool,
    pub cluster: Option<bool>,
    pub extra: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// 统一的数据库连接结构体
#[derive(Clone, Debug)]
pub enum SqlxDatabaseConnection {
    Sqlite(Pool<Sqlite>),
    Any(Pool<Any>),
}

impl SqlxDatabaseConnection {
    // 创建新的数据库连接
    pub async fn new(config: DatabaseConfig) -> Result<Self, String> {
        let db_type = DatabaseType::from_str(&config.r#type)?;
        let connection_string = Self::build_connection_string(&config, &db_type)?;

        println!(
            "创建数据库连接，类型: {:?}, 连接字符串: {}",
            db_type,
            if connection_string.contains("://") {
                connection_string.replace("://", "://").replace(":", "****")
            } else {
                connection_string.clone()
            }
        );

        match db_type {
            DatabaseType::Sqlite => {
                // 为SQLite使用专用连接池
                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&connection_string)
                    .await
                    .map_err(|e| {
                        format!(
                            "Failed to connect to SQLite database: {} (连接字符串: {})",
                            e,
                            if connection_string.contains("://") {
                                connection_string.replace("://", "://").replace(":", "****")
                            } else {
                                connection_string.clone()
                            }
                        )
                    })?;
                Ok(SqlxDatabaseConnection::Sqlite(pool))
            }
            _ => {
                // 其他数据库使用Any连接池
                let pool = AnyPoolOptions::new()
                    .max_connections(5)
                    .connect(&connection_string)
                    .await
                    .map_err(|e| {
                        format!(
                            "Failed to connect to database: {} (连接字符串: {})",
                            e,
                            if connection_string.contains("://") {
                                connection_string.replace("://", "://").replace(":", "****")
                            } else {
                                connection_string.clone()
                            }
                        )
                    })?;
                Ok(SqlxDatabaseConnection::Any(pool))
            }
        }
    }

    // 构建连接字符串
    fn build_connection_string(
        config: &DatabaseConfig,
        db_type: &DatabaseType,
    ) -> Result<String, String> {
        match db_type {
            DatabaseType::Sqlite => {
                let db_path = config
                    .database
                    .as_ref()
                    .ok_or_else(|| "SQLite database path is required".to_string())?;

                // 检查路径是否包含非法字符
                if db_path.contains('\t') || db_path.contains('\n') {
                    return Err(format!(
                        "SQLite database path contains invalid characters: {}",
                        db_path
                    ));
                }

                // 根据测试结果，Windows下SQLite连接URL格式为: sqlite:D\path\to\file.db
                // 使用反斜杠，不需要额外的斜杠前缀
                if cfg!(windows) {
                    // 处理UNC路径格式 (\\?\C:\path\to\file.db)
                    if db_path.starts_with("\\\\?\\") {
                        // 移除UNC前缀，直接使用标准路径
                        let standard_path = &db_path[4..];
                        Ok(format!("sqlite:{}", standard_path))
                    } else if db_path.starts_with("\\") {
                        // 网络路径
                        Ok(format!("sqlite:{}", db_path))
                    } else {
                        // 本地路径和相对路径，直接使用原始路径
                        Ok(format!("sqlite:{}", db_path))
                    }
                } else {
                    // Linux/macOS使用标准URI格式
                    Ok(format!("sqlite://{}", db_path))
                }
            }
            DatabaseType::MySql => {
                let host = config
                    .host
                    .as_ref()
                    .ok_or_else(|| "MySQL host is required".to_string())?;
                let port = config.port.unwrap_or(3306);
                let username = config
                    .username
                    .as_ref()
                    .ok_or_else(|| "MySQL username is required".to_string())?;
                let password = config
                    .password
                    .as_ref()
                    .map(|p| format!(":{}@", p))
                    .unwrap_or_else(|| "@".to_string());
                let database = config
                    .database
                    .as_ref()
                    .ok_or_else(|| "MySQL database is required".to_string())?;

                let ssl_param = if config.ssl { "&ssl-mode=required" } else { "" };

                Ok(format!(
                    "mysql://{}{}:{}:{}/{}{}",
                    username, password, host, port, database, ssl_param
                ))
            }
            DatabaseType::PostgreSql => {
                let host = config
                    .host
                    .as_ref()
                    .ok_or_else(|| "PostgreSQL host is required".to_string())?;
                let port = config.port.unwrap_or(5432);
                let username = config
                    .username
                    .as_ref()
                    .ok_or_else(|| "PostgreSQL username is required".to_string())?;
                let password = config
                    .password
                    .as_ref()
                    .map(|p| format!(":{}@", p))
                    .unwrap_or_else(|| "@".to_string());
                let database = config
                    .database
                    .as_ref()
                    .ok_or_else(|| "PostgreSQL database is required".to_string())?;

                let ssl_param = if config.ssl { "?sslmode=require" } else { "" };

                Ok(format!(
                    "postgresql://{}{}:{}:{}/{}{}",
                    username, password, host, port, database, ssl_param
                ))
            }
            DatabaseType::Redis => {
                // Redis 使用不同的连接方式，这里返回错误
                Err("Redis is not supported in sqlx, use redis crate separately".to_string())
            }
        }
    }

    // 测试连接
    pub async fn test_connection(&self) -> Result<(), String> {
        match self {
            SqlxDatabaseConnection::Sqlite(pool) => {
                sqlx::query("SELECT 1")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| format!("Connection test failed: {}", e))?;
            }
            SqlxDatabaseConnection::Any(pool) => {
                sqlx::query("SELECT 1")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| format!("Connection test failed: {}", e))?;
            }
        }

        Ok(())
    }

    // 执行原始SQL查询
    pub async fn execute_raw(&self, sql: &str) -> Result<(), String> {
        match self {
            SqlxDatabaseConnection::Sqlite(pool) => {
                sqlx::query(sql)
                    .execute(pool)
                    .await
                    .map_err(|e| format!("Failed to execute query: {}", e))?;
            }
            SqlxDatabaseConnection::Any(pool) => {
                sqlx::query(sql)
                    .execute(pool)
                    .await
                    .map_err(|e| format!("Failed to execute query: {}", e))?;
            }
        }

        Ok(())
    }

    // 获取单行数据 - 简化版本，直接返回错误
    pub async fn fetch_one(&self, _sql: &str) -> Result<AnyRow, String> {
        Err("fetch_one not implemented for current connection type".to_string())
    }

    // 获取所有行数据 - SQLite专用版本
    pub async fn fetch_all_sqlite(
        &self,
        sql: &str,
    ) -> Result<Vec<sqlx::sqlite::SqliteRow>, String> {
        match self {
            SqlxDatabaseConnection::Sqlite(pool) => {
                let rows = sqlx::query(sql)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| format!("Failed to fetch all rows: {}", e))?;
                Ok(rows)
            }
            SqlxDatabaseConnection::Any(_pool) => {
                Err("fetch_all_sqlite can only be used with SQLite connections".to_string())
            }
        }
    }

    // 关闭连接
    pub async fn close(&self) -> Result<(), String> {
        match self {
            SqlxDatabaseConnection::Sqlite(pool) => {
                pool.close().await;
            }
            SqlxDatabaseConnection::Any(pool) => {
                pool.close().await;
            }
        }
        Ok(())
    }

    // 执行INSERT/UPDATE/DELETE语句并返回影响行数
    pub async fn execute(&self, sql: &str) -> Result<u64, String> {
        match self {
            SqlxDatabaseConnection::Sqlite(pool) => {
                let result = sqlx::query(sql)
                    .execute(pool)
                    .await
                    .map_err(|e| format!("Failed to execute SQL: {}", e))?;
                Ok(result.rows_affected())
            }
            SqlxDatabaseConnection::Any(pool) => {
                let result = sqlx::query(sql)
                    .execute(pool)
                    .await
                    .map_err(|e| format!("Failed to execute SQL: {}", e))?;
                Ok(result.rows_affected())
            }
        }
    }

    // 执行带参数的INSERT语句
    pub async fn execute_with_params(&self, sql: &str, params: Vec<String>) -> Result<u64, String> {
        match self {
            SqlxDatabaseConnection::Sqlite(pool) => {
                let mut query = sqlx::query(sql);
                for param in params {
                    query = query.bind(param);
                }
                let result = query
                    .execute(pool)
                    .await
                    .map_err(|e| format!("Failed to execute SQL with params: {}", e))?;
                Ok(result.rows_affected())
            }
            SqlxDatabaseConnection::Any(pool) => {
                let mut query = sqlx::query(sql);
                for param in params {
                    query = query.bind(param);
                }
                let result = query
                    .execute(pool)
                    .await
                    .map_err(|e| format!("Failed to execute SQL with params: {}", e))?;
                Ok(result.rows_affected())
            }
        }
    }
}

// 数据库管理器，用于管理多个数据库连接
pub struct DatabaseManager {
    connections: std::collections::HashMap<String, SqlxDatabaseConnection>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            connections: std::collections::HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, id: String, connection: SqlxDatabaseConnection) {
        self.connections.insert(id, connection);
    }

    pub fn get_connection(&self, id: &str) -> Option<&SqlxDatabaseConnection> {
        self.connections.get(id)
    }

    pub fn remove_connection(&mut self, id: &str) -> Option<SqlxDatabaseConnection> {
        self.connections.remove(id)
    }

    pub async fn test_connection(&self, id: &str) -> Result<(), String> {
        if let Some(connection) = self.get_connection(id) {
            connection.test_connection().await
        } else {
            Err("Connection not found".to_string())
        }
    }

    pub fn get_all_connection_ids(&self) -> Vec<String> {
        self.connections.keys().cloned().collect()
    }
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}
