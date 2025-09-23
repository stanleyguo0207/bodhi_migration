<script lang="ts">
  export let variant: 'primary' | 'secondary' | 'danger' | 'success' = 'primary';
  export let size: 'small' | 'medium' | 'large' = 'medium';
  export let disabled: boolean = false;
  export let loading: boolean = false;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let fullWidth: boolean = false;

  const variantClasses = {
    primary: 'btn-primary',
    secondary: 'btn-secondary', 
    danger: 'btn-danger',
    success: 'btn-success'
  };

  const sizeClasses = {
    small: 'btn-sm',
    medium: 'btn-md',
    large: 'btn-lg'
  };

  $: classes = [
    'btn',
    variantClasses[variant],
    sizeClasses[size],
    fullWidth ? 'btn-full-width' : '',
    loading ? 'btn-loading' : ''
  ].join(' ');
</script>

<button
  {type}
  class={classes}
  {disabled}
  on:click
>
  {#if loading}
    <span class="btn-spinner"></span>
  {/if}
  <span class="btn-content" class:btn-content-loading={loading}>
    <slot />
  </span>
</button>

<style>
  .btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    outline: none;
    user-select: none;
  }

  .btn:focus {
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.3);
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary {
    background-color: #007AFF;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background-color: #0051D5;
  }

  .btn-secondary {
    background-color: #E5E5EA;
    color: #333;
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: #D1D1D6;
  }

  .btn-danger {
    background-color: #FF3B30;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background-color: #D70015;
  }

  .btn-success {
    background-color: #34C759;
    color: white;
  }

  .btn-success:hover:not(:disabled) {
    background-color: #248A3D;
  }

  .btn-sm {
    padding: 6px 12px;
    font-size: 14px;
  }

  .btn-md {
    padding: 10px 16px;
    font-size: 16px;
  }

  .btn-lg {
    padding: 14px 24px;
    font-size: 18px;
  }

  .btn-full-width {
    width: 100%;
  }

  .btn-loading {
    pointer-events: none;
  }

  .btn-spinner {
    position: absolute;
    width: 16px;
    height: 16px;
    border: 2px solid transparent;
    border-top: 2px solid currentColor;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .btn-content {
    transition: opacity 0.2s ease;
  }

  .btn-content-loading {
    opacity: 0.7;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>