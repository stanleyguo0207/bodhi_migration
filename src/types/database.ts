// 数据库类型定义
export enum DatabaseType {
  MySQL = "mysql",
  Redis = "redis",
  PostgreSQL = "postgresql",
}

// 数据库连接配置接口
export interface DatabaseConfig {
  id: string;
  name: string;
  type: DatabaseType;
  host?: string;
  port?: number;
  username?: string;
  password?: string;
  database?: string;
  ssl?: boolean;
  extra?: Record<string, string>;
  createdAt: string;
  updatedAt: string;
}

// 迁移策略接口
export interface MigrationStrategy {
  id: string;
  name: string;
  type: "full" | "incremental" | "delta";
  batchSize?: number;
  retryCount?: number;
  timeout?: number;
  filters?: Record<string, any>;
  transformers?: Array<{
    name: string;
    params: Record<string, any>;
  }>;
}

// 流水线任务接口
export interface PipelineTask {
  id: string;
  name: string;
  sourceDbId: string;
  targetDbId: string;
  strategyId: string;
  status: "pending" | "running" | "completed" | "failed" | "paused";
  progress: number;
  startTime?: string;
  endTime?: string;
  error?: string;
  logs?: Array<{
    timestamp: string;
    message: string;
    level: "info" | "warning" | "error";
  }>;
  createdAt: string;
  updatedAt: string;
}
