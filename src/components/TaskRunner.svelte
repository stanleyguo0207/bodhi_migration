<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { PipelineTask } from "../types/database";
  import {
    selectedTaskId,
    pipelineTasks,
    startPipelineTask,
  } from "../stores/appStore";

  // Props
  export let taskId: string | null = null;

  // 任务数据
  let task: PipelineTask | null = null;
  let logs: Array<{
    timestamp: string;
    message: string;
    level: "info" | "warning" | "error";
  }> = [];
  let progress = 0;
  let status = "pending";

  // 定时器引用 - 使用浏览器兼容的类型
  let interval: number | null = null;

  // 初始化
  onMount(() => {
    if (taskId) {
      selectedTaskId.set(taskId);
      loadTaskDetails();

      // 设置定时器定期刷新任务状态
      interval = setInterval(() => {
        loadTaskDetails();
      }, 2000); // 每2秒刷新一次
    }
  });

  // 清理定时器
  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });

  // 加载任务详情
  const loadTaskDetails = async () => {
    if (!taskId) return;

    try {
      // 通过Tauri调用后端API获取任务详情
      const taskDetails = await invoke("get_pipeline_task", { id: taskId });
      task = taskDetails as PipelineTask;

      // 更新进度和状态
      progress = task.progress;
      status = task.status;

      // 更新日志
      logs = task.logs || [];
    } catch (error) {
      console.error("Failed to load task details:", error);
    }
  };

  // 启动任务
  const handleStartTask = async () => {
    if (!taskId) return;

    try {
      await startPipelineTask(taskId);
      // 立即刷新任务状态
      await loadTaskDetails();
    } catch (error) {
      console.error("Failed to start task:", error);
    }
  };

  // 暂停任务
  const handlePauseTask = async () => {
    if (!taskId) return;

    try {
      await invoke("pause_pipeline_task", { id: taskId });
      await loadTaskDetails();
    } catch (error) {
      console.error("Failed to pause task:", error);
    }
  };

  // 取消任务
  const handleCancelTask = async () => {
    if (!taskId) return;

    try {
      await invoke("cancel_pipeline_task", { id: taskId });
      await loadTaskDetails();
    } catch (error) {
      console.error("Failed to cancel task:", error);
    }
  };

  // 重试任务
  const handleRetryTask = async () => {
    if (!taskId) return;

    try {
      await invoke("retry_pipeline_task", { id: taskId });
      await loadTaskDetails();
    } catch (error) {
      console.error("Failed to retry task:", error);
    }
  };
</script>

