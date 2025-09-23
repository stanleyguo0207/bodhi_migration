// Task utility functions
import type { PipelineTask } from '../../../shared/types/database.types';

export function getTaskStatusColor(status: string): string {
  const colorMap = {
    'pending': '#FFC107',
    'running': '#2196F3',
    'completed': '#4CAF50',
    'failed': '#F44336',
    'paused': '#FF9800'
  };
  return colorMap[status as keyof typeof colorMap] || '#757575';
}

export function getTaskStatusText(status: string): string {
  const textMap = {
    'pending': '待处理',
    'running': '运行中',
    'completed': '已完成',
    'failed': '失败',
    'paused': '已暂停'
  };
  return textMap[status as keyof typeof textMap] || '未知状态';
}

export function formatTaskDuration(startTime?: string, endTime?: string): string {
  if (!startTime) return '未开始';
  
  const start = new Date(startTime);
  const end = endTime ? new Date(endTime) : new Date();
  const duration = end.getTime() - start.getTime();
  
  if (duration < 60000) {
    return `${Math.floor(duration / 1000)}秒`;
  } else if (duration < 3600000) {
    return `${Math.floor(duration / 60000)}分钟`;
  } else {
    return `${Math.floor(duration / 3600000)}小时`;
  }
}

export function filterTasksByStatus(tasks: PipelineTask[], status: string): PipelineTask[] {
  return tasks.filter(task => task.status === status);
}

export function getActiveTasks(tasks: PipelineTask[]): PipelineTask[] {
  return tasks.filter(task => task.progress > 0 && task.status !== 'completed');
}