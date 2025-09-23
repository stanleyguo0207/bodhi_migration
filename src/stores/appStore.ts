import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type {
  DatabaseConfig,
  PipelineTask,
  MigrationStrategy,
} from "../types/database";
import { DatabaseType } from "../types/database";

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
    // 首先测试数据库连接是否有效
    let connectionId;
    if (config.type === DatabaseType.Redis) {
      // 对于Redis，我们使用url格式
      const redisUrl = `redis://${
        config.username ? `${config.username}:${config.password}@` : ""
      }${config.host}:${config.port}`;
      connectionId = await invoke("add_redis_connection", {
        url: redisUrl,
        db: config.database ? parseInt(config.database) : undefined,
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

    console.log("Database connection established, connection ID:", connectionId);

    // 创建一个新的配置对象，使用后端返回的ID
    const configWithBackendId = {
      ...config,
      id: connectionId as string, // 后端返回的ID
    };

    // 将配置保存到SQLite数据库
    await invoke("save_database_config_to_db", {
      config: configWithBackendId
    });

    // 更新本地存储
    databases.update((prev) => {
      // 检查是否已有相同后端ID的配置
      const index = prev.findIndex((db) => db.id === configWithBackendId.id);
      if (index !== -1) {
        prev[index] = configWithBackendId;
      } else {
        prev.push(configWithBackendId);
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
