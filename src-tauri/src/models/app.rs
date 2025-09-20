// 应用状态模型

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::database::connection::DatabaseConnectionManager;
use crate::migration::{MigrationPipeline};

// 应用状态
pub struct AppState {
    // 数据库连接管理器
    pub conn_manager: Arc<RwLock<DatabaseConnectionManager>>,
    // 迁移流水线
    pub migration_pipeline: Arc<RwLock<MigrationPipeline>>,
}

impl AppState {
    // 创建新的应用状态
    pub fn new() -> Self {
        let conn_manager = Arc::new(RwLock::new(DatabaseConnectionManager::new()));
        let migration_pipeline = Arc::new(RwLock::new(
            MigrationPipeline::new(conn_manager.clone())
        ));
        
        Self {
            conn_manager,
            migration_pipeline,
        }
    }
}