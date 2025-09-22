<script lang="ts">
  import { onMount, onDestroy } from "svelte";
import {
  databases,
  pipelineTasks,
  appLoading,
  loadDatabaseConfigs,
  removeDatabaseConfig
} from "../stores/appStore";
  import type { DatabaseConfig, PipelineTask } from "../types/database";
  import { Chart, registerables } from "chart.js";
  import type { ChartConfiguration } from "chart.js";

  // 注册 Chart.js 组件
  Chart.register(...registerables);

  // 用于存储 Chart 实例
  let taskStatusChart: Chart | null = null;

  // 接收从父组件传入的属性
  export let view: string = "dashboard";
  export let onAddDatabase: () => void;
  export let onEditDatabase: (id: string) => void;
  export let onCreateTask: () => void;
  export let onViewTask: (id: string) => void;

  // 模拟数据，因为实际数据可能为空
  const mockDatabases: DatabaseConfig[] = [
    {
      id: "db_1",
      name: "MySQL - sales_db",
      type: "mysql",
      host: "192.168.1.100",
      port: 3306,
      username: "admin",
      database: "sales",
      ssl: false,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "db_2",
      name: "PostgreSQL - analytics_db",
      type: "postgresql",
      host: "192.168.1.101",
      port: 5432,
      username: "analyst",
      database: "analytics",
      ssl: true,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "db_3",
      name: "MongoDB - catalog",
      type: "mongodb",
      host: "192.168.1.102",
      port: 27017,
      username: "mongo_admin",
      database: "catalog",
      ssl: false,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "db_4",
      name: "SQLite - local_db",
      type: "sqlite",
      path: "./local.db",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "db_5",
      name: "Redis - cache",
      type: "redis",
      host: "192.168.1.103",
      port: 6379,
      extra: { db: "0" },
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
  ];

  const mockTasks: PipelineTask[] = [
    {
      id: "task_1",
      name: "订单数据迁移",
      sourceDbId: "db_1",
      targetDbId: "db_2",
      strategyId: "strategy_1",
      status: "running",
      progress: 65,
      startTime: new Date(Date.now() - 3600000).toISOString(),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "task_2",
      name: "客户数据同步",
      sourceDbId: "db_3",
      targetDbId: "db_1",
      strategyId: "strategy_2",
      status: "running",
      progress: 42,
      startTime: new Date(Date.now() - 1800000).toISOString(),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "task_3",
      name: "产品数据迁移",
      sourceDbId: "db_1",
      targetDbId: "db_3",
      strategyId: "strategy_3",
      status: "completed",
      progress: 100,
      startTime: new Date(Date.now() - 7200000).toISOString(),
      endTime: new Date(Date.now() - 3600000).toISOString(),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "task_4",
      name: "历史数据归档",
      sourceDbId: "db_2",
      targetDbId: "db_4",
      strategyId: "strategy_4",
      status: "failed",
      progress: 30,
      startTime: new Date(Date.now() - 86400000).toISOString(),
      error: "连接超时",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
    {
      id: "task_5",
      name: "缓存数据刷新",
      sourceDbId: "db_2",
      targetDbId: "db_5",
      strategyId: "strategy_5",
      status: "pending",
      progress: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    },
  ];

  // 计算增长率
  const getGrowthRate = (current: number, previous: number) => {
    if (previous === 0) return current > 0 ? "+100%" : "0%";
    const rate = (((current - previous) / previous) * 100).toFixed(0);
    return Number(rate) > 0 ? `+${rate}%` : `${rate}%`;
  };

  // 获取当前显示的数据（优先使用store数据，没有则使用模拟数据）
  const displayDatabases = $databases.length > 0 ? $databases : mockDatabases;
  const displayTasks = $pipelineTasks.length > 0 ? $pipelineTasks : mockTasks;

  // 任务状态统计
  const runningTasksCount = displayTasks.filter(
    (task) => task.status === "running"
  ).length;
  const completedTasksCount = displayTasks.filter(
    (task) => task.status === "completed"
  ).length;
  const failedTasksCount = displayTasks.filter(
    (task) => task.status === "failed"
  ).length;
  const pendingTasksCount = displayTasks.filter(
    (task) => task.status === "pending"
  ).length;

  let taskStatusChartCanvas: HTMLCanvasElement;

  // 图表数据配置
  const chartData = {
    labels: ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"],
    datasets: [
      {
        label: "MySQL",
        data: [12, 19, 13, 15, 20, 17],
        borderColor: "#4CAF50",
        backgroundColor: "rgba(76, 175, 80, 0.1)",
        tension: 0.4,
        fill: true,
      },
      {
        label: "PostgreSQL",
        data: [8, 12, 9, 15, 18, 14],
        borderColor: "#3F51B5",
        backgroundColor: "rgba(63, 81, 181, 0.1)",
        tension: 0.4,
        fill: true,
      },
      {
        label: "MongoDB",
        data: [5, 10, 8, 12, 15, 11],
        borderColor: "#FF9800",
        backgroundColor: "rgba(255, 152, 0, 0.1)",
        tension: 0.4,
        fill: true,
      },
    ],
  };

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      y: {
        beginAtZero: true,
        grid: {
          color: "rgba(0, 0, 0, 0.05)",
        },
      },
      x: {
        grid: {
          display: false,
        },
      },
    },
    plugins: {
      legend: {
        position: "top",
        labels: {
          usePointStyle: true,
          padding: 20,
        },
      },
    },
  };

  // 初始化图表
  onMount(() => {
    if (taskStatusChartCanvas) {
      taskStatusChart = new Chart(taskStatusChartCanvas, {
        type: "line",
        data: chartData,
        options: chartOptions,
      });
    }
  });

  // 销毁图表
  onDestroy(() => {
    if (taskStatusChart) {
      taskStatusChart.destroy();
      taskStatusChart = null;
    }
  });

  // 最近活动
  const recentActivities = [
    {
      id: "activity_1",
      type: "success",
      message: "任务执行成功 - 订单数据迁移",
      time: "5分钟前",
      icon: "✅",
    },
    {
      id: "activity_2",
      type: "info",
      message: "数据库连接成功 - PostgreSQL-analytics_db",
      time: "15分钟前",
      icon: "🔌",
    },
    {
      id: "activity_3",
      type: "warning",
      message: "迁移速度降低 - 客户数据同步",
      time: "30分钟前",
      icon: "⚠️",
    },
    {
      id: "activity_4",
      type: "error",
      message: "任务执行失败 - 历史数据归档",
      time: "1小时前",
      icon: "❌",
    },
    {
      id: "activity_5",
      type: "info",
      message: "任务开始 - 产品数据迁移",
      time: "2小时前",
      icon: "▶️",
    },
  ];

  // 活跃迁移任务（进度超过0%且不是已完成状态）
  const activeTasks = displayTasks.filter(
    (task) => task.progress > 0 && task.status !== "completed"
  );

  // 获取数据库名称
  const getDatabaseName = (id: string) => {
    const db = displayDatabases.find((db) => db.id === id);
    return db ? db.name : "未知数据库";
  };
