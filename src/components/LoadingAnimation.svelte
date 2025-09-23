<script lang="ts">
  export let size: 'small' | 'medium' | 'large' = 'medium';
  export let color: string = 'var(--apple-primary)';
  export let text: string = '';
  
  const sizeMap = {
    small: { width: '20px', height: '20px' },
    medium: { width: '32px', height: '32px' },
    large: { width: '48px', height: '48px' }
  };
</script>

<div class="loading-container">
  <div 
    class="loading-spinner {size}"
    style="--spinner-color: {color}; --spinner-size: {sizeMap[size].width};"
  >
    <div class="spinner-ring"></div>
    <div class="spinner-ring"></div>
    <div class="spinner-ring"></div>
  </div>
  {#if text}
    <span class="loading-text">{text}</span>
  {/if}
</div>

<style>
  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    background: transparent;
  }

  .loading-spinner {
    position: relative;
    width: var(--spinner-size);
    height: var(--spinner-size);
    background: transparent;
  }

  .spinner-ring {
    position: absolute;
    width: 100%;
    height: 100%;
    border: 2px solid transparent;
    border-radius: 50%;
    animation: spin 1.5s cubic-bezier(0.68, -0.55, 0.265, 1.55) infinite;
  }

  .spinner-ring:nth-child(1) {
    border-top-color: var(--spinner-color);
    animation-delay: 0s;
  }

  .spinner-ring:nth-child(2) {
    border-right-color: var(--spinner-color);
    animation-delay: 0.2s;
    opacity: 0.8;
  }

  .spinner-ring:nth-child(3) {
    border-bottom-color: var(--spinner-color);
    animation-delay: 0.4s;
    opacity: 0.6;
  }

  .loading-spinner.small .spinner-ring {
    border-width: 1.5px;
  }

  .loading-spinner.medium .spinner-ring {
    border-width: 2px;
  }

  .loading-spinner.large .spinner-ring {
    border-width: 3px;
  }

  .loading-text {
    font-family: var(--apple-font, -apple-system, BlinkMacSystemFont, "SF Pro", "Helvetica Neue", sans-serif);
    font-size: 14px;
    font-weight: 400;
    color: var(--apple-text-secondary, #6b7280);
    letter-spacing: -0.01em;
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg) scale(1);
    }
    50% {
      transform: rotate(180deg) scale(1.05);
    }
    100% {
      transform: rotate(360deg) scale(1);
    }
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Alternative pulsing animation for subtle loading */
  .loading-spinner.pulse .spinner-ring {
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.1);
      opacity: 0.7;
    }
  }

  /* Glow effect for modern aesthetic */
  .loading-spinner::before {
    content: '';
    position: absolute;
    top: -4px;
    left: -4px;
    right: -4px;
    bottom: -4px;
    border-radius: 50%;
    background: radial-gradient(circle, var(--spinner-color) 0%, transparent 70%);
    opacity: 0.2;
    animation: glow 2s ease-in-out infinite alternate;
    z-index: -1;
  }

  @keyframes glow {
    0% {
      opacity: 0.1;
      transform: scale(0.8);
    }
    100% {
      opacity: 0.3;
      transform: scale(1.2);
    }
  }
</style>