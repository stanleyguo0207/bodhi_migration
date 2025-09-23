<script lang="ts">
  import { onMount } from 'svelte';
  import { loadingManager, type LoadingConfig } from '../utils/loadingUtils';
  import LoadingAnimation from './LoadingAnimation.svelte';
  import ModernLoadingAnimation from './ModernLoadingAnimation.svelte';
  
  export let config: Partial<LoadingConfig> = {};
  export let type: 'basic' | 'modern' = 'modern';
  export let overlay: boolean = false;
  export let fullscreen: boolean = false;
  
  let loadingState = { isLoading: false, config: config };
  let unsubscribe: (() => void) | null = null;
  
  onMount(() => {
    // Subscribe to loading manager state
    unsubscribe = loadingManager.subscribe((isLoading, managerConfig) => {
      loadingState = { isLoading, config: { ...managerConfig, ...config } };
    });
    
    return () => {
      if (unsubscribe) {
        unsubscribe();
      }
    };
  });
  
  // Determine if we should show loading based on props or manager state
  $: shouldShowLoading = loadingState.isLoading || config.text || config.size;
</script>

{#if shouldShowLoading}
  <div 
    class="loading-wrapper"
    class:overlay={overlay}
    class:fullscreen={fullscreen}
  >
    <div class="loading-content">
      {#if type === 'basic'}
        <LoadingAnimation
          size={loadingState.config.size || 'medium'}
          color={loadingState.config.color || 'var(--apple-primary)'}
          text={loadingState.config.text || ''}
        />
      {:else}
        <ModernLoadingAnimation
          size={loadingState.config.size || 'medium'}
          color={loadingState.config.color || 'var(--apple-primary)'}
          text={loadingState.config.text || ''}
          style={loadingState.config.style || 'rings'}
        />
      {/if}
    </div>
  </div>
{/if}

<style>
  .loading-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
  }

  .loading-content {
    position: relative;
    z-index: 1;
  }

  .loading-wrapper.overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    z-index: 100;
  }

  .loading-wrapper.fullscreen {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(245, 245, 247, 0.95);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    z-index: 9999;
  }

  .loading-wrapper.fullscreen .loading-content {
    transform: scale(1.5);
  }

  /* Subtle animation for wrapper appearance */
  .loading-wrapper {
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>