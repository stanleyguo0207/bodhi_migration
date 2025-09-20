<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { PipelineTask, MigrationStrategy } from '../types/database';
  import { databases, strategies, createPipelineTask } from '../stores/appStore';
  
  // 表单数据
  let formData = {
    name: '',
    sourceDbId: '',
    targetDbId: '',
    strategyId: '',
    customStrategy: {
      type: 'full' as 'full' | 'incremental' | 'delta',
      batchSize: 1000,
      retryCount: 3,
      timeout: 3600
    }
  };
  
  // 表单状态
  let isSubmitting = false;
  let submitSuccess = false;
  let errorMessage = '';
  let useCustomStrategy = false;
  
  // 加载策略和数据库数据
  onMount(async () => {
    try {
      // 通过Tauri调用后端API加载策略
      const strategiesData = await invoke('get_all_strategies');
      strategies.set(strategiesData as MigrationStrategy[]);
    } catch (error) {
      console.error('Failed to load strategies:', error);
      errorMessage = '加载迁移策略失败';
    }
  });
  
  // 表单提交处理
  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    
    if (isSubmitting) return;
    
    isSubmitting = true;
    errorMessage = '';
    
    try {
      // 验证表单数据
      if (!formData.name.trim()) {
        throw new Error('请输入任务名称');
      }
      
      if (!formData.sourceDbId) {
        throw new Error('请选择源数据库');
      }
      
      if (!formData.targetDbId) {
        throw new Error('请选择目标数据库');
      }
      
      if (formData.sourceDbId === formData.targetDbId) {
        throw new Error('源数据库和目标数据库不能相同');
      }
      
      if (!useCustomStrategy && !formData.strategyId) {
        throw new Error('请选择迁移策略');
      }
      
      // 创建任务数据
      // 使用类型断言确保TypeScript接受strategy属性
      const taskData = {
        name: formData.name,
        sourceDbId: formData.sourceDbId,
        targetDbId: formData.targetDbId,
        strategyId: useCustomStrategy ? 'custom' : formData.strategyId,
        // 其他字段会在后端设置
      } as Omit<PipelineTask, 'id' | 'createdAt' | 'updatedAt' | 'progress' | 'status'> & { strategy?: typeof formData.customStrategy };
      
      // 如果使用自定义策略，需要将策略信息一起传递
      if (useCustomStrategy) {
        taskData['strategy'] = formData.customStrategy;
      }
      
      // 创建流水线任务
      await createPipelineTask(taskData);
      
      submitSuccess = true;
      
      // 重置表单
      formData = {
        name: '',
        sourceDbId: '',
        targetDbId: '',
        strategyId: '',
        customStrategy: {
          type: 'full',
          batchSize: 1000,
          retryCount: 3,
          timeout: 3600
        }
      };
      
      useCustomStrategy = false;
      
      // 3秒后重置成功状态
      setTimeout(() => {
        submitSuccess = false;
      }, 3000);
    } catch (error) {
      console.error('Failed to create pipeline task:', error);
      errorMessage = error instanceof Error ? error.message : '创建流水线任务失败';
    } finally {
      isSubmitting = false;
    }
  };
</script>

<div class="task-creation-form">
  <h2>创建数据库迁移任务</h2>
  
  {#if errorMessage}
    <div class="error-message">{errorMessage}</div>
  {/if}
  
  {#if submitSuccess}
    <div class="success-message">流水线任务创建成功！</div>
  {/if}
  
  <form on:submit={handleSubmit}>
    <div class="form-group">
      <label for="name">任务名称</label>
      <input
        id="name"
        type="text"
        bind:value={formData.name}
        placeholder="请输入任务名称"
        required
      />
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="sourceDbId">源数据库</label>
        <select
          id="sourceDbId"
          bind:value={formData.sourceDbId}
          required
        >
          <option value="">请选择源数据库</option>
          {#each $databases as database}
            <option value={database.id}>{database.name} ({database.type})</option>
          {/each}
        </select>
      </div>
      
      <div class="form-group">
        <label for="targetDbId">目标数据库</label>
        <select
          id="targetDbId"
          bind:value={formData.targetDbId}
          required
        >
          <option value="">请选择目标数据库</option>
          {#each $databases as database}
            <option value={database.id}>{database.name} ({database.type})</option>
          {/each}
        </select>
      </div>
    </div>
    
    <div class="form-group">
      <label>
        <input
          type="checkbox"
          bind:checked={useCustomStrategy}
        />
        使用自定义迁移策略
      </label>
    </div>
    
    {#if !useCustomStrategy}
      <div class="form-group">
        <label for="strategyId">选择迁移策略</label>
        <select
          id="strategyId"
          bind:value={formData.strategyId}
          required
        >
          <option value="">请选择迁移策略</option>
          {#each $strategies as strategy}
            <option value={strategy.id}>{strategy.name}</option>
          {/each}
        </select>
      </div>
    {:else}
      <div class="custom-strategy-section">
        <h3>自定义迁移策略</h3>
        
        <div class="form-group">
          <label for="strategyType">迁移类型</label>
          <select
            id="strategyType"
            bind:value={formData.customStrategy.type}
          >
            <option value="full">全量迁移</option>
            <option value="incremental">增量迁移</option>
            <option value="delta">差异迁移</option>
          </select>
        </div>
        
        <div class="form-group">
          <label for="batchSize">批处理大小</label>
          <input
            id="batchSize"
            type="number"
            bind:value={formData.customStrategy.batchSize}
            min="1"
            placeholder="批处理大小"
          />
        </div>
        
        <div class="form-group">
          <label for="retryCount">重试次数</label>
          <input
            id="retryCount"
            type="number"
            bind:value={formData.customStrategy.retryCount}
            min="0"
            placeholder="重试次数"
          />
        </div>
        
        <div class="form-group">
          <label for="timeout">超时时间（秒）</label>
          <input
            id="timeout"
            type="number"
            bind:value={formData.customStrategy.timeout}
            min="30"
            placeholder="超时时间"
          />
        </div>
      </div>
    {/if}
    
    <div class="form-actions">
      <button type="submit" disabled={isSubmitting}>
        {isSubmitting ? '创建中...' : '创建任务'}
      </button>
      <button type="button" class="cancel-button">取消</button>
    </div>
  </form>
</div>

<style>
  .task-creation-form {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }
  
  .task-creation-form h2 {
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
  
  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
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
  
  .custom-strategy-section {
    background-color: #f5f5f5;
    padding: 15px;
    border-radius: 4px;
  }
  
  .custom-strategy-section h3 {
    font-size: 1.2rem;
    margin-bottom: 15px;
    color: #333;
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