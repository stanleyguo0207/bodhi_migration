// 迁移流水线模块

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::database::connection::DatabaseConnectionManager;
use crate::migration::MigrationTask;
use crate::migration::task::TaskStatus;
use crate::migration::task::TaskProgress;

// 迁移流水线
pub struct MigrationPipeline {
    tasks: Arc<RwLock<HashMap<String, MigrationTask>>>,
    conn_manager: Arc<RwLock<DatabaseConnectionManager>>,
}

impl MigrationPipeline {
    // 创建新的迁移流水线
    pub fn new(conn_manager: Arc<RwLock<DatabaseConnectionManager>>) -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            conn_manager,
        }
    }
    
    // 添加任务到流水线
    pub async fn add_task(&self, task: MigrationTask) -> String {
        let id = task.id.clone();
        let mut tasks = self.tasks.write().await;
        tasks.insert(id.clone(), task);
        id
    }
    
    // 获取任务
    pub async fn get_task(&self, id: &str) -> Option<MigrationTask> {
        let tasks = self.tasks.read().await;
        tasks.get(id).cloned()
    }
    
    // 获取所有任务
    pub async fn get_all_tasks(&self) -> Vec<MigrationTask> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }
    
    // 更新任务
    pub async fn update_task(&self, task: MigrationTask) -> Result<(), String> {
        let mut tasks = self.tasks.write().await;
        let id = task.id.clone();
        
        if !tasks.contains_key(&id) {
            return Err(format!("Task not found: {}", id));
        }
        
        tasks.insert(id, task);
        Ok(())
    }
    
    // 开始任务
    pub async fn start_task(&self, id: &str) -> Result<(), String> {
        let mut task = self.get_task(id).await
            .ok_or_else(|| format!("Task not found: {}", id))?;
        
        // 开始任务
        task.start(self.conn_manager.clone()).await?;
        
        // 更新任务状态
        self.update_task(task).await?;
        
        // 在实际应用中，这里会启动一个单独的异步任务来执行迁移
        let pipeline = self.clone();
        let id = id.to_string();
        tokio::spawn(async move {
            if let Err(err) = pipeline.execute_task(&id).await {
                // 记录错误，实际应用中应该有更好的错误处理
                eprintln!("Error executing task {}: {}", id, err);
            }
        });
        
        Ok(())
    }
    
    // 暂停任务
    pub async fn pause_task(&self, id: &str) -> Result<(), String> {
        let mut task = self.get_task(id).await
            .ok_or_else(|| format!("Task not found: {}", id))?;
        
        task.pause()?;
        self.update_task(task).await?;
        
        Ok(())
    }
    
    // 取消任务
    pub async fn cancel_task(&self, id: &str) -> Result<(), String> {
        let mut task = self.get_task(id).await
            .ok_or_else(|| format!("Task not found: {}", id))?;
        
        task.cancel()?;
        self.update_task(task).await?;
        
        Ok(())
    }
    
    // 重试任务
    pub async fn retry_task(&self, id: &str) -> Result<(), String> {
        let mut task = self.get_task(id).await
            .ok_or_else(|| format!("Task not found: {}", id))?;
        
        // 重置任务状态
        task.status = TaskStatus::Created;
        task.error = None;
        task.progress = TaskProgress::new();
        task.started_at = None;
        task.completed_at = None;
        
        self.update_task(task).await?;
        
        // 开始任务
        self.start_task(id).await?;
        
        Ok(())
    }
    
    // 执行任务（内部方法）
    async fn execute_task(&self, id: &str) -> Result<(), String> {
        let mut task = self.get_task(id).await
            .ok_or_else(|| format!("Task not found: {}", id))?;
        
        // 确保任务状态是Running
        if task.status != TaskStatus::Running {
            return Err(format!("Task is not running: {}", id));
        }
        
        // 执行迁移策略
        match task.strategy.execute(&task.source_db_id, &task.target_db_id, self.conn_manager.clone()).await {
            Ok(_) => {
                // 任务成功完成
                task.complete()?;
            },
            Err(err) => {
                // 任务失败
                task.fail(err)?;
            },
        }
        
        // 更新任务状态
        self.update_task(task).await?;
        
        Ok(())
    }
}

// 实现克隆特性
impl Clone for MigrationPipeline {
    fn clone(&self) -> Self {
        Self {
            tasks: self.tasks.clone(),
            conn_manager: self.conn_manager.clone(),
        }
    }
}