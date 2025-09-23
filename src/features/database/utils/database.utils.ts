// Database utility functions
import type { DatabaseConfig } from '../../../shared/types/database.types';
import { DatabaseType } from '../types/database.types';

export function validateDatabaseConfig(config: Partial<DatabaseConfig>): string[] {
  const errors: string[] = [];

  if (!config.name?.trim()) {
    errors.push('数据库名称不能为空');
  }

  if (!config.type) {
    errors.push('数据库类型不能为空');
  }

  if (config.type !== DatabaseType.Redis) {
    if (!config.host?.trim()) {
      errors.push('主机地址不能为空');
    }

    if (!config.port || config.port <= 0 || config.port > 65535) {
      errors.push('端口必须是1-65535之间的有效数字');
    }
  }

  return errors;
}

export function getDatabaseIcon(type: DatabaseType): string {
  const iconMap = {
    [DatabaseType.MySQL]: '/icon_mysql.svg',
    [DatabaseType.PostgreSQL]: '/icon_postgresql.svg',
    [DatabaseType.Redis]: '/icon_redis.svg'
  };
  return iconMap[type] || '/icon_database.svg';
}

export function getDatabaseColor(type: DatabaseType): string {
  const colorMap = {
    [DatabaseType.MySQL]: '#4CAF50',
    [DatabaseType.PostgreSQL]: '#3F51B5',
    [DatabaseType.Redis]: '#FF5722'
  };
  return colorMap[type] || '#757575';
}

export function generateConnectionId(): string {
  return `db_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}