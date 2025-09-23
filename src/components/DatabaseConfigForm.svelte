<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type DatabaseConfig, DatabaseType } from "../types/database";
  import { saveDatabaseConfig } from "../stores/appStore";

  // Props
  export let databaseId: string | null = null;
  export let onClose: () => void;
  export let onSaveSuccess: () => void;

  // 表单数据
  let formData: DatabaseConfig = {
    id: "",
    name: "",
    type: DatabaseType.Redis,
    host: "localhost",
    port: 6379,
    username: "",
    password: "",
    database: "",
    ssl: false,
    extra: {} as Record<string, string>,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  // 表单状态
  let isSubmitting = false;
  let submitSuccess = false;
  let errorMessage = "";

  // 表单验证错误
  let formErrors: Record<string, string> = {};

  // 辅助变量用于Redis数据库索引
  let redisDbIndex: string = "0";

  // 连接测试状态
  let isTestingConnection = false;
  let connectionTestResult: "success" | "error" | null = null;
  let connectionTestMessage = "";

  // 当编辑现有数据库时，加载数据库配置
  onMount(async () => {
    if (databaseId) {
      try {
        // 通过Tauri调用后端API获取所有数据库配置，然后本地过滤
        const allConfigs = await invoke("get_all_database_configs_from_db");
        const configs = allConfigs as DatabaseConfig[];
        const config = configs.find((c: DatabaseConfig) => c.id === databaseId);
        
        if (config) {
          formData = { ...config };
          // 确保createdAt字段存在
          if (!formData.createdAt) {
            formData.createdAt = new Date().toISOString();
          }
          // 初始化Redis数据库索引
          if (formData.type === DatabaseType.Redis && formData.extra?.["db"]) {
            redisDbIndex = formData.extra["db"];
          }
        } else {
          console.error("Database config not found for ID:", databaseId);
          errorMessage = "未找到指定的数据库配置";
        }
      } catch (error) {
        console.error("Failed to load database config:", error);
        errorMessage = "加载数据库配置失败";
      }
    } else {
      // 生成新的ID
      formData.id = `db_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      // 确保createdAt字段被设置
      formData.createdAt = new Date().toISOString();
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
      default:
        break;
    }

    // 清除相关错误
    clearFormErrors();
  };

  // 清除表单错误
  const clearFormErrors = () => {
    formErrors = {};
    errorMessage = "";
  };

  // 表单验证
  const validateForm = (): boolean => {
    const errors: Record<string, string> = {};

    // 验证必填字段
    if (!formData.name.trim()) {
      errors.name = "数据库名称不能为空";
    }

    if (!formData.host?.trim()) {
      errors.host = "主机地址不能为空";
    }

    if (!formData.port) {
      errors.port = "端口号不能为空";
    } else if (isNaN(Number(formData.port))) {
      errors.port = "端口号必须是数字";
    }

    if (!formData.username?.trim()) {
      errors.username = "用户名不能为空";
    }

    if (formData.type !== DatabaseType.Redis && !formData.database?.trim()) {
      errors.database = "数据库名称不能为空";
    }

    if (formData.ssl) {
      // 这里可以添加SSL相关的验证
    }

    formErrors = errors;
    return Object.keys(errors).length === 0;
  };

  // 表单提交处理
  const handleSubmit = async (event: Event) => {
    event.preventDefault();

    if (isSubmitting) return;

    // 验证表单
    if (!validateForm()) {
      return;
    }

    isSubmitting = true;
    errorMessage = "";

    try {
      // 记录当前时间戳信息
      const originalCreatedAt = formData.createdAt;
      console.log("提交前的创建时间:", originalCreatedAt);
      console.log("是否为编辑模式:", !!databaseId);
      
      // 更新时间戳 - 只在编辑时更新updatedAt，保持createdAt不变
      formData.updatedAt = new Date().toISOString();
      
      // 确保createdAt字段在编辑模式下不被修改
      if (databaseId && formData.createdAt) {
        // 保持原有的createdAt时间戳
        console.log("保持原有的创建时间:", formData.createdAt);
      } else if (!databaseId) {
        // 新配置时设置createdAt
        formData.createdAt = new Date().toISOString();
        console.log("新配置，设置创建时间:", formData.createdAt);
      }
      
      console.log("准备保存的配置数据:", {
        id: formData.id,
        name: formData.name,
        createdAt: formData.createdAt,
        updatedAt: formData.updatedAt,
        isEditMode: !!databaseId
      });

      // 保存数据库配置
      await saveDatabaseConfig(formData);
      
      console.log("数据库配置保存成功");

      submitSuccess = true;

      // 调用成功回调函数进行跳转
      onSaveSuccess();

      // 3秒后重置成功状态
      setTimeout(() => {
        submitSuccess = false;
      }, 3000);
    } catch (error) {
      console.error("Failed to save database config:", error);
      errorMessage =
        error instanceof Error ? error.message : "保存数据库配置失败";
    } finally {
      isSubmitting = false;
    }
  };

  // 测试连接
  const testConnection = async () => {
    // 验证必要的字段
    if (!validateForm()) {
      return;
    }

    isTestingConnection = true;
    connectionTestResult = null;
    connectionTestMessage = "";

    try {
      // 模拟连接测试
      // 实际项目中应该调用API测试连接
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // 模拟连接结果（80%的概率成功）
      const isSuccess = Math.random() > 0.2;

      if (isSuccess) {
        connectionTestResult = "success";
        connectionTestMessage = "连接成功！";
      } else {
        connectionTestResult = "error";
        connectionTestMessage = "连接失败，请检查配置";
      }
    } catch (error) {
      console.error("Connection test failed:", error);
      connectionTestResult = "error";
      connectionTestMessage =
        error instanceof Error ? error.message : "连接测试失败";
    } finally {
      isTestingConnection = false;

      // 3秒后重置连接测试结果
      setTimeout(() => {
        connectionTestResult = null;
        connectionTestMessage = "";
      }, 3000);
    }
  };

  // 取消操作
  const handleCancel = () => {
    onClose();
  };
</script>

<div class="database-config-form-container">
  <div class="form-header">
    <div class="header-content">
      <div class="form-icon" style="background-color: #007AFF20; color: #007AFF">
        {#if databaseId}
          ✏️
        {:else}
          ➕
        {/if}
      </div>
      <h2>{databaseId ? "编辑数据库" : "添加数据库"}</h2>
    </div>
    <button class="close-button" on:click={handleCancel}>&times;</button>
  </div>

  {#if errorMessage}
    <div class="global-error-message">{errorMessage}</div>
  {/if}

  {#if submitSuccess}
    <div class="global-success-message">数据库配置保存成功！</div>
  {/if}

  <form on:submit={handleSubmit} class="database-config-form">
    <div class="form-section">
      <h3>基本信息</h3>

      <div class="form-row">
        <div class="detail-card form-group">
        <label for="name">数据库名称 <span class="required">*</span></label>
        <input
          type="text"
          id="name"
          bind:value={formData.name}
          placeholder="请输入数据库名称"
          class={formErrors.name ? "error" : ""}
          on:input={() => delete formErrors.name}
        />
        {#if formErrors.name}
          <span class="error-message">{formErrors.name}</span>
        {/if}
      </div>

      <div class="detail-card form-group">
        <label for="type">数据库类型 <span class="required">*</span></label>
        <select
          id="type"
          bind:value={formData.type}
          class={formErrors.type ? "error" : ""}
          on:change={(e) =>
            handleTypeChange(e.currentTarget.value as DatabaseType)}
        >
          <option value={DatabaseType.Redis} selected>Redis</option>
          <option value={DatabaseType.MySQL} disabled>MySQL (即将支持)</option
          >
          <option value={DatabaseType.PostgreSQL} disabled
            >PostgreSQL (即将支持)</option
          >
        </select>
        {#if formErrors.type}
          <span class="error-message">{formErrors.type}</span>
        {/if}
      </div>
      </div>
    </div>

    <div class="form-section">
      <h3>连接信息</h3>

      <div class="detail-grid">
        <div class="detail-card form-group">
          <label for="host">主机地址 <span class="required">*</span></label>
          <input
            type="text"
            id="host"
            bind:value={formData.host}
            placeholder="请输入主机地址"
            class={formErrors.host ? "error" : ""}
            on:input={() => delete formErrors.host}
          />
          {#if formErrors.host}
            <span class="error-message">{formErrors.host}</span>
          {/if}
        </div>

        <div class="detail-card form-group">
          <label for="port">端口号 <span class="required">*</span></label>
          <input
            type="number"
            id="port"
            bind:value={formData.port}
            min="1"
            max="65535"
            placeholder="请输入端口号"
            class={formErrors.port ? "error" : ""}
            on:input={() => delete formErrors.port}
          />
          {#if formErrors.port}
            <span class="error-message">{formErrors.port}</span>
          {/if}
        </div>
      </div>

      {#if formData.type !== DatabaseType.Redis}
        <div class="detail-grid">
          <div class="detail-card form-group">
            <label for="database"
              >数据库名称 <span class="required">*</span></label
            >
            <input
              type="text"
              id="database"
              bind:value={formData.database}
              placeholder="请输入数据库名称"
              class={formErrors.database ? "error" : ""}
              on:input={() => delete formErrors.database}
            />
            {#if formErrors.database}
              <span class="error-message">{formErrors.database}</span>
            {/if}
          </div>
        </div>
      {/if}

      <div class="detail-grid">
        <div class="detail-card form-group">
          <label for="username">用户名 <span class="required">*</span></label>
          <input
            type="text"
            id="username"
            bind:value={formData.username}
            placeholder="请输入用户名"
            class={formErrors.username ? "error" : ""}
            on:input={() => delete formErrors.username}
          />
          {#if formErrors.username}
            <span class="error-message">{formErrors.username}</span>
          {/if}
        </div>

        <div class="detail-card form-group">
          <label for="password">密码</label>
          <input
            type="password"
            id="password"
            bind:value={formData.password}
            placeholder="请输入密码"
          />
        </div>
      </div>
    </div>

    <div class="form-section">
      <div class="checkbox-group">
        <input type="checkbox" id="ssl" bind:checked={formData.ssl} />
        <label for="ssl">启用SSL连接</label>
      </div>

      {#if formData.ssl}
        <div class="ssl-config">
          <div class="detail-card form-group">
            <label for="ssl-cert">SSL证书</label>
            <textarea
              id="ssl-cert"
              placeholder="请输入SSL证书内容"
              rows="4"
              class="monospace"
            ></textarea>
          </div>

          <div class="detail-card form-group">
            <label for="ssl-key">SSL密钥</label>
            <textarea
              id="ssl-key"
              placeholder="请输入SSL密钥内容"
              rows="4"
              class="monospace"
            ></textarea>
          </div>
        </div>
      {/if}
    </div>

    {#if formData.type === DatabaseType.Redis}
      <div class="form-section">
        <h3>Redis配置</h3>
        <div class="detail-grid">
          <div class="detail-card form-group">
            <label for="redis-db">数据库索引</label>
            <input
              id="redis-db"
              type="number"
              min="0"
              max="15"
              bind:value={redisDbIndex}
              placeholder="请输入Redis数据库索引（0-15）"
              on:change={(e) => {
                (formData.extra ??= {})["db"] = e.currentTarget.value;
                redisDbIndex = e.currentTarget.value;
              }}
            />
          </div>
        </div>
      </div>
    {/if}

    <div class="form-actions">
      <div class="connection-test">
        <button
          type="button"
          class="test-button"
          on:click={testConnection}
          disabled={isTestingConnection}
        >
          {isTestingConnection ? "测试中..." : "测试连接"}
        </button>
        {#if connectionTestResult}
          <span class={`connection-result ${connectionTestResult}`}>
            {connectionTestMessage}
          </span>
        {/if}
      </div>

      <div class="submit-buttons">
        <button type="button" class="cancel-button" on:click={handleCancel}
          >取消</button
        >
        <button type="submit" class="submit-button" disabled={isSubmitting}>
          {isSubmitting ? "提交中..." : databaseId ? "更新" : "添加"}
        </button>
      </div>
    </div>
  </form>
</div>

<style>
  .database-config-form-container {
    background: var(--apple-background-secondary, #ffffff);
    border-radius: var(--apple-border-radius-large, 14px);
    box-shadow: 0 25px 50px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(0, 0, 0, 0.05);
    max-width: 600px;
    margin: 0 auto;
    position: relative;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "SF Pro Text", sans-serif;
    overflow: hidden;
    animation: modalSlideIn 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes modalSlideIn {
    from {
      opacity: 0;
      transform: translateY(30px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .form-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 24px 32px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    background: linear-gradient(135deg, #f8f9fa 0%, #ffffff 100%);
  }

  .header-content {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .form-icon {
    width: 56px;
    height: 56px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 28px;
    flex-shrink: 0;
  }

  .form-header h2 {
    margin: 0;
    font-size: 24px;
    font-weight: 700;
    color: #000;
    letter-spacing: -0.5px;
  }

  .close-button {
    background: rgba(0, 0, 0, 0.05);
    border: none;
    font-size: 24px;
    color: #8e8e93;
    cursor: pointer;
    padding: 0;
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    font-family: inherit;
    flex-shrink: 0;
  }

  .close-button:hover {
    background: rgba(0, 0, 0, 0.1);
    color: #000;
    transform: scale(1.05);
  }

  .global-error-message {
    background-color: #fff2f0;
    color: #ff3b30;
    padding: 16px 32px;
    border-left: 4px solid #ff3b30;
    margin-bottom: 0;
    font-weight: 500;
  }

  .global-success-message {
    background-color: #f6ffed;
    color: #34c759;
    padding: 16px 32px;
    border-left: 4px solid #34c759;
    margin-bottom: 0;
    font-weight: 500;
  }

  .database-config-form {
    padding: 32px;
    overflow-y: auto;
    max-height: calc(85vh - 140px);
  }

  .form-section {
    margin-bottom: 32px;
  }

  .form-section h3 {
    font-size: 16px;
    font-weight: 600;
    color: #000;
    margin: 0 0 20px 0;
    padding-bottom: 0;
    border-bottom: none;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .form-section h3::before {
    content: '';
    width: 3px;
    height: 16px;
    background: #007AFF;
    border-radius: 2px;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 16px;
  }

  .detail-card {
    background: #f8f9fa;
    border-radius: 12px;
    padding: 20px;
    border: 1px solid #e5e5ea;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .detail-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
  }

  .form-group {
    margin-bottom: 0;
  }

  .form-group label {
    display: block;
    font-size: 14px;
    color: #000;
    margin-bottom: 8px;
    font-weight: 500;
  }

  .required {
    color: #f5222d;
  }

  .form-group input,
  .form-group select,
  .form-group textarea {
    width: 100%;
    padding: 12px 16px;
    border: 1px solid #e5e5ea;
    border-radius: var(--apple-border-radius-medium, 10px);
    font-size: 14px;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    box-sizing: border-box;
    font-family: inherit;
    background: #ffffff;
  }

  .form-group input:focus,
  .form-group select:focus,
  .form-group textarea:focus {
    outline: none;
    border-color: #007AFF;
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.15);
    transform: translateY(-1px);
  }

  .form-group input.error,
  .form-group select.error,

  .form-group input.error:focus,
  .form-group select.error:focus,

  .error-message {
    display: block;
    font-size: 12px;
    color: #ff3b30;
    margin-top: 6px;
    font-weight: 500;
  }

  .file-input-container {
    display: flex;
    gap: 8px;
  }

  .browse-button {
    background-color: #f2f2f7;
    border: 1px solid #e5e5ea;
    border-radius: var(--apple-border-radius-medium, 10px);
    padding: 10px 16px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    white-space: nowrap;
    font-weight: 500;
  }

  .browse-button:hover {
    background-color: #e5e5ea;
    border-color: #007AFF;
    color: #007AFF;
    transform: translateY(-1px);
  }

  .checkbox-group {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
    background: #f8f9fa;
    padding: 16px;
    border-radius: var(--apple-border-radius-medium, 10px);
    border: 1px solid #e5e5ea;
  }

  .checkbox-group input[type="checkbox"] {
    width: 18px;
    height: 18px;
    margin-right: 12px;
    accent-color: #007AFF;
  }

  .checkbox-group label {
    margin: 0;
    font-weight: 500;
    color: #000;
  }

  .ssl-config {
    background-color: #f8f9fa;
    padding: 20px;
    border-radius: var(--apple-border-radius-medium, 10px);
    margin-top: 12px;
    border: 1px solid #e5e5ea;
  }

  .monospace {
    font-family: "Consolas", "Monaco", "Courier New", monospace;
  }

  .form-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 32px;
    border-top: 1px solid rgba(0, 0, 0, 0.05);
    margin-top: 8px;
  }

  .connection-test {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .test-button {
    background-color: #34c759;
    color: white;
    border: none;
    border-radius: var(--apple-border-radius-medium, 10px);
    padding: 10px 16px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    font-weight: 500;
  }

  .test-button:hover:not(:disabled) {
    background-color: #2fb94f;
    transform: translateY(-1px);
  }

  .test-button:disabled {
    background-color: #e5e5ea;
    cursor: not-allowed;
    color: #8e8e93;
  }

  .connection-result {
    font-size: 14px;
    padding: 8px 12px;
    border-radius: var(--apple-border-radius-small, 6px);
    border: 1px solid transparent;
    font-weight: 500;
  }

  .connection-result.success {
    background-color: #f6ffed;
    color: #34c759;
    border-color: #d9f7be;
  }

  .connection-result.error {
    background-color: #fff2f0;
    color: #ff3b30;
    border-color: #ffccc7;
  }

  .submit-buttons {
    display: flex;
    gap: 12px;
  }

  /* Add subtle loading animation */
  @keyframes pulse {
    0% { opacity: 1; }
    50% { opacity: 0.7; }
    100% { opacity: 1; }
  }

  .submit-button:disabled {
    animation: pulse 1.5s ease-in-out infinite;
  }

  .cancel-button {
    background-color: white;
    color: #8e8e93;
    border: 1px solid #e5e5ea;
    border-radius: var(--apple-border-radius-medium, 10px);
    padding: 10px 20px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    font-weight: 500;
  }

  .cancel-button:hover {
    border-color: #007AFF;
    color: #007AFF;
    transform: translateY(-1px);
  }

  .submit-button {
    background-color: #007AFF;
    color: white;
    border: none;
    border-radius: var(--apple-border-radius-medium, 10px);
    padding: 10px 20px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    font-weight: 500;
  }

  .submit-button:hover:not(:disabled) {
    background-color: #0051d5;
    transform: translateY(-1px);
  }

  .submit-button:disabled {
    background-color: #e5e5ea;
    cursor: not-allowed;
    color: #8e8e93;
  }

  /* 响应式布局 */
  @media (max-width: 768px) {
    .database-config-form-container {
      margin: 0 16px;
    }

    .form-row {
      grid-template-columns: 1fr;
    }

    .form-actions {
      flex-direction: column;
      align-items: stretch;
      gap: 16px;
    }

    .connection-test {
      justify-content: center;
    }

    .submit-buttons {
      justify-content: center;
    }
  }
</style>
