import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import  {
  DatabaseType,
  type DatabaseConfig,
  type PipelineTask,
  type MigrationStrategy,
} from "../types/database.types";

// 数据库配置存储
export const databases = writable<DatabaseConfig[]>([]);

// 迁移策略存储
export const strategies = writable<MigrationStrategy[]>([]);

// 流水线任务存储
export const pipelineTasks = writable<PipelineTask[]>([]);

// 当前选中的数据库ID
export const selectedDatabaseId = writable<string | null>(null);

// 当前选中的流水线任务ID
export const selectedTaskId = writable<string | null>(null);

// 应用加载状态
export const appLoading = writable(true);

// 保存数据库配置到存储
export async function saveDatabaseConfig(
  config: DatabaseConfig
): Promise<void> {
  try {
    let connectionId: string;
    
    // 判断是新建还是编辑现有配置
    const isNewConfig = !config.id || config.id.startsWith('db_');
    
    if (isNewConfig) {
      // 新建配置：创建新的数据库连接
      if (config.type === DatabaseType.Redis) {
        // 对于Redis，我们使用url格式
        let redisUrl: string;
        
        // 验证主机地址，确保不包含非法字符
        const host = config.host || 'localhost';
        const port = config.port || 6379;
        
        // 验证主机名是否包含非法字符
        if (host.includes(' ') || host.includes('\t') || host.includes('\n')) {
          throw new Error(`Invalid Redis host: contains whitespace characters`);
        }
        
        // 构建Redis URL，确保主机地址有效
        if (config.username && config.password) {
          // URL编码用户名和密码，避免特殊字符问题
          const encodedUsername = encodeURIComponent(config.username);
          const encodedPassword = encodeURIComponent(config.password);
          redisUrl = `redis://${encodedUsername}:${encodedPassword}@${host}:${port}`;
        } else {
          redisUrl = `redis://${host}:${port}`;
        }
        
        console.log("构建的Redis URL:", redisUrl.replace(/:[^:@]+@/, ':****@')); // 日志中隐藏密码
        console.log("原始配置:", { host: config.host, port: config.port, username: config.username, database: config.database });
        
        connectionId = await invoke("add_redis_connection", {
          url: redisUrl,
          db: config.database ? parseInt(config.database) : 0,
        });
      } else if (config.type === DatabaseType.MySQL) {
        connectionId = await invoke("add_mysql_connection", {
          host: config.host,
          port: config.port,
          username: config.username,
          password: config.password || "",
          database: config.database || "",
        });
      } else if (config.type === DatabaseType.PostgreSQL) {
        // 注意：PostgreSQL在当前后端实现中可能不被支持
        throw new Error("PostgreSQL is not yet supported in the backend");
      } else {
        throw new Error(`Unsupported database type: ${config.type}`);
      }
      console.log("New database connection established, connection ID:", connectionId);
    } else {
      // 编辑现有配置：使用现有的连接ID
      connectionId = config.id;
      console.log("Updating existing database configuration, connection ID:", connectionId);
    }

    // 创建配置对象，使用正确的ID
    const configToSave = {
      ...config,
      id: connectionId,
    };

    // 将配置保存到SQLite数据库
    const savedId = await invoke<string>("save_database_config_to_db", {
      config: configToSave
    });

    // 验证保存的ID与我们的配置ID一致
    if (savedId !== configToSave.id) {
      console.warn("Backend returned different ID than expected:", savedId, "vs", configToSave.id);
    }

    // 更新本地存储 - 使用原始配置对象（保持所有字段包括时间戳）
    databases.update((prev) => {
      // 检查是否已有相同ID的配置
      const index = prev.findIndex((db) => db.id === configToSave.id);
      if (index !== -1) {
        // 更新现有配置
        prev[index] = configToSave;
      } else {
        // 添加新配置
        prev.push(configToSave);
      }
      return [...prev];
    });
  } catch (error) {
    console.error("Failed to save database config:", error);
    throw error;
  }
}

// 从存储加载所有数据库配置
export async function loadDatabaseConfigs(): Promise<void> {
  try {
    // 通过Tauri调用后端API加载数据库配置（从SQLite）
    const configs = await invoke("get_all_database_configs_from_db");
    databases.set(configs as DatabaseConfig[]);
  } catch (error) {
    console.error("Failed to load database configs:", error);
    throw error;
  }
}

// 从store中获取特定ID的数据库配置
export async function getDatabaseConfigById(id: string): Promise<DatabaseConfig | undefined> {
  let config: DatabaseConfig | undefined;
  
  // 检查store中是否已有数据
  databases.subscribe((currentDatabases) => {
    if (currentDatabases.length > 0) {
      config = currentDatabases.find((db) => db.id === id);
    }
  })();
  
  // 如果store中没有数据或找不到指定ID的配置，则加载所有配置
  if (!config) {
    await loadDatabaseConfigs();
    
    // 再次尝试从store中查找
    databases.subscribe((currentDatabases) => {
      config = currentDatabases.find((db) => db.id === id);
    })();
  }
  
  return config;
}

// 创建新的流水线任务
export async function createPipelineTask(
  task: Omit<
    PipelineTask,
    "id" | "createdAt" | "updatedAt" | "progress" | "status"
  >
): Promise<PipelineTask> {
  try {
    // 通过Tauri调用后端API创建流水线任务
    const newTask = await invoke("create_pipeline_task", { task });

    // 更新本地存储
    pipelineTasks.update((prev) => [...prev, newTask as PipelineTask]);

    return newTask as PipelineTask;
  } catch (error) {
    console.error("Failed to create pipeline task:", error);
    throw error;
  }
}

// 启动流水线任务
export async function startPipelineTask(taskId: string): Promise<void> {
  try {
    // 通过Tauri调用后端API启动流水线任务
    await invoke("start_pipeline_task", { taskId });
  } catch (error) {
    console.error("Failed to start pipeline task:", error);
    throw error;
  }
}

// 删除数据库配置
export async function removeDatabaseConfig(id: string): Promise<void> {
  try {
    // 移除数据库连接
    await invoke("remove_database_connection", { id });

    // 从SQLite数据库中删除配置
    await invoke("delete_database_config_from_db", { id });

    // 从本地存储中移除
    databases.update((prev) => prev.filter((db) => db.id !== id));
  } catch (error) {
    console.error("Failed to remove database config:", error);
    throw error;
  }
}
