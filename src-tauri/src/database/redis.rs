// Redis数据库连接实现

use super::connection::{DatabaseConnection, DatabaseConnectionType};
use redis::{Client, Connection};
use redis::aio::MultiplexedConnection as AsyncConnection;
use redis::aio::MultiplexedConnection;

// Redis数据库配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub db: Option<usize>,
}

// Redis数据库连接
#[derive(Clone, Debug)]
pub struct RedisConnection {
    config: RedisConfig,
    client: Client,
}

impl RedisConnection {
    // 创建新的Redis连接
    pub fn new(config: RedisConfig) -> Result<Self, String> {
        let client = Client::open(&*config.url)
            .map_err(|e| format!("Failed to create Redis client: {}", e))?;
        
        Ok(Self {
            config,
            client,
        })
    }
    
    // 获取同步连接
    pub fn get_sync_connection(&self) -> Result<Connection, String> {
        self.client.get_connection()
            .map_err(|e| format!("Failed to get Redis connection: {}", e))
    }
    
    // 获取异步连接
    pub async fn get_async_connection(&self) -> Result<AsyncConnection, String> {
        self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("Failed to get Redis async connection: {}", e))
    }
    
    // 获取多路复用异步连接
    pub async fn get_multiplexed_async_connection(&self) -> Result<MultiplexedConnection, String> {
        self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("Failed to get Redis multiplexed connection: {}", e))
    }
}

#[async_trait::async_trait]
impl DatabaseConnection for RedisConnection {
    // 获取连接类型
    fn get_type(&self) -> DatabaseConnectionType {
        DatabaseConnectionType::Redis
    }
    
    // 测试连接是否有效
    async fn test_connection(&self) -> Result<(), String> {
        let mut conn = self.get_async_connection().await?;
        
        // 执行PING命令测试连接
        redis::cmd("PING").query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| format!("Connection test failed: {}", e))?;
        
        Ok(())
    }
    
    // 关闭连接（对于Redis，连接通常由客户端管理，这里只是验证功能）
    async fn close(&self) -> Result<(), String> {
        // Redis连接通常由客户端管理，这里我们可以简单返回成功
        Ok(())
    }
}