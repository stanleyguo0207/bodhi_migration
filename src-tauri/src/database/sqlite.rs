// SQLite数据库连接实现

use std::sync::{Arc, Mutex, MutexGuard};
use super::connection::{DatabaseConnection, DatabaseConnectionType};
use rusqlite::{Connection as RusqliteConnection, OpenFlags};

// SQLite数据库配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SQLiteConfig {
    pub db_path: String,
    pub read_only: bool,
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
        
        Ok(Self {
            config,
            connection: Arc::new(Mutex::new(Some(connection))),
        })
    }
    
    // 获取数据库连接
    pub fn get_connection(&self) -> Result<MutexGuard<Option<RusqliteConnection>>, String> {
        let conn = self.connection.lock().map_err(|e| format!("Failed to lock connection: {}", e))?;
        if conn.is_none() {
            return Err("Connection is closed".to_string());
        }
        Ok(conn)
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