</script>

<div class="dashboard-container">
  <!-- 主要内容区域 -->
  <main class="dashboard-main">
    {#if view === "dashboard"}
      <!-- 统计卡片区域 -->
      <section class="stats-cards">
        <div class="stat-card">
          <div class="stat-icon database-icon">🗄️</div>
          <div class="stat-content">
            <div class="stat-number">{displayDatabases.length}</div>
            <div class="stat-label">总数据库数</div>
            <div class="stat-change">
              <span class="change-indicator positive">↗</span>
              <span class="change-value">12%</span>
              <span class="change-text">相比上月</span>
            </div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon active-icon">⚡</div>
          <div class="stat-content">
            <div class="stat-number">{runningTasksCount}</div>
            <div class="stat-label">活跃迁移任务</div>
            <div class="stat-change">
              <span class="change-indicator positive">↗</span>
              <span class="change-value">5%</span>
              <span class="change-text">相比昨日</span>
            </div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon completed-icon">✅</div>
          <div class="stat-content">
            <div class="stat-number">{completedTasksCount}</div>
            <div class="stat-label">完成的迁移任务</div>
            <div class="stat-change">
              <span class="change-indicator positive">↗</span>
              <span class="change-value">8%</span>
              <span class="change-text">相比上周</span>
            </div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon failed-icon">❌</div>
          <div class="stat-content">
            <div class="stat-number">{failedTasksCount}</div>
            <div class="stat-label">失败的迁移任务</div>
            <div class="stat-change">
              <span class="change-indicator negative">↘</span>
              <span class="change-value">33%</span>
              <span class="change-text">相比上周</span>
            </div>
          </div>
        </div>
      </section>

      <!-- 图表和最近活动区域 -->
      <section class="charts-activities">
        <!-- 任务执行状态图表 -->
        <div class="chart-container">
          <div class="chart-header">
            <h3>任务执行状态</h3>
            <div class="chart-tabs">
              <button class="active">日</button>
              <button>周</button>
              <button>月</button>
            </div>
          </div>
          <div class="chart-content">
            <div class="chart-wrapper">
              <canvas bind:this={taskStatusChartCanvas}></canvas>
            </div>
          </div>
        </div>

        <!-- 最近活动 -->
        <div class="activities-container">
          <div class="activities-header">
            <h3>最近活动</h3>
            <button class="view-all-btn">查看全部</button>
          </div>
          <div class="activities-list">
            {#each recentActivities as activity}
              <div class={`activity-item ${activity.type}`}>
                <span class="activity-icon">{activity.icon}</span>
                <div class="activity-content">
                  <div class="activity-message">{activity.message}</div>
                  <div class="activity-time">{activity.time}</div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </section>

      <!-- 活跃迁移任务表格 -->
      <section class="active-tasks-section">
        <div class="section-header">
          <h3>活跃迁移任务</h3>
          <div class="section-actions">
            <button class="filter-btn">筛选</button>
            <button class="view-all-btn">查看全部</button>
          </div>
        </div>

        {#if activeTasks.length > 0}
          <div class="tasks-table">
            <table>
              <thead>
                <tr>
                  <th>任务名称</th>
                  <th>源数据库</th>
                  <th>目标数据库</th>
                  <th>状态</th>
                  <th>进度</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                {#each activeTasks as task}
                  <tr>
                    <td>{task.name}</td>
                    <td>{getDatabaseName(task.sourceDbId)}</td>
                    <td>{getDatabaseName(task.targetDbId)}</td>
                    <td>
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
                    </td>
                    <td>
                      <div class="progress-bar">
                        <div
                          class="progress-fill"
                          style="width: ${task.progress}%"
                        ></div>
                        <span class="progress-text">{task.progress}%</span>
                      </div>
                    </td>
                    <td>
                      <div class="task-actions">
                        <button class="action-btn view-btn">查看</button>
                        <button class="action-btn pause-btn">暂停</button>
                        <button class="action-btn stop-btn">停止</button>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {:else}
          <div class="empty-state">
            <p>当前没有活跃的迁移任务</p>
          </div>
        {/if}
      </section>
    {:else if view === "databaseConfig"}
      <section class="database-config-section">
        <div class="section-header">
          <h3>数据库配置</h3>
          <button class="primary-button" on:click={onAddDatabase}
            >添加数据库</button
          >
        </div>
        {#if displayDatabases.length > 0}
          <div class="databases-list">
            {#each displayDatabases as db}
              <div class="database-item">
                <div class="database-header">
                  <div class={`database-type-icon ${db.type.toLowerCase()}`}>
                    {#if db.type === 'MySQL'}
                      <span class="db-icon">🗄️</span>
                    {:else if db.type === 'PostgreSQL'}
                      <span class="db-icon">🐘</span>
                    {:else if db.type === 'MongoDB'}
                      <span class="db-icon">🍃</span>
                    {:else if db.type === 'SQLite'}
                      <span class="db-icon">💾</span>
                    {:else if db.type === 'Redis'}
                      <span class="db-icon">🔑</span>
                    {/if}
                  </div>
                  <div class="database-name-status">
                    <h4>{db.name}</h4>
                    <div class="status-indicator">
                      <span class={`status-dot ${db.name === 'customer_db' ? 'warning' : 'success'}`}></span>
                      <span class="status-text">{db.name === 'customer_db' ? '连接超时' : '已连接'}</span>
                    </div>
                  </div>
                </div>
                
                <div class="database-details">
                  <div class="detail-item">
                    <span class="detail-label">主机:</span>
                    <span class="detail-value">{db.host || 'localhost'}</span>
                  </div>
                  <div class="detail-item">
                    <span class="detail-label">端口:</span>
                    <span class="detail-value">{db.port}</span>
                  </div>
                  <div class="detail-item">
                    <span class="detail-label">用户名:</span>
                    <span class="detail-value">{db.username}</span>
                  </div>
                  {#if db.database}
                    <div class="detail-item">
                      <span class="detail-label">数据库:</span>
                      <span class="detail-value">{db.database}</span>
                    </div>
                  {/if}
                </div>
                
                <div class="database-actions">
                  <div class="action-buttons-left">
                    <button class="text-btn test-connection-btn" title="测试连接">
                      🔌 测试连接
                    </button>
                    <button class="text-btn view-details-btn" title="查看详情">
                      👁️ 查看详情
                    </button>
                  </div>
                  <div class="action-buttons-right">
                    <button class="icon-btn edit-btn" on:click={() => onEditDatabase(db.id)} title="编辑">
                      ⚙️
                    </button>
                    <button class="icon-btn delete-btn" on:click={async () => {
                      if (confirm(`确定要删除数据库 ${db.name} 吗？`)) {
                        await removeDatabaseConfig(db.id);
                      }
                    }} title="删除">
                      🗑️
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="empty-state">
            <p>当前没有配置的数据库</p>
            <button class="primary-button" on:click={onAddDatabase}
              >添加第一个数据库</button
            >
          </div>
        {/if}
      </section>
    {:else if view === "migrationPipeline"}
      <section class="migration-pipeline-section">
        <div class="section-header">
          <h3>迁移流水线</h3>
          <button class="primary-button" on:click={onCreateTask}
            >创建流水线</button
          >
        </div>
        
        <!-- 流水线任务列表 -->
        <div class="pipelines-list">
          <!-- MySQL到PostgreSQL的用户数据迁移 -->
          <div class="pipeline-card">
            <div class="pipeline-header">
              <h4>MySQL到PostgreSQL用户数据迁移</h4>
              <div class="pipeline-status completed">已完成</div>
            </div>
            
            <div class="pipeline-flow">
              <div class="database-box source">
                <div class="db-icon mysql">🗄️</div>
                <div class="db-info">
                  <div class="db-name">MySQL</div>
                  <div class="db-details">production_db</div>
                </div>
              </div>
              
              <div class="arrow">
                <div class="arrow-line"></div>
                <div class="arrow-icon">→</div>
              </div>
              
              <div class="database-box target">
                <div class="db-icon postgres">🐘</div>
                <div class="db-info">
                  <div class="db-name">PostgreSQL</div>
                  <div class="db-details">analytics_db</div>
                </div>
              </div>
            </div>
            
            <div class="pipeline-meta">
              <div class="meta-item">
                <span class="meta-label">上次运行:</span>
                <span class="meta-value">昨天 08:30</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">状态:</span>
                <span class="status-badge completed">成功</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">记录数:</span>
                <span class="meta-value">15,324</span>
              </div>
            </div>
            
            <div class="pipeline-actions">
              <button class="pipeline-btn view-btn" title="查看详情">
                👁️ 详情
              </button>
              <button class="pipeline-btn run-btn" title="运行任务">
                ▶️ 运行
              </button>
              <button class="pipeline-btn edit-btn" title="编辑任务">
                ⚙️ 编辑
              </button>
            </div>
          </div>
          
          <!-- SQL Server到MongoDB的产品数据同步 -->
          <div class="pipeline-card">
            <div class="pipeline-header">
              <h4>SQL Server到MongoDB产品数据同步</h4>
              <div class="pipeline-status running">运行中</div>
            </div>
            
            <div class="pipeline-flow">
              <div class="database-box source">
                <div class="db-icon sqlserver">🗂️</div>
                <div class="db-info">
                  <div class="db-name">SQL Server</div>
                  <div class="db-details">products</div>
                </div>
              </div>
              
              <div class="arrow">
                <div class="arrow-line running"></div>
                <div class="arrow-icon">→</div>
              </div>
              
              <div class="database-box target">
                <div class="db-icon mongodb">🍃</div>
                <div class="db-info">
                  <div class="db-name">MongoDB</div>
                  <div class="db-details">product_catalog</div>
                </div>
              </div>
            </div>
            
            <div class="pipeline-meta">
              <div class="meta-item">
                <span class="meta-label">开始时间:</span>
                <span class="meta-value">今天 10:15</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">状态:</span>
                <span class="status-badge running">运行中</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">进度:</span>
                <div class="progress-mini">
                  <div class="progress-fill" style="width: 65%"></div>
                </div>
                <span class="progress-text-mini">65%</span>
              </div>
            </div>
            
            <div class="pipeline-actions">
              <button class="pipeline-btn view-btn" title="查看详情">
                👁️ 详情
              </button>
              <button class="pipeline-btn pause-btn" title="暂停任务">
                ⏸️ 暂停
              </button>
              <button class="pipeline-btn stop-btn" title="停止任务">
                ⏹️ 停止
              </button>
            </div>
          </div>
          
          <!-- 多数据库聚合项目 -->
          <div class="pipeline-card">
            <div class="pipeline-header">
              <h4>多数据库聚合项目</h4>
              <div class="pipeline-status failed">失败</div>
            </div>
            
            <div class="pipeline-flow complex">
              <div class="database-box source">
                <div class="db-icon mysql">🗄️</div>
                <div class="db-info">
                  <div class="db-name">MySQL</div>
                  <div class="db-details">orders</div>
                </div>
              </div>
              
              <div class="arrow">
                <div class="arrow-line failed"></div>
                <div class="arrow-icon">→</div>
              </div>
              
              <div class="etl-box">
                <div class="etl-icon">🔄</div>
                <div class="etl-name">ETL处理</div>
              </div>
              
              <div class="arrow">
                <div class="arrow-line failed"></div>
                <div class="arrow-icon">→</div>
              </div>
              
              <div class="database-box target">
                <div class="db-icon redshift">🔴</div>
                <div class="db-info">
                  <div class="db-name">Redshift</div>
                  <div class="db-details">datawarehouse</div>
                </div>
              </div>
            </div>
            
            <div class="pipeline-meta">
              <div class="meta-item">
                <span class="meta-label">上次运行:</span>
                <span class="meta-value">前天 18:45</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">状态:</span>
                <span class="status-badge failed">失败</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">错误:</span>
                <span class="error-text">连接超时</span>
              </div>
            </div>
            
            <div class="pipeline-actions">
              <button class="pipeline-btn view-btn" title="查看详情">
                👁️ 详情
              </button>
              <button class="pipeline-btn rerun-btn" title="重新运行">
                🔄 重试
              </button>
              <button class="pipeline-btn edit-btn" title="编辑任务">
                ⚙️ 编辑
              </button>
            </div>
          </div>
        </div>
      </section>
    {:else if view === "taskManagement"}
      <section class="task-management-section">
        <div class="section-header">
          <h3>任务管理</h3>
          <div class="search-filter-container">
            <input type="text" placeholder="搜索任务" class="search-input" />
            <select class="filter-select">
              <option value="all">全部状态</option>
              <option value="running">运行中</option>
              <option value="completed">已完成</option>
              <option value="failed">失败</option>
              <option value="paused">已暂停</option>
              <option value="pending">等待中</option>
            </select>
            <button class="primary-button" on:click={onCreateTask}
              >创建任务</button
            >
          </div>
        </div>
        {#if displayTasks.length > 0}
          <div class="tasks-table">
            <table>
              <thead>
                <tr>
                  <th>任务名称</th>
                  <th>流水线</th>
                  <th>状态</th>
                  <th>进度</th>
                  <th>开始时间</th>
                  <th>预计完成时间</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                {#each displayTasks as task}
                  <tr>
                    <td>{task.name}</td>
                    <td>{task.pipeline || "-"}</td>
                    <td>
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
                    </td>
                    <td>
                      <div class="progress-bar">
                        <div class="progress-fill" style="width: {task.progress}%"></div>
                        <span class="progress-text">{task.progress}%</span>
                      </div>
                    </td>
                    <td>{task.startTime || "-"}</td>
                    <td>{task.estimatedEndTime || "-"}</td>
                    <td>
                      <div class="task-actions">
                        <button class="action-btn view-btn" on:click={() => onViewTask(task.id)}>查看</button>
                        {#if task.status === "running"}
                          <button class="action-btn pause-btn">暂停</button>
                        {/if}
                        {#if task.status === "paused" || task.status === "failed"}
                          <button class="action-btn" style="background-color: var(--apple-success); color: white;">继续</button>
                        {/if}
                        {#if task.status === "running" || task.status === "paused"}
                          <button class="action-btn stop-btn">停止</button>
                        {/if}
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
          <div class="pagination-controls">
            <span>显示 1 至 14 共 14 条</span>
            <div class="pagination-buttons">
              <button class="pagination-btn" disabled>&laquo;</button>
              <button class="pagination-btn active">1</button>
              <button class="pagination-btn">2</button>
              <button class="pagination-btn">3</button>
              <button class="pagination-btn">4</button>
              <button class="pagination-btn">5</button>
              <button class="pagination-btn">&raquo;</button>
            </div>
          </div>
        {:else}
          <div class="empty-state">
            <p>当前没有创建的任务</p>
            <button class="primary-button" on:click={onCreateTask}
              >创建第一个任务</button
            >
          </div>
        {/if}
      </section>
    {/if}
  </main>
</div>

<style>
  /* 全局样式重置 */
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto",
      "Oxygen", "Ubuntu", "Cantarell", sans-serif;
    background-color: #f5f7fa;
    color: #333;
  }

  /* 主容器样式 */
  .dashboard-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  /* 顶部导航栏 */
  .main-nav {
    background: white;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    padding: 0 24px;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    position: sticky;
    top: 0;
    z-index: 1000;
  }

  /* Apple Design System Variables */
  :root {
    --apple-font: -apple-system, BlinkMacSystemFont, "SF Pro", "Helvetica Neue",
      sans-serif;
    --apple-primary: #007aff;
    --apple-secondary: #5856d6;
    --apple-success: #34c759;
    --apple-warning: #ff9500;
    --apple-danger: #ff3b30;
    --apple-background: #f5f5f7;
    --apple-card-bg: #ffffff;
    --apple-text-primary: #111827;
    --apple-text-secondary: #6b7280;
    --apple-text-tertiary: #9ca3af;
    --apple-border-radius: 12px;
    --apple-border-width: 1px;
    --apple-border-color: #e5e7eb;
    --apple-shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
    --apple-shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  /* Logo Styles - Apple Design */
  .logo {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 18px;
    font-weight: 600;
    color: var(--apple-primary);
    font-family: var(--apple-font);
  }

  /* Main Content Area - Apple Design */
  .dashboard-main {
    flex: 1;
    padding: 24px;
    max-width: 1400px;
    width: 100%;
    margin: 0 auto;
    background-color: var(--apple-background);
    overflow: hidden;
    height: auto;
    position: relative;
  }

  /* Stats Cards - Apple Design */
  .stats-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
    margin-bottom: 24px;
  }

  .stat-card {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 20px;
    box-shadow: var(--apple-shadow-sm);
    display: flex;
    align-items: center;
    gap: 16px;
    transition: all 0.2s ease;
    border: var(--apple-border-width) solid var(--apple-border-color);
  }

  .stat-card:hover {
    transform: translateY(-1px);
    box-shadow: var(--apple-shadow-md);
    border-color: var(--apple-primary);
  }

  .stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
  }

  .database-icon {
    background-color: rgba(0, 122, 255, 0.1);
    color: var(--apple-primary);
  }

  .active-icon {
    background-color: rgba(255, 149, 0, 0.1);
    color: var(--apple-warning);
  }

  .completed-icon {
    background-color: rgba(52, 199, 89, 0.1);
    color: var(--apple-success);
  }

  .failed-icon {
    background-color: rgba(255, 59, 48, 0.1);
    color: var(--apple-danger);
  }

  .stat-content {
    flex: 1;
  }

  .stat-number {
    font-size: 28px;
    font-weight: 700;
    color: var(--apple-text-primary);
    margin-bottom: 4px;
    font-family: var(--apple-font);
  }

  .stat-label {
    font-size: 14px;
    color: var(--apple-text-secondary);
    margin-bottom: 8px;
    font-family: var(--apple-font);
  }

  .stat-change {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
  }

  .change-indicator {
    font-size: 14px;
  }

  .change-indicator.positive {
    color: var(--apple-success);
  }

  .change-indicator.negative {
    color: var(--apple-danger);
  }

  .change-value {
    font-weight: 500;
  }

  .change-text {
    color: var(--apple-text-tertiary);
  }

  /* Charts and Activities - Apple Design */
  .charts-activities {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 24px;
    margin-bottom: 24px;
  }

  /* Chart Container - Apple Design */
  .chart-container {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 20px;
    box-shadow: var(--apple-shadow-sm);
    border: var(--apple-border-width) solid var(--apple-border-color);
  }

  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .chart-header h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
  }

  .chart-tabs {
    display: flex;
    gap: 4px;
    background-color: var(--apple-background);
    padding: 4px;
    border-radius: 8px;
  }

  .chart-tabs button {
    padding: 6px 14px;
    border: none;
    background: none;
    border-radius: 6px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
    font-family: var(--apple-font);
    color: var(--apple-text-secondary);
  }

  .chart-tabs button.active {
    background: var(--apple-primary);
    color: white;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .chart-content {
    height: 300px;
  }

  .chart-wrapper {
    height: 100%;
    position: relative;
  }

  .chart-wrapper canvas {
    max-width: 100%;
    height: 100% !important;
  }

  /* Recent Activities - Apple Design */
  .activities-container {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 20px;
    box-shadow: var(--apple-shadow-sm);
    border: var(--apple-border-width) solid var(--apple-border-color);
  }

  .activities-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .activities-header h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
  }

  .view-all-btn {
    background: none;
    border: none;
    color: var(--apple-primary);
    font-size: 12px;
    cursor: pointer;
    padding: 6px 10px;
    border-radius: 6px;
    transition: background-color 0.3s ease;
    font-family: var(--apple-font);
  }

  .view-all-btn:hover {
    background-color: rgba(0, 122, 255, 0.1);
  }

  .activities-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .activity-item {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px;
    border-radius: 8px;
    background-color: var(--apple-background);
    transition: all 0.2s ease;
  }

  .activity-item:hover {
    background-color: rgba(0, 122, 255, 0.05);
  }

  .activity-item.success {
    border-left: 3px solid var(--apple-success);
  }

  .activity-item.info {
    border-left: 3px solid var(--apple-primary);
  }

  .activity-item.warning {
    border-left: 3px solid var(--apple-warning);
  }

  .activity-item.error {
    border-left: 3px solid var(--apple-danger);
  }

  .activity-icon {
    font-size: 16px;
    margin-top: 2px;
  }

  .activity-content {
    flex: 1;
  }

  .activity-message {
    font-size: 14px;
    color: var(--apple-text-primary);
    margin-bottom: 4px;
    font-family: var(--apple-font);
  }

  .activity-time {
    font-size: 12px;
    color: var(--apple-text-tertiary);
    font-family: var(--apple-font);
  }

  /* Active Tasks - Apple Design */
  .active-tasks-section {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 20px;
    box-shadow: var(--apple-shadow-sm);
    border: var(--apple-border-width) solid var(--apple-border-color);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .section-header h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
  }

  .section-actions {
    display: flex;
    gap: 8px;
  }

  .filter-btn {
    background: none;
    border: 1px solid var(--apple-border-color);
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
    font-family: var(--apple-font);
    color: var(--apple-text-secondary);
  }

  .filter-btn:hover {
    border-color: var(--apple-primary);
    color: var(--apple-primary);
    background-color: rgba(0, 122, 255, 0.05);
  }

  /* Task Table - Apple Design */
  .tasks-table {
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    text-align: left;
    padding: 12px;
    font-size: 14px;
    font-family: var(--apple-font);
  }

  th {
    background-color: var(--apple-background);
    font-weight: 600;
    color: var(--apple-text-secondary);
    border-bottom: 1px solid var(--apple-border-color);
  }

  td {
    border-bottom: 1px solid var(--apple-border-color);
    color: var(--apple-text-primary);
  }

  tr:hover {
    background-color: rgba(0, 122, 255, 0.05);
  }

  /* Status Badges - Apple Design */
  .status-badge {
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 500;
    font-family: var(--apple-font);
  }

  .status-badge.running {
    background-color: rgba(52, 199, 89, 0.1);
    color: var(--apple-success);
    border: 1px solid rgba(52, 199, 89, 0.2);
  }

  .status-badge.completed {
    background-color: rgba(0, 122, 255, 0.1);
    color: var(--apple-primary);
    border: 1px solid rgba(0, 122, 255, 0.2);
  }

  .status-badge.failed {
    background-color: rgba(255, 59, 48, 0.1);
    color: var(--apple-danger);
    border: 1px solid rgba(255, 59, 48, 0.2);
  }

  .status-badge.pending {
    background-color: var(--apple-background);
    color: var(--apple-text-secondary);
    border: 1px solid var(--apple-border-color);
  }

  /* Progress Bar - Apple Design */
  .progress-bar {
    width: 100%;
    height: 14px;
    background-color: var(--apple-background);
    border-radius: 5px;
    overflow: hidden;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    background: var(--apple-primary);
    border-radius: 3px;
    transition: width 0.3s ease;
  }

  .progress-text {
    position: absolute;
    right: 8px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 12px;
    color: var(--apple-text-secondary);
    font-weight: 500;
    font-family: var(--apple-font);
  }

  /* Task Actions - Apple Design */
  .task-actions {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }

  .action-btn {
    padding: 5px 10px;
    border: none;
    border-radius: 6px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
    font-family: var(--apple-font);
    font-weight: 500;
    min-width: 50px;
    text-align: center;
  }

  .view-btn {
    background-color: var(--apple-primary);
    color: white;
  }

  .view-btn:hover {
    background-color: rgba(0, 122, 255, 0.9);
  }

  .pause-btn {
    background-color: var(--apple-warning);
    color: white;
  }

  .pause-btn:hover {
    background-color: rgba(255, 149, 0, 0.9);
  }

  .stop-btn {
    background-color: var(--apple-danger);
    color: white;
  }

  .stop-btn:hover {
    background-color: rgba(255, 59, 48, 0.9);
  }

  /* Search and Filter - Apple Design */
  .search-filter-container {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .search-input {
    padding: 8px 12px;
    border: 1px solid var(--apple-border-color);
    border-radius: 6px;
    background-color: var(--apple-background);
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
    font-size: 14px;
    width: 200px;
    transition: all 0.3s ease;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--apple-primary);
    background-color: var(--apple-card-bg);
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
  }

  .filter-select {
    padding: 8px 12px;
    border: 1px solid var(--apple-border-color);
    border-radius: 6px;
    background-color: var(--apple-background);
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
    font-size: 14px;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .filter-select:focus {
    outline: none;
    border-color: var(--apple-primary);
    background-color: var(--apple-card-bg);
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
  }

  /* Pagination - Apple Design */
  .pagination-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 20px;
    padding: 12px 0;
    color: var(--apple-text-secondary);
    font-family: var(--apple-font);
    font-size: 14px;
  }

  .pagination-buttons {
    display: flex;
    gap: 4px;
  }

  .pagination-btn {
    padding: 6px 10px;
    border: 1px solid var(--apple-border-color);
    background-color: var(--apple-card-bg);
    color: var(--apple-text-secondary);
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.3s ease;
    min-width: 32px;
    text-align: center;
  }

  .pagination-btn:hover:not(:disabled) {
    border-color: var(--apple-primary);
    color: var(--apple-primary);
    background-color: rgba(0, 122, 255, 0.05);
  }

  .pagination-btn.active {
    background-color: var(--apple-primary);
    color: white;
    border-color: var(--apple-primary);
  }

  .pagination-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Empty State - Apple Design */
  .empty-state {
    text-align: center;
    padding: 40px;
    color: var(--apple-text-tertiary);
    font-family: var(--apple-font);
  }

  /* Database Config Styles */
  .database-config-section .section-header {
    margin-bottom: 20px;
  }

  .databases-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 20px;
  }

  .database-item {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 20px;
    box-shadow: var(--apple-shadow-sm);
    border: var(--apple-border-width) solid var(--apple-border-color);
    display: flex;
    flex-direction: column;
    gap: 16px;
    transition: all 0.3s ease;
  }

  .database-item:hover {
    transform: translateY(-2px);
    box-shadow: var(--apple-shadow-md);
  }

  .database-header {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .database-type-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
  }

  .database-type-icon.mysql {
    background-color: rgba(255, 149, 0, 0.1);
    color: #FF9500;
  }

  .database-type-icon.postgresql {
    background-color: rgba(52, 199, 89, 0.1);
    color: #34C759;
  }

  .database-type-icon.mongodb {
    background-color: rgba(66, 133, 244, 0.1);
    color: #4285F4;
  }

  .database-type-icon.sqlite {
    background-color: rgba(156, 163, 175, 0.1);
    color: #9CA3AF;
  }

  .database-type-icon.redis {
    background-color: rgba(239, 68, 68, 0.1);
    color: #EF4444;
  }

  .database-name-status {
    flex: 1;
  }

  .database-name-status h4 {
    margin: 0 0 4px 0;
    font-size: 16px;
    font-weight: 600;
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .status-dot.success {
    background-color: var(--apple-success);
  }

  .status-dot.warning {
    background-color: var(--apple-warning);
  }

  .status-dot.error {
    background-color: var(--apple-danger);
  }

  .status-text {
    font-size: 12px;
    color: var(--apple-text-secondary);
    font-family: var(--apple-font);
  }

  .database-details {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .detail-label {
    font-size: 12px;
    color: var(--apple-text-tertiary);
    font-family: var(--apple-font);
  }

  .detail-value {
    font-size: 13px;
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
    font-weight: 500;
  }

  .database-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 16px;
    padding-top: 12px;
    border-top: 1px solid var(--apple-border-color);
  }
  
  .action-buttons-left {
    display: flex;
    gap: 16px;
  }
  
  .action-buttons-right {
    display: flex;
    gap: 8px;
  }
  
  .text-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 12px;
    color: var(--apple-text-secondary);
    font-family: var(--apple-font);
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-radius: 6px;
    transition: all 0.2s ease;
  }
  
  .text-btn:hover {
    background-color: rgba(0, 122, 255, 0.05);
    color: var(--apple-primary);
  }
  
  .test-connection-btn:hover {
    color: var(--apple-success);
  }
  
  .icon-btn {
    width: 28px;
    height: 28px;
    border: 1px solid var(--apple-border-color);
    background-color: transparent;
    color: var(--apple-text-secondary);
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .icon-btn:hover {
    background-color: rgba(0, 122, 255, 0.05);
  }

  .delete-btn:hover {
    background-color: rgba(255, 59, 48, 0.1);
    border-color: rgba(255, 59, 48, 0.3);
    color: #FF3B30;
  }

  .edit-btn:hover {
    background-color: rgba(0, 122, 255, 0.05);
    border-color: var(--apple-border-color);
    color: var(--apple-primary);
  }

  /* Pipeline and Task Management Styles */
  .migration-pipeline-section .section-header,
  .task-management-section .section-header {
    margin-bottom: 20px;
  }

  /* Pipeline List Styles */
  .pipelines-list {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  /* Pipeline Card Styles */
  .pipeline-card {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 20px;
    box-shadow: var(--apple-shadow-sm);
    border: var(--apple-border-width) solid var(--apple-border-color);
    transition: all 0.3s ease;
  }

  .pipeline-card:hover {
    box-shadow: var(--apple-shadow-md);
    transform: translateY(-1px);
  }

  /* Pipeline Header */
  .pipeline-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .pipeline-header h4 {
    font-size: 16px;
    font-weight: 600;
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
    margin: 0;
  }

  .pipeline-status {
    padding: 4px 12px;
    border-radius: 16px;
    font-size: 12px;
    font-weight: 500;
    font-family: var(--apple-font);
  }

  .pipeline-status.running {
    background-color: rgba(255, 149, 0, 0.1);
    color: var(--apple-warning);
  }

  .pipeline-status.completed {
    background-color: rgba(52, 199, 89, 0.1);
    color: var(--apple-success);
  }

  .pipeline-status.failed {
    background-color: rgba(255, 59, 48, 0.1);
    color: var(--apple-danger);
  }

  /* Pipeline Flow */
  .pipeline-flow {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 24px;
    margin: 20px 0;
    padding: 20px;
    background-color: var(--apple-background);
    border-radius: 12px;
  }

  .pipeline-flow.complex {
    flex-wrap: wrap;
  }

  /* Database Box */
  .database-box {
    display: flex;
    align-items: center;
    gap: 12px;
    background: white;
    padding: 12px 16px;
    border-radius: 8px;
    box-shadow: var(--apple-shadow-sm);
    border: 1px solid var(--apple-border-color);
    min-width: 160px;
  }

  .db-icon {
    width: 32px;
    height: 32px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
  }

  .db-icon.mysql {
    background-color: rgba(255, 149, 0, 0.1);
    color: #FF9500;
  }

  .db-icon.postgres {
    background-color: rgba(52, 199, 89, 0.1);
    color: #34C759;
  }

  .db-icon.sqlserver {
    background-color: rgba(0, 122, 255, 0.1);
    color: var(--apple-primary);
  }

  .db-icon.mongodb {
    background-color: rgba(66, 133, 244, 0.1);
    color: #4285F4;
  }

  .db-icon.redshift {
    background-color: rgba(239, 68, 68, 0.1);
    color: #EF4444;
  }

  .db-info {
    display: flex;
    flex-direction: column;
  }

  .db-name {
    font-size: 14px;
    font-weight: 500;
    color: var(--apple-text-primary);
    font-family: var(--apple-font);
  }

  .db-details {
    font-size: 12px;
    color: var(--apple-text-secondary);
    font-family: var(--apple-font);
  }

  /* Arrow */
  .arrow {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .arrow-line {
    width: 60px;
    height: 2px;
    background-color: #E5E7EB;
    position: relative;
  }

  .arrow-line.running {
    background: linear-gradient(90deg, var(--apple-primary), var(--apple-success));
    animation: pulse 2s infinite;
  }

  .arrow-line.failed {
    background-color: var(--apple-danger);
  }

  .arrow-icon {
    font-size: 16px;
    color: var(--apple-text-secondary);
  }

  /* ETL Box */
  .etl-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    background: white;
    padding: 12px 16px;
    border-radius: 8px;
    box-shadow: var(--apple-shadow-sm);
    border: 1px solid var(--apple-border-color);
    min-width: 100px;
  }

  .etl-icon {
    width: 32px;
    height: 32px;
    border-radius: 6px;
    background-color: rgba(156, 163, 175, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    color: var(--apple-text-secondary);
  }

  .etl-name {
    font-size: 12px;
    color: var(--apple-text-secondary);
    font-family: var(--apple-font);
  }

  /* Pipeline Meta */
  .pipeline-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 20px;
    margin: 16px 0;
    padding-top: 16px;
    border-top: 1px solid var(--apple-border-color);
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .meta-label {
    font-size: 12px;
    color: var(--apple-text-tertiary);
    font-family: var(--apple-font);
  }

  .meta-value {
    font-size: 12px;
    color: var(--apple-text-secondary);
    font-family: var(--apple-font);
  }

  .status-badge {
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 500;
    font-family: var(--apple-font);
  }

  .status-badge.running {
    background-color: rgba(255, 149, 0, 0.1);
    color: var(--apple-warning);
  }

  .status-badge.completed {
    background-color: rgba(52, 199, 89, 0.1);
    color: var(--apple-success);
  }

  .status-badge.failed {
    background-color: rgba(255, 59, 48, 0.1);
    color: var(--apple-danger);
  }

  /* Mini Progress Bar */
  .progress-mini {
    width: 60px;
    height: 4px;
    background-color: #E5E7EB;
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--apple-primary), var(--apple-success));
    border-radius: 2px;
  }

  .progress-text-mini {
    font-size: 11px;
    color: var(--apple-text-secondary);
    font-family: var(--apple-font);
    margin-left: 8px;
  }

  .error-text {
    font-size: 12px;
    color: var(--apple-danger);
    font-family: var(--apple-font);
  }

  /* Pipeline Actions */
  .pipeline-actions {
    display: flex;
    gap: 8px;
    padding-top: 16px;
    border-top: 1px solid var(--apple-border-color);
  }

  .pipeline-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 12px;
    border: 1px solid var(--apple-border-color);
    border-radius: 6px;
    background: white;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s ease;
    font-family: var(--apple-font);
    font-weight: 500;
    color: var(--apple-text-secondary);
  }

  .pipeline-btn:hover {
    background-color: rgba(0, 122, 255, 0.05);
    border-color: var(--apple-primary);
    color: var(--apple-primary);
  }

  .pipeline-btn.view-btn:hover {
    background-color: rgba(0, 122, 255, 0.05);
  }

  .pipeline-btn.run-btn:hover {
    background-color: rgba(52, 199, 89, 0.05);
    border-color: var(--apple-success);
    color: var(--apple-success);
  }

  .pipeline-btn.rerun-btn:hover {
    background-color: rgba(52, 199, 89, 0.05);
    border-color: var(--apple-success);
    color: var(--apple-success);
  }

  .pipeline-btn.pause-btn:hover {
    background-color: rgba(255, 149, 0, 0.05);
    border-color: var(--apple-warning);
    color: var(--apple-warning);
  }

  .pipeline-btn.stop-btn:hover {
    background-color: rgba(255, 59, 48, 0.05);
    border-color: var(--apple-danger);
    color: var(--apple-danger);
  }

  .pipeline-btn.edit-btn:hover {
    background-color: rgba(88, 86, 214, 0.05);
    border-color: var(--apple-secondary);
    color: var(--apple-secondary);
  }

  /* Animation */
  @keyframes pulse {
    0% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
    100% {
      opacity: 0.6;
    }
  }

  /* Responsive Design - Apple Design */
  @media (max-width: 1200px) {
    .charts-activities {
      grid-template-columns: 1fr;
    }

    .chart-content {
      height: 350px;
    }
  }

  @media (max-width: 768px) {
    .dashboard-main {
      padding: 16px;
    }

    .stats-cards {
      grid-template-columns: 1fr;
      gap: 16px;
    }

    .section-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 12px;
    }

    .section-actions {
      width: 100%;
      justify-content: space-between;
    }

    .tasks-table {
      margin: -12px;
      padding: 12px;
    }

    th,
    td {
      padding: 8px;
      font-size: 12px;
    }

    .task-actions {
      flex-direction: column;
      gap: 4px;
    }
  }
</style>
