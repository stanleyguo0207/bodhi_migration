// Bodhi Database Migration Application

// 导入必要的模块
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::State;

// 导入自定义模块
mod database;
mod migration;
mod models;

// 导入具体类型
use database::{connection::DatabaseConnectionEnum, sqlite::SQLiteConnection, sqlite::SQLiteConfig, mysql::MySQLConnection, mysql::MySQLConfig, redis::RedisConnection, redis::RedisConfig};
use migration::{MigrationTask, MigrationStrategyEnum, strategy::{FullMigrationStrategy, IncrementalMigrationStrategy, CustomSQLMigrationStrategy}};
use models::AppState;

// 初始化应用状态
#[derive(Clone, serde::Serialize)]
struct InitResult {
    success: bool,
    message: String,
}

#[tauri::command]
async fn init_app(_: State<'_, Arc<RwLock<AppState>>>) -> Result<InitResult, String> {
    // 应用初始化逻辑
    // 这里可以加载保存的配置、初始化数据库等
    
    Ok(InitResult {
        success: true,
        message: "Application initialized successfully".to_string(),
    })
}

// 数据库连接管理命令

// 添加SQLite连接
#[tauri::command]
async fn add_sqlite_connection(
    db_path: String,
    read_only: bool,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let config = SQLiteConfig {
        db_path,
        read_only,
    };
    
    let connection = SQLiteConnection::new(config)
        .map_err(|e| format!("Failed to create SQLite connection: {}", e))?;
    
    let conn_manager = state.read().await.conn_manager.clone();
    let id = conn_manager.write().await.add_connection(DatabaseConnectionEnum::SQLite(connection)).await;
    
    Ok(id)
}

// 添加MySQL连接
#[tauri::command]
async fn add_mysql_connection(
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let config = MySQLConfig {
        host,
        port,
        username,
        password,
        database,
    };
    
    let connection = MySQLConnection::new(config)
        .map_err(|e| format!("Failed to create MySQL connection: {}", e))?;
    
    let conn_manager = state.read().await.conn_manager.clone();
    let id = conn_manager.write().await.add_connection(DatabaseConnectionEnum::MySQL(connection)).await;
    
    Ok(id)
}

// 添加Redis连接
#[tauri::command]
async fn add_redis_connection(
    url: String,
    db: Option<usize>,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let config = RedisConfig {
        url,
        db,
    };
    
    let connection = RedisConnection::new(config)
        .map_err(|e| format!("Failed to create Redis connection: {}", e))?;
    
    let conn_manager = state.read().await.conn_manager.clone();
    let id = conn_manager.write().await.add_connection(DatabaseConnectionEnum::Redis(connection)).await;
    
    Ok(id)
}

// 测试数据库连接
#[tauri::command]
async fn test_database_connection(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let conn_manager = state.read().await.conn_manager.clone();
    conn_manager.write().await.test_connection(&id).await?;
    Ok(true)
}

// 移除数据库连接
#[tauri::command]
async fn remove_database_connection(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let conn_manager = state.read().await.conn_manager.clone();
    let result = conn_manager.write().await.remove_connection(&id).await;
    Ok(result.is_some())
}

// 迁移任务管理命令

// 创建迁移任务
#[tauri::command]
async fn create_migration_task(
    name: String,
    description: String,
    source_db_id: String,
    target_db_id: String,
    strategy_type: String,
    strategy_params: Option<serde_json::Value>,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    // 根据策略类型创建相应的策略
    let strategy: MigrationStrategyEnum = match strategy_type.as_str() {
        "full" => MigrationStrategyEnum::Full(FullMigrationStrategy::new()),
        "incremental" => {
            let last_migration_id = strategy_params.and_then(|p| {
                p.get("last_migration_id").and_then(|v| v.as_str()).map(|s| s.to_string())
            });
            MigrationStrategyEnum::Incremental(IncrementalMigrationStrategy::new(last_migration_id))
        },
        "custom_sql" => {
            let source_sql = strategy_params.clone()
                .and_then(|p| p.get("source_sql").and_then(|v| v.as_str().map(|s| s.to_string())))
                .ok_or_else(|| "Missing source_sql parameter".to_string())?;
            let target_sql_template = strategy_params
                .and_then(|p| p.get("target_sql_template").and_then(|v| v.as_str().map(|s| s.to_string())))
                .ok_or_else(|| "Missing target_sql_template parameter".to_string())?;
            MigrationStrategyEnum::CustomSQL(CustomSQLMigrationStrategy::new(
                name.clone(),
                source_sql.to_string(),
                target_sql_template.to_string()
            ))
        },
        _ => return Err(format!("Unknown strategy type: {}", strategy_type)),
    };
    
    // 创建任务
    let task = MigrationTask::new(
        name,
        description,
        source_db_id,
        target_db_id,
        strategy
    );
    
    let task_id = task.id.clone();
    
    // 添加任务到流水线
    let pipeline = state.read().await.migration_pipeline.clone();
    pipeline.write().await.add_task(task).await;
    
    Ok(task_id)
}

// 开始迁移任务
#[tauri::command]
async fn start_migration_task(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let pipeline = state.read().await.migration_pipeline.clone();
    pipeline.write().await.start_task(&id).await?;
    Ok(true)
}

// 暂停迁移任务
#[tauri::command]
async fn pause_migration_task(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let pipeline = state.read().await.migration_pipeline.clone();
    pipeline.write().await.pause_task(&id).await?;
    Ok(true)
}

// 取消迁移任务
#[tauri::command]
async fn cancel_migration_task(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let pipeline = state.read().await.migration_pipeline.clone();
    pipeline.write().await.cancel_task(&id).await?;
    Ok(true)
}

// 重试迁移任务
#[tauri::command]
async fn retry_migration_task(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let pipeline = state.read().await.migration_pipeline.clone();
    pipeline.write().await.retry_task(&id).await?;
    Ok(true)
}

// 获取迁移任务
#[tauri::command]
async fn get_migration_task(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Option<serde_json::Value>, String> {
    let pipeline = state.read().await.migration_pipeline.clone();
    let task = pipeline.write().await.get_task(&id).await;
    
    // 将任务转换为JSON
    match task {
        Some(task) => {
            let task_json = serde_json::to_value(task)
                .map_err(|e| format!("Failed to serialize task: {}", e))?;
            Ok(Some(task_json))
        },
        None => Ok(None),
    }
}

// 获取所有迁移任务
#[tauri::command]
async fn get_all_migration_tasks(
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<serde_json::Value>, String> {
    let pipeline = state.read().await.migration_pipeline.clone();
    let tasks = pipeline.write().await.get_all_tasks().await;
    
    // 将任务列表转换为JSON
    let tasks_json: Vec<serde_json::Value> = tasks
        .into_iter()
        .map(|task| serde_json::to_value(task))
        .collect::<Result<_, _>>()
        .map_err(|e| format!("Failed to serialize tasks: {}", e))?;
    
    Ok(tasks_json)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建应用状态
    let app_state = Arc::new(RwLock::new(AppState::new()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 注册应用状态
        .manage(app_state)
        // 注册命令
        .invoke_handler(tauri::generate_handler![
            init_app,
            // 数据库连接管理命令
            add_sqlite_connection,
            add_mysql_connection,
            add_redis_connection,
            test_database_connection,
            remove_database_connection,
            // 迁移任务管理命令
            create_migration_task,
            start_migration_task,
            pause_migration_task,
            cancel_migration_task,
            retry_migration_task,
            get_migration_task,
            get_all_migration_tasks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
