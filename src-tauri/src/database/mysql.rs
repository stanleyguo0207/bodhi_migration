// MySQL数据库连接实现

use super::connection::{DatabaseConnection, DatabaseConnectionType};
use mysql_async::prelude::*;
use mysql_async::{Conn, Opts, OptsBuilder};

// MySQL数据库配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MySQLConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

// MySQL数据库连接
#[derive(Clone, Debug)]
pub struct MySQLConnection {
    config: MySQLConfig,
    opts: Opts,
}

impl MySQLConnection {
    // 创建新的MySQL连接
    pub fn new(config: MySQLConfig) -> Result<Self, String> {
        let opts = OptsBuilder::from_opts(Opts::default())
            .ip_or_hostname(&config.host)
            .tcp_port(config.port)
            .user(Some(&config.username))
            .pass(Some(&config.password))
            .db_name(Some(&config.database))
            .into();
        
        Ok(Self {
            config,
            opts,
        })
    }
    
    // 创建连接池
    pub async fn get_connection(&self) -> Result<Conn, String> {
        Conn::new(self.opts.clone())
            .await
            .map_err(|e| format!("Failed to connect to MySQL database: {}", e))
    }
}

#[async_trait::async_trait]
impl DatabaseConnection for MySQLConnection {
    // 获取连接类型
    fn get_type(&self) -> DatabaseConnectionType {
        DatabaseConnectionType::MySQL
    }
    
    // 测试连接是否有效
    async fn test_connection(&self) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        
        // 执行简单查询测试连接，指定返回类型为Option<u8>
        conn.query_first::<Option<u8>, &str>("SELECT 1")
            .await
            .map_err(|e| format!("Connection test failed: {}", e))?;
        
        // MySQL连接通常由连接池管理，不需要手动关闭
        
        
        Ok(())
    }
    
    // 关闭连接（对于MySQL，连接通常由连接池管理，这里只是验证功能）
    async fn close(&self) -> Result<(), String> {
        // MySQL连接通常由连接池管理，这里我们可以简单返回成功
        Ok(())
    }
}