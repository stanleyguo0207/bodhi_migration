<script>
  import { DatabaseConfigForm } from "../../features/database";
  
  let showForm = true;
  
  function handleClose() {
    showForm = false;
  }
  
  function handleSaveSuccess() {
    console.log("Database saved successfully");
    showForm = false;
  }
</script>

<svelte:head>
  <title>Password Toggle Test</title>
</svelte:head>

<div class="test-container">
  <h1>密码显示/隐藏功能测试</h1>
  <p>测试DatabaseConfigForm中的密码输入框眼睛图标功能</p>
  
  {#if showForm}
    <DatabaseConfigForm 
      onClose={handleClose}
      onSaveSuccess={handleSaveSuccess}
    />
  {:else}
    <div class="success-message">
      <p>✅ 测试完成！</p>
      <button on:click={() => showForm = true}>重新打开表单</button>
    </div>
  {/if}
</div>

<style>
  .test-container {
    max-width: 800px;
    margin: 2rem auto;
    padding: 2rem;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", sans-serif;
  }
  
  h1 {
    color: #333;
    margin-bottom: 1rem;
  }
  
  p {
    color: #666;
    margin-bottom: 2rem;
  }
  
  .success-message {
    text-align: center;
    padding: 3rem;
    background: #f6ffed;
    border: 1px solid #d9f7be;
    border-radius: 12px;
    color: #34c759;
  }
  
  .success-message button {
    margin-top: 1rem;
    padding: 10px 20px;
    background: #007AFF;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s ease;
  }
  
  .success-message button:hover {
    background: #0051d5;
    transform: translateY(-1px);
  }
</style>