<script lang="ts">
  import { onMount } from 'svelte';
  import { databases, pipelineTasks, appLoading, loadDatabaseConfigs } from '../stores/appStore';
  import type { DatabaseConfig, PipelineTask } from '../types/database';
  
  // 加载应用数据
  onMount(async () => {
    try {
      await loadDatabaseConfigs();
      // 这里可以添加加载其他数据的逻辑
    } catch (error) {
      console.error('Failed to load app data:', error);
    } finally {
      appLoading.set(false);
    }
  });
  
  // 获取运行中的任务数量
  const runningTasksCount = $pipelineTasks.filter(task => task.status === 'running').length;
  
  // 获取已完成的任务数量
  const completedTasksCount = $pipelineTasks.filter(task => task.status === 'completed').length;
  
  // 获取失败的任务数量
  const failedTasksCount = $pipelineTasks.filter(task => task.status === 'failed').length;
  
  // 获取等待中的任务数量
  const pendingTasksCount = $pipelineTasks.filter(task => task.status === 'pending').length;
</script>

<div class="dashboard-container">
  {#if $appLoading}
    <div class="loading">
      <h2>加载中...</h2>
    </div>
  {:else}
    <header class="dashboard-header">
      <h1>数据库迁移控制台</h1>
      <p>管理和监控数据库迁移任务</p>
    </header>
    
    <section class="overview-section">
      <div class="overview-card">
        <h3>数据库配置</h3>
        <p class="count">{$databases.length}</p>
        <p class="description">已配置的数据库数量</p>
      </div>
      
      <div class="overview-card">
        <h3>运行中任务</h3>
        <p class="count running">{runningTasksCount}</p>
        <p class="description">当前正在执行的迁移任务</p>
      </div>
      
      <div class="overview-card">
        <h3>已完成任务</h3>
        <p class="count completed">{completedTasksCount}</p>
        <p class="description">成功完成的迁移任务</p>
      </div>
      
      <div class="overview-card">
        <h3>失败任务</h3>
        <p class="count failed">{failedTasksCount}</p>
        <p class="description">执行失败的迁移任务</p>
      </div>
      
      <div class="overview-card">
        <h3>等待中任务</h3>
        <p class="count pending">{pendingTasksCount}</p>
        <p class="description">等待执行的迁移任务</p>
      </div>
    </section>
    
    <section class="databases-section">
      <div class="section-header">
        <h2>已配置数据库</h2>
        <button class="add-button">添加数据库</button>
      </div>
      
      {#if $databases.length === 0}
        <div class="empty-state">
          <p>尚未配置任何数据库</p>
          <button class="primary-button">点击添加第一个数据库</button>
        </div>
      {:else}
        <div class="databases-grid">
          {#each $databases as database}
            <div class="database-card">
              <h3>{database.name}</h3>
              <p class="database-type">{database.type}</p>
              <p class="database-details">
                {database.host || database.path || 'localhost'}
                {database.port ? `:${database.port}` : ''}
                {database.database ? `/${database.database}` : ''}
              </p>
              <div class="card-actions">
                <button class="edit-button">编辑</button>
                <button class="delete-button">删除</button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </section>
    
    <section class="tasks-section">
      <div class="section-header">
        <h2>最近任务</h2>
        <button class="add-button">创建新任务</button>
      </div>
      
      {#if $pipelineTasks.length === 0}
        <div class="empty-state">
          <p>尚未创建任何迁移任务</p>
          <button class="primary-button">点击创建第一个任务</button>
        </div>
      {:else}
        <div class="tasks-table">
          <table>
            <thead>
              <tr>
                <th>任务名称</th>
                <th>源数据库</th>
                <th>目标数据库</th>
                <th>状态</th>
                <th>进度</th>
                <th>开始时间</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {#each $pipelineTasks.slice(0, 5) as task}
                <tr>
                  <td>{task.name}</td>
                  <td>
                    {#if $databases.find(db => db.id === task.sourceDbId)}
                      {$databases.find(db => db.id === task.sourceDbId)?.name}
                    {:else}
                      未知
                    {/if}
                  </td>
                  <td>
                    {#if $databases.find(db => db.id === task.targetDbId)}
                      {$databases.find(db => db.id === task.targetDbId)?.name}
                    {:else}
                      未知
                    {/if}
                  </td>
                  <td>
                    <span class={`status-badge ${task.status}`}>
                      {task.status === 'running' ? '运行中' : 
                       task.status === 'completed' ? '已完成' : 
                       task.status === 'failed' ? '失败' : 
                       task.status === 'paused' ? '已暂停' : '等待中'}
                    </span>
                  </td>
                  <td>
                    <div class="progress-bar">
                      <div 
                        class="progress-fill"
                        style="width: ${task.progress}%"
                      ></div>
                    </div>
                  </td>
                  <td>{task.startTime ? new Date(task.startTime).toLocaleString() : '-'}</td>
                  <td>
                    <button class="view-button">查看</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </section>
  {/if}
</div>

<style>
  .dashboard-container {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }
  
  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 50vh;
  }
  
  .dashboard-header {
    text-align: center;
    margin-bottom: 40px;
  }
  
  .dashboard-header h1 {
    font-size: 2.5rem;
    color: #333;
    margin-bottom: 10px;
  }
  
  .dashboard-header p {
    font-size: 1.2rem;
    color: #666;
  }
  
  .overview-section {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 20px;
    margin-bottom: 40px;
  }
  
  .overview-card {
    background: white;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    text-align: center;
  }
  
  .overview-card h3 {
    font-size: 1rem;
    color: #666;
    margin-bottom: 10px;
  }
  
  .count {
    font-size: 2.5rem;
    font-weight: bold;
    margin: 10px 0;
  }
  
  .count.running {
    color: #4CAF50;
  }
  
  .count.completed {
    color: #2196F3;
  }
  
  .count.failed {
    color: #f44336;
  }
  
  .count.pending {
    color: #FF9800;
  }
  
  .description {
    font-size: 0.9rem;
    color: #999;
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }
  
  .section-header h2 {
    font-size: 1.5rem;
    color: #333;
  }
  
  .add-button {
    background-color: #2196F3;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 8px 16px;
    cursor: pointer;
    font-size: 1rem;
  }
  
  .add-button:hover {
    background-color: #0b7dda;
  }
  
  .empty-state {
    text-align: center;
    padding: 40px;
    background: #f9f9f9;
    border-radius: 8px;
  }
  
  .empty-state p {
    color: #666;
    margin-bottom: 20px;
  }
  
  .primary-button {
    background-color: #4CAF50;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 10px 20px;
    cursor: pointer;
    font-size: 1rem;
  }
  
  .primary-button:hover {
    background-color: #45a049;
  }
  
  .databases-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;
  }
  
  .database-card {
    background: white;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .database-card h3 {
    font-size: 1.2rem;
    margin-bottom: 5px;
  }
  
  .database-type {
    color: #666;
    font-size: 0.9rem;
    margin-bottom: 10px;
  }
  
  .database-details {
    color: #999;
    font-size: 0.8rem;
    margin-bottom: 15px;
  }
  
  .card-actions {
    display: flex;
    gap: 10px;
  }
  
  .edit-button {
    background-color: #FF9800;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    cursor: pointer;
    font-size: 0.9rem;
  }
  
  .edit-button:hover {
    background-color: #e68900;
  }
  
  .delete-button {
    background-color: #f44336;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    cursor: pointer;
    font-size: 0.9rem;
  }
  
  .delete-button:hover {
    background-color: #d32f2f;
  }
  
  .tasks-table {
    overflow-x: auto;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    padding: 12px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }
  
  th {
    background-color: #f2f2f2;
  }
  
  .status-badge {
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.8rem;
    color: white;
  }
  
  .status-badge.running {
    background-color: #4CAF50;
  }
  
  .status-badge.completed {
    background-color: #2196F3;
  }
  
  .status-badge.failed {
    background-color: #f44336;
  }
  
  .status-badge.paused {
    background-color: #FF9800;
  }
  
  .status-badge.pending {
    background-color: #9E9E9E;
  }
  
  .progress-bar {
    width: 100%;
    height: 8px;
    background-color: #f2f2f2;
    border-radius: 4px;
    overflow: hidden;
  }
  
  .progress-fill {
    height: 100%;
    background-color: #4CAF50;
    border-radius: 4px;
  }
  
  .view-button {
    background-color: #2196F3;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    cursor: pointer;
    font-size: 0.9rem;
  }
  
  .view-button:hover {
    background-color: #0b7dda;
  }
</style>