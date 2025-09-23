// SQLite数据库连接实现

use std::sync::{Arc, Mutex, MutexGuard};
use super::connection::{DatabaseConnection, DatabaseConnectionType};
use rusqlite::{Connection as RusqliteConnection, OpenFlags};
use serde::{Serialize, Deserialize};

// SQLite数据库配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SQLiteConfig {
    pub db_path: String,
    pub read_only: bool,
}

// 数据库配置结构体，用于持久化
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
    pub extra: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// SQLite数据库连接
#[derive(Clone, Debug)]
pub struct SQLiteConnection {
    config: SQLiteConfig,
    connection: Arc<Mutex<Option<RusqliteConnection>>>,
}

impl SQLiteConnection {
    // 创建新的SQLite连接
    pub fn new(config: SQLiteConfig) -> Result<Self, String> {
        let mut flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE;
        
        if config.read_only {
            flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        }
        
        let connection = RusqliteConnection::open_with_flags(&config.db_path, flags)
            .map_err(|e| format!("Failed to open SQLite database: {}", e))?;
        
        // 初始化数据库表
        Self::init_database(&connection).map_err(|e| format!("Failed to initialize database: {}", e))?;
        
        Ok(Self {
            config,
            connection: Arc::new(Mutex::new(Some(connection))),
        })
    }
    
    // 初始化数据库表
    fn init_database(conn: &RusqliteConnection) -> Result<(), rusqlite::Error> {
        // 创建数据库配置表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS database_configs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                host TEXT,
                port INTEGER,
                username TEXT,
                password TEXT,
                database TEXT,
                ssl INTEGER DEFAULT 0,
                extra TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }
    
    // 获取数据库连接
    pub fn get_connection(&self) -> Result<MutexGuard<Option<RusqliteConnection>>, String> {
        let conn = self.connection.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
        if conn.is_none() {
            return Err("Connection is closed".to_string());
        }
        Ok(conn)
    }
    
    // 保存数据库配置
    pub fn save_database_config(&self, config: &DatabaseConfig) -> Result<(), String> {
        let conn = self.get_connection()?;
        let conn = conn.as_ref().ok_or("Connection is closed".to_string())?;
        
        conn.execute(
            "INSERT INTO database_configs (id, name, type, host, port, username, password, database, ssl, extra, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                type = excluded.type,
                host = excluded.host,
                port = excluded.port,
                username = excluded.username,
                password = excluded.password,
                database = excluded.database,
                ssl = excluded.ssl,
                extra = excluded.extra,
                updated_at = excluded.updated_at",
            ( 
                &config.id, 
                &config.name, 
                &config.r#type, 
                &config.host, 
                &config.port, 
                &config.username, 
                &config.password, 
                &config.database, 
                &(config.ssl as i32), 
                &config.extra, 
                &config.created_at, 
                &config.updated_at 
            ),
        ).map_err(|e| format!("Failed to save database config: {}", e))?;
        
        Ok(())
    }
    
    // 获取所有数据库配置
    pub fn get_all_database_configs(&self) -> Result<Vec<DatabaseConfig>, String> {
        let conn = self.get_connection()?;
        let conn = conn.as_ref().ok_or("Connection is closed".to_string())?;
        
        let mut stmt = conn.prepare(
            "SELECT id, name, type, host, port, username, password, database, ssl, extra, created_at, updated_at FROM database_configs"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let configs = stmt.query_map([], |row| {
            // 正确获取ssl字段（数据库中是INTEGER类型）
            let ssl_value: i32 = row.get(8)?;
            
            Ok(DatabaseConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                r#type: row.get(2)?,
                host: row.get(3)?,
                port: row.get(4)?,
                username: row.get(5)?,
                password: row.get(6)?,
                database: row.get(7)?,
                ssl: ssl_value > 0,
                extra: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        }).map_err(|e| format!("Failed to query database configs: {}", e))?;
        
        let mut results = Vec::new();
        for config in configs {
            results.push(config.map_err(|e| format!("Failed to read database config: {}", e))?);
        }
        
        Ok(results)
    }
    
    // 删除数据库配置
    pub fn delete_database_config(&self, id: &str) -> Result<(), String> {
        let conn = self.get_connection()?;
        let conn = conn.as_ref().ok_or("Connection is closed".to_string())?;
        
        conn.execute("DELETE FROM database_configs WHERE id = ?", [id])
            .map_err(|e| format!("Failed to delete database config: {}", e))?;
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl DatabaseConnection for SQLiteConnection {
    // 获取连接类型
    fn get_type(&self) -> DatabaseConnectionType {
        DatabaseConnectionType::SQLite
    }
    
    // 测试连接是否有效
    async fn test_connection(&self) -> Result<(), String> {
        // 由于SQLite是本地文件数据库，连接状态取决于文件是否可访问
        // 我们可以尝试一个简单的查询来验证连接
        let conn = self.connection.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
        let conn = conn.as_ref().ok_or("Connection is closed".to_string())?;
        
        conn.execute("SELECT 1", [])
            .map_err(|e| format!("Connection test failed: {}", e))?;
        
        Ok(())
    }
    
    // 关闭连接
    async fn close(&self) -> Result<(), String> {
        // SQLite连接在Drop时会自动关闭，这里我们只需要验证连接是否有效
        let mut conn = self.connection.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
        if conn.is_none() {
            return Err("Connection is already closed".to_string());
        }
        
        *conn = None;
        Ok(())
    }
}

// 当SQLiteConnection被丢弃时自动关闭连接
impl Drop for SQLiteConnection {
    fn drop(&mut self) {
        // SQLite连接会自动关闭
    }
}