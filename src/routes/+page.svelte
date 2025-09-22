<script lang="ts">
  import { onMount } from "svelte";
  import MainDashboard from "../components/MainDashboard.svelte";
  import DatabaseConfigForm from "../components/DatabaseConfigForm.svelte";
  import TaskCreationForm from "../components/TaskCreationForm.svelte";
  import TaskRunner from "../components/TaskRunner.svelte";

  // 应用状态
  let currentView = "dashboard"; // dashboard, databaseConfig, migrationPipeline, taskManagement
  let selectedDatabaseId: string | null = null;
  let selectedTaskId: string | null = null;
  let showAddDatabaseForm = false;
  let showCreateTaskForm = false;

  // 导航到不同页面
  const navigateToDashboard = () => {
    currentView = "dashboard";
    showAddDatabaseForm = false;
    showCreateTaskForm = false;
  };

  const navigateToDatabaseConfig = () => {
    currentView = "databaseConfig";
    showAddDatabaseForm = false;
    showCreateTaskForm = false;
  };

  const navigateToMigrationPipeline = () => {
    currentView = "migrationPipeline";
    showAddDatabaseForm = false;
    showCreateTaskForm = false;
  };

  const navigateToTaskManagement = () => {
    currentView = "taskManagement";
    showAddDatabaseForm = false;
    showCreateTaskForm = false;
  };

  // 打开添加数据库表单
  const openAddDatabaseForm = () => {
    showAddDatabaseForm = true;
    selectedDatabaseId = null;
  };

  // 打开编辑数据库表单
  const openEditDatabaseForm = (id: string) => {
    showAddDatabaseForm = true;
    selectedDatabaseId = id;
  };

  // 打开创建任务表单
  const openCreateTaskForm = () => {
    showCreateTaskForm = true;
  };

  // 打开运行任务页面
  const openTaskRunner = (id: string) => {
    showCreateTaskForm = false;
    selectedTaskId = id;
  };

  // 初始化应用
  onMount(() => {
    // 这里可以添加应用初始化逻辑
  });
</script>

<div class="app-container">
  <header class="app-header">
    <div class="header-left">
      <h1>Bodhi Dump</h1>
    </div>
    <nav class="app-nav">
      <button
        class={`nav-button ${currentView === "dashboard" ? "active" : ""}`}
        on:click={navigateToDashboard}
      >
        <i class="icon-dashboard"></i>
        仪表板
      </button>
      <button
        class={`nav-button ${currentView === "databaseConfig" ? "active" : ""}`}
        on:click={navigateToDatabaseConfig}
      >
        <i class="icon-database"></i>
        数据库配置
      </button>
      <button
        class={`nav-button ${currentView === "migrationPipeline" ? "active" : ""}`}
        on:click={navigateToMigrationPipeline}
      >
        <i class="icon-pipeline"></i>
        迁移流水线
      </button>
      <button
        class={`nav-button ${currentView === "taskManagement" ? "active" : ""}`}
        on:click={navigateToTaskManagement}
      >
        <i class="icon-tasks"></i>
        任务管理
      </button>
    </nav>
  </header>

  <main class="app-main">
    {#if showAddDatabaseForm}
      <DatabaseConfigForm databaseId={selectedDatabaseId} />
    {:else if showCreateTaskForm}
      <TaskCreationForm />
    {:else if selectedTaskId}
      <TaskRunner taskId={selectedTaskId} />
    {:else if currentView === "dashboard" || currentView === "databaseConfig" || currentView === "migrationPipeline" || currentView === "taskManagement"}
      <MainDashboard
        view={currentView}
        onAddDatabase={openAddDatabaseForm}
        onEditDatabase={openEditDatabaseForm}
        onCreateTask={openCreateTaskForm}
        onViewTask={openTaskRunner}
      />
    {/if}
  </main>
</div>

<style>
  /* Apple Design System Foundation */
  :root {
    /* Typography */
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display",
      "SF Pro Text", sans-serif;
    font-size: 16px;
    line-height: 1.5;

    /* Colors - Apple Human Interface Guidelines */
    --apple-background-primary: #f2f2f7;
    --apple-background-secondary: #ffffff;
    --apple-background-tertiary: #f9f9f9;
    --apple-text-primary: #000000;
    --apple-text-secondary: #3c3c43;
    --apple-text-tertiary: #8e8e93;
    --apple-accent-blue: #007aff;
    --apple-accent-green: #34c759;
    --apple-accent-orange: #ff9500;
    --apple-accent-red: #ff3b30;
    --apple-border: #e5e5ea;

    /* Spacing & Sizing */
    --apple-border-radius-small: 6px;
    --apple-border-radius-medium: 10px;
    --apple-border-radius-large: 14px;
  }

  * {
    box-sizing: border-box;
  }

  /* 全局滚动条控制 - 确保只有一个滚动条 */
  :global(html),
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    height: 100%;
    color: var(--apple-text-primary);
    background-color: var(--apple-background-primary);
  }

  .app-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--apple-background-primary);
    overflow: hidden;
  }

  /* Header */
  .app-header {
    background-color: var(--apple-background-secondary);
    border-bottom: 1px solid var(--apple-border);
    padding: 0 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 44px;
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    z-index: 10;
  }

  .header-left h1 {
    margin: 0;
    font-size: 17px;
    font-weight: 600;
    color: var(--apple-text-primary);
    letter-spacing: -0.4px;
  }

  /* Navigation */
  .app-nav {
    display: flex;
    gap: 16px;
  }

  .nav-button {
    padding: 6px 14px;
    border: none;
    background: none;
    border-radius: var(--apple-border-radius-small);
    font-size: 15px;
    color: var(--apple-text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 500;
  }

  .nav-button:hover {
    background-color: var(--apple-background-tertiary);
    color: var(--apple-text-primary);
  }

  .nav-button.active {
    color: var(--apple-accent-blue);
    background-color: rgba(0, 122, 255, 0.09);
  }

  /* Main Content */
  .app-main {
    flex: 1;
    padding: 16px;
    overflow-y: auto;
    height: calc(100vh - 44px);
    position: relative;
    z-index: 1;
  }

  /* 自定义滚动条样式 - 确保只有.app-main显示滚动条 */
  .app-main::-webkit-scrollbar {
    width: 8px;
  }

  .app-main::-webkit-scrollbar-track {
    background: transparent;
  }

  .app-main::-webkit-scrollbar-thumb {
    background-color: rgba(0, 0, 0, 0.15);
    border-radius: 4px;
  }

  .app-main::-webkit-scrollbar-thumb:hover {
    background-color: rgba(0, 0, 0, 0.25);
  }

  /* 其他浏览器兼容性 */
  .app-main {
    scrollbar-width: thin;
    scrollbar-color: rgba(0, 0, 0, 0.15) transparent;
  }

  /* Icons - SF Symbols style */
  [class^="icon-"]::before {
    display: inline-block;
    width: 16px;
    height: 16px;
    text-align: center;
    font-style: normal;
    font-variant: tabular-nums;
  }

  .icon-dashboard::before {
    content: "📊";
  }

  .icon-database::before {
    content: "🗄️";
  }

  .icon-pipeline::before {
    content: "⚙️";
  }

  .icon-tasks::before {
    content: "📋";
  }
</style>
