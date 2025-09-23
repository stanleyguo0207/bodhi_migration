// Task form types
export interface TaskFormData {
  name: string;
  sourceDbId: string;
  targetDbId: string;
  strategyId: string;
}

export interface TaskStatus {
  status: 'pending' | 'running' | 'completed' | 'failed' | 'paused';
  progress: number;
  startTime?: string;
  endTime?: string;
  error?: string;
}

export interface TaskFilter {
  status?: string;
  search?: string;
  dateRange?: {
    start: Date;
    end: Date;
  };
}