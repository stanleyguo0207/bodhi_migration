// Database form types
export enum DatabaseType {
  MySQL = 'mysql',
  PostgreSQL = 'postgresql',
  Redis = 'redis'
}

export interface DatabaseFormData {
  name: string;
  type: string;
  host: string;
  port: string;
  username: string;
  password: string;
  database: string;
  ssl: boolean;
}

export interface ValidationError {
  field: string;
  message: string;
}

export interface ConnectionTestResult {
  success: boolean;
  message: string;
}