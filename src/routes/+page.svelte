<script lang="ts">
  import { onMount } from 'svelte';
  import MainDashboard from '../components/MainDashboard.svelte';
  import DatabaseConfigForm from '../components/DatabaseConfigForm.svelte';
  import TaskCreationForm from '../components/TaskCreationForm.svelte';
  import TaskRunner from '../components/TaskRunner.svelte';
  
  // 应用状态
  let currentView = 'dashboard'; // dashboard, addDatabase, editDatabase, createTask, runTask
  let selectedDatabaseId: string | null = null;
  let selectedTaskId: string | null = null;
  
  // 导航到添加数据库页面
  const navigateToAddDatabase = () => {
    currentView = 'addDatabase';
    selectedDatabaseId = null;
  };
  
  // 导航到编辑数据库页面
  const navigateToEditDatabase = (id: string) => {
    currentView = 'editDatabase';
    selectedDatabaseId = id;
  };
  
  // 导航到创建任务页面
  const navigateToCreateTask = () => {
    currentView = 'createTask';
  };
  
  // 导航到运行任务页面
  const navigateToRunTask = (id: string) => {
    currentView = 'runTask';
    selectedTaskId = id;
  };
  
  // 返回主界面
  const navigateToDashboard = () => {
    currentView = 'dashboard';
    selectedDatabaseId = null;
    selectedTaskId = null;
  };
  
  // 初始化应用
  onMount(() => {
    // 这里可以添加应用初始化逻辑
  });
</script>

<header class="app-header">
  <div class="header-left">
    <h1>Bodhi Database Migration</h1>
  </div>
  <nav class="app-nav">
    <button 
      class={`nav-button ${currentView === 'dashboard' ? 'active' : ''}`}
      on:click={navigateToDashboard}
    >
      控制面板
    </button>
    <button 
      class="nav-button"
      on:click={navigateToAddDatabase}
    >
      添加数据库
    </button>
    <button 
      class="nav-button"
      on:click={navigateToCreateTask}
    >
      创建任务
    </button>
  </nav>
</header>

<main class="app-main">
  {#if currentView === 'dashboard'}
    <MainDashboard />
  {:else if currentView === 'addDatabase' || currentView === 'editDatabase'}
    <DatabaseConfigForm databaseId={selectedDatabaseId} />
  {:else if currentView === 'createTask'}
    <TaskCreationForm />
  {:else if currentView === 'runTask'}
    <TaskRunner taskId={selectedTaskId} />
  {/if}
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
  
  body {
    margin: 0;
    padding: 0;
  }
  
  .app-header {
    background-color: #2196F3;
    color: white;
    padding: 15px 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .header-left h1 {
    margin: 0;
    font-size: 1.5rem;
  }
  
  .app-nav {
    display: flex;
    gap: 10px;
  }
  
  .nav-button {
    background-color: rgba(255, 255, 255, 0.2);
    color: white;
    border: none;
    border-radius: 4px;
    padding: 8px 16px;
    cursor: pointer;
    font-size: 1rem;
    transition: background-color 0.3s;
  }
  
  .nav-button:hover {
    background-color: rgba(255, 255, 255, 0.3);
  }
  
  .nav-button.active {
    background-color: rgba(255, 255, 255, 0.4);
  }
  
  .app-main {
    padding: 20px;
  }
</style>
