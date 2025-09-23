<script lang="ts">
  export let size: 'small' | 'medium' | 'large' = 'medium';
  export let color: string = 'var(--apple-primary)';
  export let text: string = '';
  export let style: 'rings' | 'dots' | 'wave' = 'rings';
  
  const sizeMap = {
    small: { width: '24px', height: '24px', dotSize: '4px' },
    medium: { width: '40px', height: '40px', dotSize: '6px' },
    large: { width: '56px', height: '56px', dotSize: '8px' }
  };
</script>

<div class="loading-container">
  {#if style === 'rings'}
    <div 
      class="loading-rings {size}"
      style="--spinner-color: {color}; --spinner-size: {sizeMap[size].width};"
    >
      <div class="ring"></div>
      <div class="ring"></div>
      <div class="ring"></div>
    </div>
  {:else if style === 'dots'}
    <div 
      class="loading-dots {size}"
      style="--spinner-color: {color}; --dot-size: {sizeMap[size].dotSize};"
    >
      <div class="dot"></div>
      <div class="dot"></div>
      <div class="dot"></div>
      <div class="dot"></div>
    </div>
  {:else if style === 'wave'}
    <div 
      class="loading-wave {size}"
      style="--spinner-color: {color}; --bar-width: 3px; --spinner-size: {sizeMap[size].width};"
    >
      <div class="bar"></div>
      <div class="bar"></div>
      <div class="bar"></div>
      <div class="bar"></div>
      <div class="bar"></div>
    </div>
  {/if}
  
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
    gap: 16px;
    background: transparent;
    padding: 20px;
  }

  /* Rings Animation - Modern Glass Effect */
  .loading-rings {
    position: relative;
    width: var(--spinner-size);
    height: var(--spinner-size);
    background: transparent;
  }

  .ring {
    position: absolute;
    width: 100%;
    height: 100%;
    border: 2px solid transparent;
    border-radius: 50%;
    animation: ringRotate 2s cubic-bezier(0.4, 0, 0.2, 1) infinite;
    backdrop-filter: blur(1px);
  }

  .ring:nth-child(1) {
    border-top-color: var(--spinner-color);
    border-right-color: var(--spinner-color);
    animation-delay: 0s;
  }

  .ring:nth-child(2) {
    width: 70%;
    height: 70%;
    top: 15%;
    left: 15%;
    border-left-color: var(--spinner-color);
    border-bottom-color: var(--spinner-color);
    animation-delay: -0.6s;
  }

  .ring:nth-child(3) {
    width: 40%;
    height: 40%;
    top: 30%;
    left: 30%;
    border-top-color: var(--spinner-color);
    border-left-color: var(--spinner-color);
    animation-delay: -1.2s;
  }

  @keyframes ringRotate {
    0% {
      transform: rotate(0deg) scale(1);
    }
    50% {
      transform: rotate(180deg) scale(1.1);
    }
    100% {
      transform: rotate(360deg) scale(1);
    }
  }

  /* Dots Animation - Fluid Motion */
  .loading-dots {
    display: flex;
    gap: 4px;
    align-items: center;
    height: var(--spinner-size);
  }

  .dot {
    width: var(--dot-size);
    height: var(--dot-size);
    background-color: var(--spinner-color);
    border-radius: 50%;
    animation: dotFloat 1.4s ease-in-out infinite both;
  }

  .dot:nth-child(1) {
    animation-delay: -0.32s;
  }

  .dot:nth-child(2) {
    animation-delay: -0.16s;
  }

  .dot:nth-child(3) {
    animation-delay: 0s;
  }

  .dot:nth-child(4) {
    animation-delay: 0.16s;
  }

  @keyframes dotFloat {
    0%, 80%, 100% {
      transform: scale(0.8);
      opacity: 0.5;
    }
    40% {
      transform: scale(1.2);
      opacity: 1;
    }
  }

  /* Wave Animation - Modern Minimal */
  .loading-wave {
    display: flex;
    gap: 2px;
    align-items: center;
    height: var(--spinner-size);
  }

  .bar {
    width: var(--bar-width);
    height: 100%;
    background-color: var(--spinner-color);
    border-radius: 2px;
    animation: wave 1.2s ease-in-out infinite;
  }

  .bar:nth-child(1) {
    animation-delay: -1.2s;
    height: 60%;
  }

  .bar:nth-child(2) {
    animation-delay: -1.0s;
    height: 80%;
  }

  .bar:nth-child(3) {
    animation-delay: -0.8s;
    height: 100%;
  }

  .bar:nth-child(4) {
    animation-delay: -0.6s;
    height: 80%;
  }

  .bar:nth-child(5) {
    animation-delay: -0.4s;
    height: 60%;
  }

  @keyframes wave {
    0%, 40%, 100% {
      transform: scaleY(0.4);
      opacity: 0.4;
    }
    20% {
      transform: scaleY(1);
      opacity: 1;
    }
  }

  .loading-text {
    font-family: var(--apple-font, -apple-system, BlinkMacSystemFont, "SF Pro", "Helvetica Neue", sans-serif);
    font-size: 14px;
    font-weight: 400;
    color: var(--apple-text-secondary, #6b7280);
    letter-spacing: -0.01em;
    animation: fadeInText 0.6s ease-out;
  }

  @keyframes fadeInText {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Size variations */
  .loading-rings.small .ring {
    border-width: 1.5px;
  }

  .loading-rings.large .ring {
    border-width: 3px;
  }

  .loading-dots.small {
    gap: 3px;
  }

  .loading-dots.large {
    gap: 6px;
  }

  .loading-wave.small {
    gap: 1.5px;
  }

  .loading-wave.large {
    gap: 3px;
  }

  /* Glass morphism effect for modern aesthetic */
  .loading-rings::before,
  .loading-dots::before,
  .loading-wave::before {
    content: '';
    position: absolute;
    top: -8px;
    left: -8px;
    right: -8px;
    bottom: -8px;
    border-radius: 50%;
    background: radial-gradient(circle, var(--spinner-color) 0%, transparent 60%);
    opacity: 0.1;
    animation: subtleGlow 3s ease-in-out infinite alternate;
    z-index: -1;
    filter: blur(8px);
  }

  .loading-dots::before,
  .loading-wave::before {
    border-radius: 20px;
  }

  @keyframes subtleGlow {
    0% {
      opacity: 0.05;
      transform: scale(0.9);
    }
    100% {
      opacity: 0.15;
      transform: scale(1.1);
    }
  }
</style>