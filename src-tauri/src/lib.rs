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
use database::{connection::DatabaseConnectionEnum, sqlite::{SQLiteConnection, SQLiteConfig, DatabaseConfig}, mysql::MySQLConnection, mysql::MySQLConfig, redis::RedisConnection, redis::RedisConfig};
use migration::{MigrationTask, MigrationStrategyEnum, strategy::{FullMigrationStrategy, IncrementalMigrationStrategy, CustomSQLMigrationStrategy}};
use models::AppState;
use tauri::utils::platform::current_exe;

// 初始化应用状态
#[derive(Clone, serde::Serialize)]
struct InitResult {
    success: bool,
    message: String,
}

// 初始化应用数据库
#[tauri::command]
async fn init_app(state: State<'_, Arc<RwLock<AppState>>>) -> Result<InitResult, String> {
    // 应用初始化逻辑
    // 获取应用程序目录
    let exe_path = current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;
    let app_dir = exe_path.parent()
        .ok_or_else(|| "Failed to get executable directory".to_string())?
        .join("bodhi_migration");
    
    // 创建目录（如果不存在）
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create app directory: {}", e))?;
    
    // 数据库文件路径
    let db_path = app_dir.join("bodhi_migration.db").to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())?
        .to_string();
    
    // 创建SQLite连接
    let config = SQLiteConfig {
        db_path: db_path.clone(),
        read_only: false,
    };
    
    let connection = SQLiteConnection::new(config)
        .map_err(|e| format!("Failed to create SQLite connection: {}", e))?;
    
    // 将SQLite连接添加到连接管理器
    let conn_manager = state.read().await.conn_manager.clone();
    let connection_id = conn_manager.write().await.add_connection(DatabaseConnectionEnum::SQLite(connection)).await;
    
    // 保存连接ID到应用状态
    state.write().await.sqlite_config_connection_id = Some(connection_id);
    
    Ok(InitResult {
        success: true,
        message: "Application initialized successfully".to_string(),
    })
}

// 数据库连接管理命令

// 保存数据库配置到SQLite
#[tauri::command]
async fn save_database_config_to_db(
    config: serde_json::Value,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    // 确保应用有SQLite连接
    let app_state = state.read().await;
    let connection_id = app_state.sqlite_config_connection_id.clone()
        .ok_or_else(|| "SQLite config connection not initialized".to_string())?;
    
    let conn_manager = app_state.conn_manager.clone();
    
    // 通过保存的ID获取SQLite连接
    let sqlite_conn_enum = conn_manager.read().await.get_connection(&connection_id).await
        .ok_or_else(|| "No SQLite connection found".to_string())?;
    
    // 转换为SQLiteConnection
    let sqlite_conn = if let DatabaseConnectionEnum::SQLite(conn) = sqlite_conn_enum {
        conn
    } else {
        return Err("Failed to get SQLite connection".to_string());
    };
    
    // 解析配置
    let id = config.get("id").and_then(|v| v.as_str()).ok_or_else(|| "Missing id".to_string())?;
    let name = config.get("name").and_then(|v| v.as_str()).ok_or_else(|| "Missing name".to_string())?;
    let r#type = config.get("type").and_then(|v| v.as_str()).ok_or_else(|| "Missing type".to_string())?;
    let host = config.get("host").and_then(|v| v.as_str()).map(|s| s.to_string());
    let port = config.get("port").and_then(|v| v.as_i64());
    let username = config.get("username").and_then(|v| v.as_str()).map(|s| s.to_string());
    let password = config.get("password").and_then(|v| v.as_str()).map(|s| s.to_string());
    let database = config.get("database").and_then(|v| v.as_str()).map(|s| s.to_string());
    let ssl = config.get("ssl").and_then(|v| v.as_bool()).unwrap_or(false);
    let extra = config.get("extra").and_then(|v| v.as_str()).map(|s| s.to_string());
    let created_at = config.get("createdAt").and_then(|v| v.as_str()).ok_or_else(|| "Missing createdAt".to_string())?;
    let updated_at = config.get("updatedAt").and_then(|v| v.as_str()).ok_or_else(|| "Missing updatedAt".to_string())?;
    
    // 记录时间戳信息用于调试
    println!("保存数据库配置 - ID: {}, 创建时间: {}, 更新时间: {}", id, created_at, updated_at);
    
    // 创建DatabaseConfig结构体
    let db_config = DatabaseConfig {
        id: id.to_string(),
        name: name.to_string(),
        r#type: r#type.to_string(),
        host,
        port,
        username,
        password,
        database,
        ssl,
        extra,
        created_at: created_at.to_string(),
        updated_at: updated_at.to_string(),
    };
    
    // 保存配置
    sqlite_conn.save_database_config(&db_config)?;
    
    Ok(id.to_string())
}

// 从SQLite获取所有数据库配置
#[tauri::command]
async fn get_all_database_configs_from_db(
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<serde_json::Value, String> {
    // 确保应用有SQLite连接
    let app_state = state.read().await;
    let connection_id = app_state.sqlite_config_connection_id.clone()
        .ok_or_else(|| "SQLite config connection not initialized".to_string())?;
    
    let conn_manager = app_state.conn_manager.clone();
    
    // 通过保存的ID获取SQLite连接
    let sqlite_conn_enum = conn_manager.read().await.get_connection(&connection_id).await
        .ok_or_else(|| "No SQLite connection found".to_string())?;
    
    // 转换为SQLiteConnection
    let sqlite_conn = if let DatabaseConnectionEnum::SQLite(conn) = sqlite_conn_enum {
        conn
    } else {
        return Err("Failed to get SQLite connection".to_string());
    };
    
    // 获取所有配置
    let configs = sqlite_conn.get_all_database_configs()?;
    
    // 转换为JSON
    let configs_json = serde_json::to_value(configs)
        .map_err(|e| format!("Failed to serialize configs: {}", e))?;
    
    Ok(configs_json)
}

// 从SQLite删除数据库配置
#[tauri::command]
async fn delete_database_config_from_db(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    // 确保应用有SQLite连接
    let app_state = state.read().await;
    let connection_id = app_state.sqlite_config_connection_id.clone()
        .ok_or_else(|| "SQLite config connection not initialized".to_string())?;
    
    let conn_manager = app_state.conn_manager.clone();
    
    // 通过保存的ID获取SQLite连接
    let sqlite_conn_enum = conn_manager.read().await.get_connection(&connection_id).await
        .ok_or_else(|| "No SQLite connection found".to_string())?;
    
    // 转换为SQLiteConnection
    let sqlite_conn = if let DatabaseConnectionEnum::SQLite(conn) = sqlite_conn_enum {
        conn
    } else {
        return Err("Failed to get SQLite connection".to_string());
    };
    
    // 删除配置
    sqlite_conn.delete_database_config(&id)?;
    
    Ok(true)
}

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
            // 数据库配置持久化命令
            save_database_config_to_db,
            get_all_database_configs_from_db,
            delete_database_config_from_db,
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
