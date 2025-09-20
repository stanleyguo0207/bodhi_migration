// 迁移任务模块

use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tokio::time::Instant;
use uuid::Uuid;
use serde_with::{serde_as, TimestampMilliSeconds};
use crate::database::connection::DatabaseConnectionManager;
use crate::migration::{MigrationStrategyEnum};

// 任务状态枚举
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TaskStatus {
    Created,
    Running,
    Paused,
    Completed,
    Failed,
    Canceled,
}

// 任务进度信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskProgress {
    pub completed_items: u64,
    pub total_items: Option<u64>,
    pub current_operation: String,
    #[serde(skip)]
    pub start_time: Instant,
    #[serde(skip)]
    pub last_update_time: Instant,
}

// 为TaskProgress实现自定义的反序列化
impl<'de> serde::Deserialize<'de> for TaskProgress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 定义一个没有Instant字段的临时结构体用于反序列化
        #[derive(Debug, serde::Deserialize)]
        struct TaskProgressTemp {
            completed_items: u64,
            total_items: Option<u64>,
            current_operation: String,
        }
        
        let temp = TaskProgressTemp::deserialize(deserializer)?;
        
        // 创建一个新的Instant用于start_time和last_update_time
        let now = Instant::now();
        
        Ok(Self {
            completed_items: temp.completed_items,
            total_items: temp.total_items,
            current_operation: temp.current_operation,
            start_time: now,
            last_update_time: now,
        })
    }
}

impl TaskProgress {
    // 创建新的进度信息
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            completed_items: 0,
            total_items: None,
            current_operation: "Initializing".to_string(),
            start_time: now,
            last_update_time: now,
        }
    }
    
    // 更新进度
    pub fn update(&mut self, completed_items: u64, current_operation: String) {
        self.completed_items = completed_items;
        self.current_operation = current_operation;
        self.last_update_time = Instant::now();
    }
    
    // 设置总项目数
    pub fn set_total_items(&mut self, total_items: u64) {
        self.total_items = Some(total_items);
    }
    
    // 获取进度百分比
    pub fn get_percentage(&self) -> Option<f64> {
        match self.total_items {
            Some(total) if total > 0 => {
                Some((self.completed_items as f64 / total as f64) * 100.0)
            },
            _ => None,
        }
    }
    
    // 获取已运行时间
    pub fn get_duration(&self) -> Duration {
        self.last_update_time.saturating_duration_since(self.start_time)
    }
}

// 迁移任务
#[serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MigrationTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_db_id: String,
    pub target_db_id: String,
    pub strategy: MigrationStrategyEnum,
    pub status: TaskStatus,
    pub progress: TaskProgress,
    pub error: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: SystemTime,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub updated_at: SystemTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub started_at: Option<SystemTime>,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub completed_at: Option<SystemTime>,
}

impl MigrationTask {
    // 创建新的迁移任务
    pub fn new(
        name: String,
        description: String,
        source_db_id: String,
        target_db_id: String,
        strategy: MigrationStrategyEnum,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            source_db_id,
            target_db_id,
            strategy,
            status: TaskStatus::Created,
            progress: TaskProgress::new(),
            error: None,
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
        }
    }
    
    // 开始任务
    pub async fn start(&mut self, _: Arc<RwLock<DatabaseConnectionManager>>) -> Result<(), String> {
        if self.status != TaskStatus::Created && self.status != TaskStatus::Paused && self.status != TaskStatus::Failed {
            return Err(format!("Cannot start task in {} status", self.status_to_string()));
        }
        
        self.status = TaskStatus::Running;
        self.started_at.get_or_insert(SystemTime::now());
        self.updated_at = SystemTime::now();
        self.progress.update(0, "Starting migration".to_string());
        
        // 在实际应用中，这里会启动一个异步任务来执行迁移
        // 为了简化示例，我们仅返回成功
        Ok(())
    }
    
    // 暂停任务
    pub fn pause(&mut self) -> Result<(), String> {
        if self.status != TaskStatus::Running {
            return Err(format!("Cannot pause task in {} status", self.status_to_string()));
        }
        
        self.status = TaskStatus::Paused;
        self.updated_at = SystemTime::now();
        self.progress.update(self.progress.completed_items, "Migration paused".to_string());
        
        Ok(())
    }
    
    // 取消任务
    pub fn cancel(&mut self) -> Result<(), String> {
        if self.status == TaskStatus::Completed || self.status == TaskStatus::Canceled {
            return Err(format!("Cannot cancel task in {} status", self.status_to_string()));
        }
        
        self.status = TaskStatus::Canceled;
        self.updated_at = SystemTime::now();
        self.progress.update(self.progress.completed_items, "Migration canceled".to_string());
        
        Ok(())
    }
    
    // 完成任务
    pub fn complete(&mut self) -> Result<(), String> {
        if self.status != TaskStatus::Running {
            return Err(format!("Cannot complete task in {} status", self.status_to_string()));
        }
        
        self.status = TaskStatus::Completed;
        self.updated_at = SystemTime::now();
        self.completed_at = Some(SystemTime::now());
        self.progress.update(self.progress.completed_items, "Migration completed".to_string());
        
        Ok(())
    }
    
    // 设置任务失败
    pub fn fail(&mut self, error: String) -> Result<(), String> {
        if self.status == TaskStatus::Completed || self.status == TaskStatus::Canceled {
            return Err(format!("Cannot fail task in {} status", self.status_to_string()));
        }
        
        self.status = TaskStatus::Failed;
        self.error = Some(error);
        self.updated_at = SystemTime::now();
        self.progress.update(self.progress.completed_items, "Migration failed".to_string());
        
        Ok(())
    }
    
    // 获取状态的字符串表示
    pub fn status_to_string(&self) -> String {
        match self.status {
            TaskStatus::Created => "Created",
            TaskStatus::Running => "Running",
            TaskStatus::Paused => "Paused",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
            TaskStatus::Canceled => "Canceled",
        }
        .to_string()
    }
}