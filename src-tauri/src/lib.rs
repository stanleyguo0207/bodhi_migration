// Bodhi Database Migration Application - 使用 sqlx 统一数据库操作

// 导入必要的模块
use chrono::Utc;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;
use sqlx::Row; // 导入Row trait用于try_get方法

// 导入自定义模块
mod database;
mod migration;
mod models;


// 导入 sqlx 数据库类型
use database::{DatabaseConfig, DatabaseType, UnifiedConnectionManager};
use migration::{
    strategy::{CustomSQLMigrationStrategy, FullMigrationStrategy, IncrementalMigrationStrategy},
    MigrationStrategyEnum, MigrationTask,
};
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
    let app_dir = exe_path
        .parent()
        .ok_or_else(|| "Failed to get executable directory".to_string())?
        .join("bodhi_migration");

    // 创建目录（如果不存在）
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create app directory: {}", e))?;

    // 数据库文件路径
    let db_path = app_dir
        .join("bodhi_migration.db")
        .to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())?
        .to_string();

    // 检查数据库文件是否存在，如果不存在则创建空文件
    if !std::path::Path::new(&db_path).exists() {
        std::fs::File::create(&db_path)
            .map_err(|e| format!("Failed to create SQLite database file: {}", e))?;
        println!("SQLite database file created at: {}", db_path);
    }

    // 创建SQLite配置
    let config = DatabaseConfig {
        id: "config_db".to_string(),
        name: "Application Configuration Database".to_string(),
        r#type: "sqlite".to_string(),
        host: None,
        port: None,
        username: None,
        password: None,
        database: Some(db_path.clone()),
        ssl: false,
        extra: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    // 获取统一连接管理器
    let conn_manager = state.read().await.conn_manager.clone();

    // 添加SQLite连接
    conn_manager.add_connection(config).await?;

    // 保存连接ID到应用状态
    state.write().await.sqlite_config_connection_id = Some("config_db".to_string());

    Ok(InitResult {
        success: true,
        message: "Application initialized successfully with sqlx".to_string(),
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
    let connection_id = app_state
        .sqlite_config_connection_id
        .clone()
        .ok_or_else(|| "SQLite config connection not initialized".to_string())?;

    let conn_manager = app_state.conn_manager.clone();
    drop(app_state); // 释放读取锁

    // 获取SQLite连接
    let connection = conn_manager
        .get_sqlx_connection(&connection_id)
        .await
        .ok_or_else(|| format!("No SQLite connection found for ID: {}", connection_id))?;

    // 解析配置
    let id = config
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing id".to_string())?;
    let name = config
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing name".to_string())?;
    let r#type = config
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing type".to_string())?;
    let host = config
        .get("host")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let port = config.get("port").and_then(|v| v.as_i64());
    let username = config
        .get("username")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let password = config
        .get("password")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let database = config
        .get("database")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let ssl = config.get("ssl").and_then(|v| v.as_bool()).unwrap_or(false);
    let extra = config
        .get("extra")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let created_at = config
        .get("createdAt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing createdAt".to_string())?;
    let updated_at = config
        .get("updatedAt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing updatedAt".to_string())?;

    // 记录时间戳信息用于调试
    println!(
        "保存数据库配置 - ID: {}, 创建时间: {}, 更新时间: {}",
        id, created_at, updated_at
    );

    // 创建数据库配置表（如果不存在）
    let create_table_sql = r#"
        CREATE TABLE IF NOT EXISTS database_configs (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            host TEXT,
            port INTEGER,
            username TEXT,
            password TEXT,
            database TEXT,
            ssl BOOLEAN DEFAULT FALSE,
            extra TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
    "#;

    connection.execute_raw(create_table_sql).await?;

    // 插入或更新配置
    let insert_sql = r#"
        INSERT OR REPLACE INTO database_configs 
        (id, name, type, host, port, username, password, database, ssl, extra, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

    let port_str = port
        .map(|p| p.to_string())
        .unwrap_or_else(|| "".to_string());
    let ssl_str = if ssl {
        "true".to_string()
    } else {
        "false".to_string()
    };

    let params = vec![
        id,
        name,
        r#type,
        host.as_deref().unwrap_or(""),
        port_str.as_str(),
        username.as_deref().unwrap_or(""),
        password.as_deref().unwrap_or(""),
        database.as_deref().unwrap_or(""),
        ssl_str.as_str(),
        extra.as_deref().unwrap_or(""),
        created_at,
        updated_at,
    ];

    let rows_affected = connection
        .execute_with_params(
            insert_sql,
            params
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )
        .await?;

    println!("保存数据库配置影响行数: {}", rows_affected);

    Ok(id.to_string())
}

// 从SQLite获取所有数据库配置
#[tauri::command]
async fn get_all_database_configs_from_db(
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<serde_json::Value, String> {
    // 确保应用有SQLite连接
    let app_state: tokio::sync::RwLockReadGuard<'_, AppState> = state.read().await;
    let connection_id = app_state
        .sqlite_config_connection_id
        .clone()
        .ok_or_else(|| "SQLite config connection not initialized".to_string())?;

    println!("获取所有数据库配置 - 连接ID: {}", connection_id);

    let conn_manager = app_state.conn_manager.clone();
    drop(app_state); // 释放读取锁

    // 获取SQLite连接
    let connection = conn_manager
        .get_sqlx_connection(&connection_id)
        .await
        .ok_or_else(|| "No SQLite connection found".to_string())?;

    // 查询所有配置
    let mut configs = Vec::new();
    
    // 首先检查表是否存在
    let check_table_sql = "SELECT name FROM sqlite_master WHERE type='table' AND name='database_configs'";
    let table_exists = connection.execute(check_table_sql).await? > 0;
    
    if table_exists {
        // 查询所有配置 - 使用SQLite专用查询方法
        let select_sql = r#"
            SELECT id, name, type, host, port, username, password, database, ssl, extra, created_at, updated_at
            FROM database_configs
            ORDER BY created_at DESC
        "#;
        
        // 使用SQLite专用查询方法
        match connection.fetch_all_sqlite(select_sql).await {
            Ok(rows) => {
                // 手动解析查询结果并转换为前端期望的格式
                for row in rows {
                    let config = serde_json::json!({
                        "id": row.try_get::<String, _>("id").unwrap_or_default(),
                        "name": row.try_get::<String, _>("name").unwrap_or_default(),
                        "type": row.try_get::<String, _>("type").unwrap_or_default(),
                        "host": row.try_get::<Option<String>, _>("host").ok().flatten(),
                        "port": row.try_get::<Option<i64>, _>("port").ok().flatten(),
                        "username": row.try_get::<Option<String>, _>("username").ok().flatten(),
                        "password": row.try_get::<Option<String>, _>("password").ok().flatten(),
                        "database": row.try_get::<Option<String>, _>("database").ok().flatten(),
                        "ssl": row.try_get::<bool, _>("ssl").unwrap_or(false),
                        "extra": row.try_get::<Option<String>, _>("extra").ok().flatten(),
                        "createdAt": row.try_get::<String, _>("created_at").unwrap_or_default(),
                        "updatedAt": row.try_get::<String, _>("updated_at").unwrap_or_default(),
                    });
                    configs.push(config);
                }
                println!("成功加载 {} 个数据库配置", configs.len());
            },
            Err(e) => {
                println!("查询数据库配置失败: {}", e);
                // 如果查询失败，返回空数组而不是错误
            }
        }
    } else {
        println!("数据库配置表不存在，可能是首次运行");
    }

    // 直接返回JSON数组
    Ok(serde_json::json!(configs))
}

// 从SQLite删除数据库配置
#[tauri::command]
async fn delete_database_config_from_db(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    // 确保应用有SQLite连接
    let app_state = state.read().await;
    let connection_id = app_state
        .sqlite_config_connection_id
        .clone()
        .ok_or_else(|| "SQLite config connection not initialized".to_string())?;

    let conn_manager = app_state.conn_manager.clone();

    // 获取SQLite连接
    let connection = conn_manager
        .get_sqlx_connection(&connection_id)
        .await
        .ok_or_else(|| "No SQLite connection found".to_string())?;

    // 删除配置
    let delete_sql = "DELETE FROM database_configs WHERE id = ?";
    let params = vec![id.clone()];
    let rows_affected = connection.execute_with_params(delete_sql, params).await?;

    Ok(rows_affected > 0)
}

// 添加SQLite连接
#[tauri::command]
async fn add_sqlite_connection(
    db_path: String,
    _read_only: bool,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let connection_id = format!("sqlite_{}", Utc::now().timestamp_millis());

    let config = DatabaseConfig {
        id: connection_id.clone(),
        name: format!("SQLite Database {}", db_path),
        r#type: "sqlite".to_string(),
        host: None,
        port: None,
        username: None,
        password: None,
        database: Some(db_path),
        ssl: false,
        extra: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    let conn_manager = state.read().await.conn_manager.clone();
    conn_manager.add_connection(config).await?;

    Ok(connection_id)
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
    let connection_id = format!("mysql_{}", Utc::now().timestamp_millis());

    let config = DatabaseConfig {
        id: connection_id.clone(),
        name: format!("MySQL Database {}:{}", host, port),
        r#type: "mysql".to_string(),
        host: Some(host),
        port: Some(port as i64),
        username: Some(username),
        password: Some(password),
        database: Some(database),
        ssl: false,
        extra: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    let conn_manager = state.read().await.conn_manager.clone();
    conn_manager.add_connection(config).await?;

    Ok(connection_id)
}

// 添加Redis连接
#[tauri::command]
async fn add_redis_connection(
    url: String,
    db: Option<i64>,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let connection_id = format!("redis_{}", Utc::now().timestamp_millis());

    // 验证Redis URL格式
    if !url.starts_with("redis://") {
        return Err(format!("Invalid Redis URL format: {}", url));
    }

    let config = DatabaseConfig {
        id: connection_id.clone(),
        name: format!(
            "Redis Database {}",
            url.replace("redis://", "").replace(":", "****")
        ),
        r#type: "redis".to_string(),
        host: Some(url),
        port: None,
        username: None,
        password: None,
        database: db.map(|d| d.to_string()),
        ssl: false,
        extra: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    let conn_manager = state.read().await.conn_manager.clone();
    conn_manager.add_connection(config).await?;

    Ok(connection_id)
}

// 测试数据库连接
#[tauri::command]
async fn test_database_connection(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let conn_manager = state.read().await.conn_manager.clone();
    conn_manager.test_connection(&id).await?;
    Ok(true)
}

// 移除数据库连接
#[tauri::command]
async fn remove_database_connection(
    id: String,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let conn_manager = state.read().await.conn_manager.clone();
    conn_manager.remove_connection(&id).await?;
    Ok(true)
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
                p.get("last_migration_id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            });
            MigrationStrategyEnum::Incremental(IncrementalMigrationStrategy::new(last_migration_id))
        }
        "custom_sql" => {
            let source_sql = strategy_params
                .clone()
                .and_then(|p| {
                    p.get("source_sql")
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                })
                .ok_or_else(|| "Missing source_sql parameter".to_string())?;
            let target_sql_template = strategy_params
                .and_then(|p| {
                    p.get("target_sql_template")
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                })
                .ok_or_else(|| "Missing target_sql_template parameter".to_string())?;
            MigrationStrategyEnum::CustomSQL(CustomSQLMigrationStrategy::new(
                name.clone(),
                source_sql.to_string(),
                target_sql_template.to_string(),
            ))
        }
        _ => return Err(format!("Unknown strategy type: {}", strategy_type)),
    };

    // 创建任务
    let task = MigrationTask::new(name, description, source_db_id, target_db_id, strategy);

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
        }
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
            get_all_migration_tasks,

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
