<script lang="ts">
  export let type: 'text' | 'password' | 'email' | 'number' | 'tel' | 'url' = 'text';
  export let value: string = '';
  export let placeholder: string = '';
  export let label: string = '';
  export let error: string = '';
  export let disabled: boolean = false;
  export let required: boolean = false;
  export let id: string = '';

  let showPassword = false;
  $: inputType = type === 'password' && showPassword ? 'text' : type;

  function togglePassword() {
    showPassword = !showPassword;
  }
</script>

<div class="input-group">
  {#if label}
    <label for={id} class="input-label">
      {label}
      {#if required}
        <span class="required">*</span>
      {/if}
    </label>
  {/if}
  
  <div class="input-container">
    <input
      {id}
      {type}
      bind:value
      {placeholder}
      {disabled}
      {required}
      class="input"
      class:input-error={error}
      on:input
      on:change
      on:focus
      on:blur
    />
    
    {#if type === 'password'}
      <button
        type="button"
        class="password-toggle"
        on:click={togglePassword}
        aria-label={showPassword ? '隐藏密码' : '显示密码'}
      >
        {showPassword ? '🙈' : '👁️'}
      </button>
    {/if}
  </div>
  
  {#if error}
    <span class="error-message">{error}</span>
  {/if}
</div>

<style>
  .input-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .input-label {
    font-size: 14px;
    font-weight: 500;
    color: #333;
  }

  .required {
    color: #FF3B30;
  }

  .input-container {
    position: relative;
    display: flex;
    align-items: center;
  }

  .input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #C7C7CC;
    border-radius: 8px;
    font-size: 16px;
    background-color: white;
    transition: all 0.2s ease;
    outline: none;
  }

  .input:focus {
    border-color: #007AFF;
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
  }

  .input:hover:not(:disabled) {
    border-color: #007AFF;
  }

  .input:disabled {
    background-color: #F2F2F7;
    cursor: not-allowed;
  }

  .input-error {
    border-color: #FF3B30;
  }

  .input-error:focus {
    box-shadow: 0 0 0 3px rgba(255, 59, 48, 0.1);
  }

  .password-toggle {
    position: absolute;
    right: 8px;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 16px;
    padding: 4px;
    border-radius: 4px;
    transition: background-color 0.2s ease;
  }

  .password-toggle:hover {
    background-color: rgba(0, 0, 0, 0.05);
  }

  .error-message {
    font-size: 12px;
    color: #FF3B30;
  }
</style>