<div class="task-runner">
  {#if !task}
    <div class="loading">加载任务详情中...</div>
  {:else}
    <header class="task-header">
      <h1>{task.name}</h1>
      <div class="task-meta">
        <span>创建时间: {new Date(task.createdAt).toLocaleString()}</span>
        {#if task.startTime}
          <span>开始时间: {new Date(task.startTime).toLocaleString()}</span>
        {/if}
        {#if task.endTime}
          <span>结束时间: {new Date(task.endTime).toLocaleString()}</span>
        {/if}
      </div>
    </header>

    <section class="task-status-section">
      <div class="status-card">
        <h3>任务状态</h3>
        <div class="status-display">
          <span class={`status-badge ${task.status}`}>
            {task.status === "running"
              ? "运行中"
              : task.status === "completed"
                ? "已完成"
                : task.status === "failed"
                  ? "失败"
                  : task.status === "paused"
                    ? "已暂停"
                    : "等待中"}
          </span>
        </div>

        <div class="progress-section">
          <div class="progress-info">
            <span>进度</span>
            <span>{task.progress}%</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" style="width: ${task.progress}%"></div>
          </div>
        </div>
      </div>

      <div class="action-buttons">
        {#if task.status === "pending" || task.status === "paused"}
          <button class="start-button" on:click={handleStartTask}>开始</button>
        {/if}

        {#if task.status === "running"}
          <button class="pause-button" on:click={handlePauseTask}>暂停</button>
          <button class="cancel-button" on:click={handleCancelTask}>取消</button
          >
        {/if}

        {#if task.status === "failed" || task.status === "completed"}
          <button class="retry-button" on:click={handleRetryTask}>重试</button>
        {/if}
      </div>
    </section>

    <section class="task-details-section">
      <div class="details-grid">
        <div class="detail-card">
          <h3>源数据库</h3>
          <p>{task.sourceDbId}</p>
        </div>

        <div class="detail-card">
          <h3>目标数据库</h3>
          <p>{task.targetDbId}</p>
        </div>

        <div class="detail-card">
          <h3>迁移策略</h3>
          <p>{task.strategyId}</p>
        </div>
      </div>
    </section>

    <section class="task-logs-section">
      <h3>任务日志</h3>
      <div class="logs-container">
        {#if logs.length === 0}
          <div class="no-logs">暂无日志记录</div>
        {:else}
          {#each logs as log}
            <div class={`log-entry log-${log.level}`}>
              <span class="log-time"
                >{new Date(log.timestamp).toLocaleTimeString()}</span
              >
              <span class="log-message">{log.message}</span>
            </div>
          {/each}
        {/if}
      </div>
    </section>

    {#if task.error}
      <section class="task-error-section">
        <h3>错误信息</h3>
        <div class="error-details">
          <pre>{task.error}</pre>
        </div>
      </section>
    {/if}
  {/if}
</div>

<style>
  .task-runner {
    max-width: 1000px;
    margin: 0 auto;
    padding: 20px;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 50vh;
    font-size: 1.2rem;
    color: #666;
  }

  .task-header {
    margin-bottom: 30px;
  }

  .task-header h1 {
    font-size: 2rem;
    color: #333;
    margin-bottom: 10px;
  }

  .task-meta {
    display: flex;
    gap: 20px;
    color: #666;
    font-size: 0.9rem;
  }

  .task-status-section {
    background: white;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin-bottom: 30px;
  }

  .status-card {
    margin-bottom: 20px;
  }

  .status-card h3 {
    font-size: 1.2rem;
    color: #333;
    margin-bottom: 15px;
  }

  .status-display {
    margin-bottom: 20px;
  }

  .status-badge {
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 1rem;
    color: white;
    font-weight: 500;
  }

  .status-badge.running {
    background-color: #4caf50;
  }

  .status-badge.completed {
    background-color: #2196f3;
  }

  .status-badge.failed {
    background-color: #f44336;
  }

  .status-badge.paused {
    background-color: #ff9800;
  }

  .status-badge.pending {
    background-color: #9e9e9e;
  }

  .progress-section {
    margin-bottom: 20px;
  }

  .progress-info {
    display: flex;
    justify-content: space-between;
    margin-bottom: 10px;
    color: #666;
  }

  .progress-bar {
    width: 100%;
    height: 12px;
    background-color: #f2f2f2;
    border-radius: 6px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background-color: #4caf50;
    border-radius: 6px;
    transition: width 0.3s ease;
  }

  .action-buttons {
    display: flex;
    gap: 10px;
  }

  .action-buttons button {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }

  .start-button {
    background-color: #4caf50;
    color: white;
  }

  .start-button:hover {
    background-color: #45a049;
  }

  .pause-button {
    background-color: #ff9800;
    color: white;
  }

  .pause-button:hover {
    background-color: #e68900;
  }

  .cancel-button {
    background-color: #f44336;
    color: white;
  }

  .cancel-button:hover {
    background-color: #d32f2f;
  }

  .retry-button {
    background-color: #2196f3;
    color: white;
  }

  .retry-button:hover {
    background-color: #0b7dda;
  }

  .task-details-section {
    margin-bottom: 30px;
  }

  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
  }

  .detail-card {
    background: white;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .detail-card h3 {
    font-size: 1rem;
    color: #666;
    margin-bottom: 10px;
  }

  .detail-card p {
    font-size: 1.1rem;
    color: #333;
  }

  .task-logs-section {
    background: white;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin-bottom: 30px;
  }

  .task-logs-section h3 {
    font-size: 1.2rem;
    color: #333;
    margin-bottom: 15px;
  }

  .logs-container {
    max-height: 400px;
    overflow-y: auto;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 10px;
  }

  .no-logs {
    text-align: center;
    color: #999;
    padding: 20px;
  }

  .log-entry {
    display: flex;
    gap: 10px;
    padding: 8px;
    border-bottom: 1px solid #f0f0f0;
  }

  .log-entry:last-child {
    border-bottom: none;
  }

  .log-time {
    color: #999;
    font-size: 0.9rem;
    min-width: 100px;
  }

  .log-message {
    flex: 1;
  }

  .log-info {
    color: #2196f3;
  }

  .log-warning {
    color: #ff9800;
  }

  .log-error {
    color: #f44336;
  }

  .task-error-section {
    background: #ffebee;
    border-radius: 8px;
    padding: 20px;
    border-left: 4px solid #f44336;
  }

  .task-error-section h3 {
    font-size: 1.2rem;
    color: #c62828;
    margin-bottom: 15px;
  }

  .error-details pre {
    background: white;
    padding: 15px;
    border-radius: 4px;
    overflow-x: auto;
    color: #c62828;
  }
</style>
