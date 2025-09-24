// Redis 连接管理模块

use redis::{Client, AsyncCommands, aio::MultiplexedConnection};
use serde::{Serialize, Deserialize};

// Redis 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub db: Option<i64>,
}

// Redis 连接管理器
#[derive(Clone, Debug)]
pub struct RedisConnectionManager {
    config: RedisConfig,
    client: Client,
}

impl RedisConnectionManager {
    // 创建新的 Redis 连接管理器
    pub fn new(config: RedisConfig) -> Result<Self, String> {
        println!("创建Redis连接，URL: {}", config.url.replace("redis://", "").replace(":", "****"));
        
        let client = Client::open(&*config.url)
            .map_err(|e| format!("Failed to create Redis client with URL '{}': {}", config.url.replace("redis://", "").replace(":", "****"), e))?;
        
        Ok(Self {
            config,
            client,
        })
    }
    
    // 获取异步连接
    pub async fn get_connection(&self) -> Result<MultiplexedConnection, String> {
        self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("Failed to get Redis connection: {}", e))
    }
    
    // 测试连接
    pub async fn test_connection(&self) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        
        // 执行 PING 命令测试连接
        let result: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("Connection test failed: {}", e))?;
        
        if result == "PONG" {
            Ok(())
        } else {
            Err(format!("Unexpected PING response: {}", result))
        }
    }
    
    // 执行 Redis 命令
    pub async fn execute_command(&self, command: &str, args: Vec<String>) -> Result<String, String> {
        let mut conn = self.get_connection().await?;
        
        let result: String = redis::cmd(command)
            .arg(args)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("Failed to execute Redis command: {}", e))?;
        
        Ok(result)
    }
    
    // 获取字符串值
    pub async fn get_string(&self, key: &str) -> Result<Option<String>, String> {
        let mut conn = self.get_connection().await?;
        
        let result: Option<String> = conn.get(key)
            .await
            .map_err(|e| format!("Failed to get string value: {}", e))?;
        
        Ok(result)
    }
    
    // 设置字符串值
    pub async fn set_string(&self, key: &str, value: &str) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        
        conn.set::<_, _, ()>(key, value)
            .await
            .map_err(|e| format!("Failed to set string value: {}", e))?;
        
        Ok(())
    }
    
    // 删除键
    pub async fn delete_key(&self, key: &str) -> Result<bool, String> {
        let mut conn = self.get_connection().await?;
        
        let result: bool = conn.del(key)
            .await
            .map_err(|e| format!("Failed to delete key: {}", e))?;
        
        Ok(result)
    }
    
    // 获取数据库信息
    pub async fn get_info(&self) -> Result<String, String> {
        let mut conn = self.get_connection().await?;
        
        let info: String = redis::cmd("INFO")
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("Failed to get Redis info: {}", e))?;
        
        Ok(info)
    }
    
    // 关闭连接
    pub async fn close(&self) -> Result<(), String> {
        // Redis 连接由客户端管理，这里可以执行一些清理操作
        Ok(())
    }
}

// Redis 管理器，用于管理多个 Redis 连接
pub struct RedisManager {
    connections: std::collections::HashMap<String, RedisConnectionManager>,
}

impl RedisManager {
    pub fn new() -> Self {
        Self {
            connections: std::collections::HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, id: String, connection: RedisConnectionManager) {
        self.connections.insert(id, connection);
    }

    pub fn get_connection(&self, id: &str) -> Option<&RedisConnectionManager> {
        self.connections.get(id)
    }

    pub fn remove_connection(&mut self, id: &str) -> Option<RedisConnectionManager> {
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

impl Default for RedisManager {
    fn default() -> Self {
        Self::new()
    }
}