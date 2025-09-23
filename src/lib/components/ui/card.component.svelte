<script lang="ts">
  export let title: string = '';
  export let subtitle: string = '';
  export let padding: 'none' | 'small' | 'medium' | 'large' = 'medium';
  export let shadow: boolean = true;
  export let hover: boolean = false;

  const paddingClasses = {
    none: 'card-padding-none',
    small: 'card-padding-sm',
    medium: 'card-padding-md',
    large: 'card-padding-lg'
  };
</script>

<div 
  class="card {paddingClasses[padding]}"
  class:card-shadow={shadow}
  class:card-hover={hover}
>
  {#if title || subtitle}
    <div class="card-header">
      {#if title}
        <h3 class="card-title">{title}</h3>
      {/if}
      {#if subtitle}
        <p class="card-subtitle">{subtitle}</p>
      {/if}
    </div>
  {/if}
  
  <div class="card-content">
    <slot />
  </div>
</div>

<style>
  .card {
    background-color: white;
    border-radius: 12px;
    border: 1px solid #E5E5EA;
    overflow: hidden;
    transition: all 0.2s ease;
  }

  .card-shadow {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  }

  .card-hover:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  }

  .card-padding-none {
    padding: 0;
  }

  .card-padding-sm {
    padding: 12px;
  }

  .card-padding-md {
    padding: 16px;
  }

  .card-padding-lg {
    padding: 24px;
  }

  .card-header {
    padding: 16px 16px 0;
    border-bottom: 1px solid #F2F2F7;
    margin-bottom: 16px;
  }

  .card-title {
    margin: 0 0 4px 0;
    font-size: 18px;
    font-weight: 600;
    color: #333;
  }

  .card-subtitle {
    margin: 0;
    font-size: 14px;
    color: #666;
  }

  .card-content {
    padding: 0 16px 16px;
  }

  .card-padding-none .card-header {
    padding: 0;
    border-bottom: none;
    margin-bottom: 0;
  }

  .card-padding-none .card-content {
    padding: 0;
  }
</style>