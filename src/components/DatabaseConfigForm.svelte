<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { type DatabaseConfig, DatabaseType } from '../types/database';
  import { saveDatabaseConfig } from '../stores/appStore';
  
  // Props
  export let databaseId: string | null = null;
  
  // 表单数据
  let formData: DatabaseConfig = {
    id: '',
    name: '',
    type: DatabaseType.MySQL,
    host: 'localhost',
    port: 3306,
    username: 'root',
    password: '',
    database: '',
    path: '',
    ssl: false,
    extra: {} as Record<string, string>,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  };
  
  // 表单状态
  let isSubmitting = false;
  let submitSuccess = false;
  let errorMessage = '';
  
  // 辅助变量用于Redis数据库索引
  let redisDbIndex: string = '0';
  
  // 当编辑现有数据库时，加载数据库配置
  onMount(async () => {
    if (databaseId) {
      try {
        // 通过Tauri调用后端API获取数据库配置
        const config = await invoke('get_database_config', { id: databaseId });
        formData = { ...config as DatabaseConfig };
        // 初始化Redis数据库索引
        if (formData.type === DatabaseType.Redis && formData.extra?.['db']) {
          redisDbIndex = formData.extra['db'];
        }
      } catch (error) {
        console.error('Failed to load database config:', error);
        errorMessage = '加载数据库配置失败';
      }
    } else {
      // 生成新的ID
      formData.id = `db_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }
  });
  
  // 处理数据库类型变化
  const handleTypeChange = (type: DatabaseType) => {
    formData.type = type;
    
    // 根据数据库类型重置默认值
    switch (type) {
      case DatabaseType.MySQL:
        formData.port = 3306;
        break;
      case DatabaseType.Redis:
        formData.port = 6379;
        break;
      case DatabaseType.PostgreSQL:
        formData.port = 5432;
        break;
      case DatabaseType.SQLite:
        formData.path = './database.db';
        break;
      default:
        break;
    }
  };
  
  // 表单提交处理
  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    
    if (isSubmitting) return;
    
    isSubmitting = true;
    errorMessage = '';
    
    try {
      // 验证表单数据
      if (!formData.name.trim()) {
        throw new Error('请输入数据库名称');
      }
      
      if (formData.type !== DatabaseType.SQLite) {
        if (!formData.host?.trim()) {
          throw new Error('请输入数据库主机地址');
        }
        if (!formData.port) {
          throw new Error('请输入数据库端口');
        }
        if (!formData.username?.trim()) {
          throw new Error('请输入用户名');
        }
        if (formData.type !== DatabaseType.Redis && !formData.database?.trim()) {
          throw new Error('请输入数据库名称');
        }
      } else {
        if (!formData.path?.trim()) {
          throw new Error('请输入SQLite数据库文件路径');
        }
      }
      
      // 更新时间戳
      formData.updatedAt = new Date().toISOString();
      
      // 保存数据库配置
      await saveDatabaseConfig(formData);
      
      submitSuccess = true;
      
      // 3秒后重置成功状态
      setTimeout(() => {
        submitSuccess = false;
      }, 3000);
    } catch (error) {
      console.error('Failed to save database config:', error);
      errorMessage = error instanceof Error ? error.message : '保存数据库配置失败';
    } finally {
      isSubmitting = false;
    }
  };
</script>

<div class="database-config-form">
  <h2>{databaseId ? '编辑数据库配置' : '添加新数据库配置'}</h2>
  
  {#if errorMessage}
    <div class="error-message">{errorMessage}</div>
  {/if}
  
  {#if submitSuccess}
    <div class="success-message">数据库配置保存成功！</div>
  {/if}
  
  <form on:submit={handleSubmit}>
    <div class="form-group">
      <label for="name">数据库名称</label>
      <input
        id="name"
        type="text"
        bind:value={formData.name}
        placeholder="请输入数据库名称"
        required
      />
    </div>
    
    <div class="form-group">
      <label for="type">数据库类型</label>
      <select
        id="type"
        bind:value={formData.type}
        on:change={(e) => handleTypeChange(e.currentTarget.value as DatabaseType)}
        required
      >
        <option value={DatabaseType.MySQL}>MySQL</option>
        <option value={DatabaseType.Redis}>Redis</option>
        <option value={DatabaseType.SQLite}>SQLite</option>
        <option value={DatabaseType.PostgreSQL}>PostgreSQL</option>
        <option value={DatabaseType.MongoDB}>MongoDB</option>
      </select>
    </div>
    
    {#if formData.type !== DatabaseType.SQLite}
      <div class="form-group">
        <label for="host">主机地址</label>
        <input
          id="host"
          type="text"
          bind:value={formData.host}
          placeholder="请输入数据库主机地址"
          required
        />
      </div>
      
      <div class="form-group">
        <label for="port">端口</label>
        <input
          id="port"
          type="number"
          bind:value={formData.port}
          min="1"
          max="65535"
          placeholder="请输入数据库端口"
          required
        />
      </div>
    {:else}
      <div class="form-group">
        <label for="path">文件路径</label>
        <input
          id="path"
          type="text"
          bind:value={formData.path}
          placeholder="请输入SQLite数据库文件路径"
          required
        />
        <button type="button" class="browse-button">浏览...</button>
      </div>
    {/if}
    
    <div class="form-group">
      <label for="username">用户名</label>
      <input
        id="username"
        type="text"
        bind:value={formData.username}
        placeholder="请输入用户名"
        required={formData.type !== DatabaseType.SQLite}
      />
    </div>
    
    <div class="form-group">
      <label for="password">密码</label>
      <input
        id="password"
        type="password"
        bind:value={formData.password}
        placeholder="请输入密码"
      />
    </div>
    
    {#if formData.type !== DatabaseType.Redis && formData.type !== DatabaseType.SQLite}
      <div class="form-group">
        <label for="database">数据库名称</label>
        <input
          id="database"
          type="text"
          bind:value={formData.database}
          placeholder="请输入数据库名称"
          required
        />
      </div>
    {/if}
    
    <div class="form-group">
      <label for="ssl">使用SSL</label>
      <input
        id="ssl"
        type="checkbox"
        bind:checked={formData.ssl}
      />
    </div>
    
    {#if formData.type === DatabaseType.Redis}
      <div class="form-group">
        <label for="redis-db">数据库索引</label>
        <input
          id="redis-db"
          type="number"
          min="0"
          max="15"
          bind:value={redisDbIndex}
          placeholder="请输入Redis数据库索引（0-15）"
          on:change={(e) => {
            (formData.extra ??= {})['db'] = e.currentTarget.value;
            redisDbIndex = e.currentTarget.value;
          }}
        />
      </div>
    {/if}
    
    <div class="form-actions">
      <button type="submit" disabled={isSubmitting}>
        {isSubmitting ? '保存中...' : '保存配置'}
      </button>
      <button type="button" class="cancel-button">取消</button>
    </div>
  </form>
</div>

<style>
  .database-config-form {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }
  
  .database-config-form h2 {
    font-size: 1.5rem;
    color: #333;
    margin-bottom: 20px;
  }
  
  .error-message {
    background-color: #ffebee;
    color: #c62828;
    padding: 10px;
    border-radius: 4px;
    margin-bottom: 15px;
  }
  
  .success-message {
    background-color: #e8f5e9;
    color: #2e7d32;
    padding: 10px;
    border-radius: 4px;
    margin-bottom: 15px;
  }
  
  .form-group {
    margin-bottom: 20px;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 5px;
    color: #555;
    font-weight: 500;
  }
  
  .form-group input,
  .form-group select {
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
  }
  
  .form-group input[type="checkbox"] {
    width: auto;
    margin-right: 10px;
  }
  
  .browse-button {
    margin-left: 10px;
    padding: 10px 15px;
    background-color: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .browse-button:hover {
    background-color: #e0e0e0;
  }
  
  .form-actions {
    display: flex;
    gap: 10px;
    justify-content: flex-end;
    margin-top: 30px;
  }
  
  .form-actions button {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }
  
  .form-actions button[type="submit"] {
    background-color: #4CAF50;
    color: white;
  }
  
  .form-actions button[type="submit"]:hover:not(:disabled) {
    background-color: #45a049;
  }
  
  .form-actions button[type="submit"]:disabled {
    background-color: #ccc;
    cursor: not-allowed;
  }
  
  .cancel-button {
    background-color: #f44336;
    color: white;
  }
  
  .cancel-button:hover {
    background-color: #d32f2f;
  }
</style>