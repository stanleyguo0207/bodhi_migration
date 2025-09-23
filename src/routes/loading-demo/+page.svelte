<script lang="ts">
  import { LoadingAnimation, ModernLoadingAnimation } from '../../features/loading';
  
  let selectedStyle = 'rings';
  let selectedSize = 'medium';
  let customText = '加载中...';
  let customColor = '#007aff';
  let showText = true;
</script>

<svelte:head>
  <title>Loading Animations - Demo</title>
</svelte:head>

<div class="demo-container">
  <div class="header">
    <h1>Loading Animations Demo</h1>
    <p>Modern, transparent loading animations with Apple-style design</p>
  </div>

  <div class="controls">
    <div class="control-group">
      <label for="animation-style">Animation Style:</label>
      <select id="animation-style" bind:value={selectedStyle}>
        <option value="rings">Rings</option>
        <option value="dots">Dots</option>
        <option value="wave">Wave</option>
      </select>
    </div>

    <div class="control-group">
      <label for="size-select">Size:</label>
      <select id="size-select" bind:value={selectedSize}>
        <option value="small">Small</option>
        <option value="medium">Medium</option>
        <option value="large">Large</option>
      </select>
    </div>

    <div class="control-group">
      <label for="color-picker">Color:</label>
      <input id="color-picker" type="color" bind:value={customColor} />
    </div>

    <div class="control-group">
      <label>
        <input type="checkbox" bind:checked={showText} />
        Show Text
      </label>
    </div>

    {#if showText}
      <div class="control-group">
        <label for="loading-text">Text:</label>
        <input id="loading-text" type="text" bind:value={customText} placeholder="Loading text..." />
      </div>
    {/if}
  </div>

  <div class="preview-section">
    <div class="preview-card">
      <h3>Basic Loading Animation</h3>
      <div class="animation-container">
        <LoadingAnimation 
          size={selectedSize as "small" | "medium" | "large"}
          color={customColor}
          text={showText ? customText : ''}
        />
      </div>
    </div>

    <div class="preview-card">
      <h3>Modern Loading Animation</h3>
      <div class="animation-container">
        <ModernLoadingAnimation 
          size={selectedSize as "small" | "medium" | "large"}
          color={customColor}
          text={showText ? customText : ''}
          style={selectedStyle as "rings" | "dots" | "wave"}
        />
      </div>
    </div>
  </div>

  <div class="examples-section">
    <h2>Usage Examples</h2>
    
    <div class="example-grid">
      <div class="example-card">
        <h4>Button Loading</h4>
        <div class="button-example">
          <button class="demo-button">
            <LoadingAnimation size="small" color="#ffffff" />
            Processing...
          </button>
        </div>
      </div>

      <div class="example-card">
        <h4>Card Loading</h4>
        <div class="card-example">
          <ModernLoadingAnimation size="medium" color="#007aff" text="Loading data..." />
        </div>
      </div>

      <div class="example-card">
        <h4>Full Screen Loading</h4>
        <div class="fullscreen-example">
          <LoadingAnimation size="large" color="#5856d6" text="Initializing application..." />
        </div>
      </div>

      <div class="example-card">
        <h4>Inline Loading</h4>
        <div class="inline-example">
          <span>Please wait</span>
          <LoadingAnimation size="small" color="#34c759" />
        </div>
      </div>
    </div>
  </div>

  <div class="code-section">
    <h2>Implementation Code</h2>
    <div class="code-block">
      <h4>Basic Usage:</h4>
      <pre><code>&lt;LoadingAnimation size="medium" color="#007aff" text="Loading..." /&gt;</code></pre>
      
      <h4>Modern Animation:</h4>
      <pre><code>&lt;ModernLoadingAnimation 
  size="large" 
  color="#5856d6" 
  text="Processing..."
  style="rings" /&gt;</code></pre>
    </div>
  </div>
</div>

<style>
  :global(:root) {
    --apple-font: -apple-system, BlinkMacSystemFont, "SF Pro", "Helvetica Neue", sans-serif;
    --apple-primary: #007aff;
    --apple-secondary: #5856d6;
    --apple-success: #34c759;
    --apple-warning: #ff9500;
    --apple-danger: #ff3b30;
    --apple-background: #f5f5f7;
    --apple-card-bg: #ffffff;
    --apple-text-primary: #111827;
    --apple-text-secondary: #6b7280;
    --apple-text-tertiary: #9ca3af;
    --apple-border-radius: 12px;
    --apple-border-width: 1px;
    --apple-border-color: #e5e7eb;
    --apple-shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
    --apple-shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .demo-container {
    min-height: 100vh;
    background: linear-gradient(135deg, var(--apple-background) 0%, #ffffff 100%);
    padding: 40px 20px;
    font-family: var(--apple-font);
  }

  .header {
    text-align: center;
    margin-bottom: 40px;
  }

  .header h1 {
    font-size: 36px;
    font-weight: 700;
    color: var(--apple-text-primary);
    margin-bottom: 8px;
    letter-spacing: -0.02em;
  }

  .header p {
    font-size: 18px;
    color: var(--apple-text-secondary);
    font-weight: 400;
  }

  .controls {
    display: flex;
    flex-wrap: wrap;
    gap: 20px;
    justify-content: center;
    margin-bottom: 40px;
    padding: 24px;
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    box-shadow: var(--apple-shadow-sm);
    border: var(--apple-border-width) solid var(--apple-border-color);
    max-width: 800px;
    margin-left: auto;
    margin-right: auto;
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 120px;
  }

  .control-group label {
    font-size: 14px;
    font-weight: 600;
    color: var(--apple-text-primary);
  }

  .control-group select,
  .control-group input[type="text"],
  .control-group input[type="color"] {
    padding: 8px 12px;
    border: var(--apple-border-width) solid var(--apple-border-color);
    border-radius: 8px;
    font-size: 14px;
    font-family: var(--apple-font);
    background: var(--apple-card-bg);
    transition: all 0.2s ease;
  }

  .control-group select:focus,
  .control-group input:focus {
    outline: none;
    border-color: var(--apple-primary);
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
  }

  .control-group input[type="checkbox"] {
    margin-right: 8px;
  }

  .preview-section {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 24px;
    margin-bottom: 40px;
    max-width: 1000px;
    margin-left: auto;
    margin-right: auto;
  }

  .preview-card {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 32px;
    box-shadow: var(--apple-shadow-md);
    border: var(--apple-border-width) solid var(--apple-border-color);
    text-align: center;
    transition: transform 0.2s ease;
  }

  .preview-card:hover {
    transform: translateY(-2px);
  }

  .preview-card h3 {
    font-size: 20px;
    font-weight: 600;
    color: var(--apple-text-primary);
    margin-bottom: 24px;
  }

  .animation-container {
    min-height: 120px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(245, 245, 247, 0.5);
    border-radius: 12px;
    border: 1px dashed var(--apple-border-color);
  }

  .examples-section {
    max-width: 1200px;
    margin-left: auto;
    margin-right: auto;
    margin-bottom: 40px;
  }

  .examples-section h2 {
    font-size: 28px;
    font-weight: 700;
    color: var(--apple-text-primary);
    text-align: center;
    margin-bottom: 32px;
  }

  .example-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 24px;
  }

  .example-card {
    background: var(--apple-card-bg);
    border-radius: var(--apple-border-radius);
    padding: 24px;
    box-shadow: var(--apple-shadow-sm);
    border: var(--apple-border-width) solid var(--apple-border-color);
    text-align: center;
  }

  .example-card h4 {
    font-size: 16px;
    font-weight: 600;
    color: var(--apple-text-primary);
    margin-bottom: 20px;
  }

  .button-example {
    display: flex;
    justify-content: center;
  }

  .demo-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 24px;
    background: var(--apple-primary);
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .demo-button:hover {
    background: #0056b3;
    transform: translateY(-1px);
  }

  .card-example {
    padding: 20px;
    background: rgba(0, 122, 255, 0.05);
    border-radius: 8px;
    border: 1px solid rgba(0, 122, 255, 0.1);
  }

  .fullscreen-example {
    padding: 40px;
    background: linear-gradient(135deg, rgba(88, 86, 214, 0.1) 0%, rgba(0, 122, 255, 0.1) 100%);
    border-radius: 12px;
    border: 1px solid rgba(88, 86, 214, 0.2);
  }

  .inline-example {
    display: flex;
    align-items: center;
    gap: 8px;
    justify-content: center;
    padding: 16px;
    background: rgba(52, 199, 89, 0.05);
    border-radius: 8px;
    border: 1px solid rgba(52, 199, 89, 0.1);
  }

  .inline-example span {
    font-size: 14px;
    color: var(--apple-text-secondary);
  }

  .code-section {
    max-width: 800px;
    margin-left: auto;
    margin-right: auto;
    text-align: center;
  }

  .code-section h2 {
    font-size: 28px;
    font-weight: 700;
    color: var(--apple-text-primary);
    margin-bottom: 24px;
  }

  .code-block {
    background: #1e1e1e;
    border-radius: var(--apple-border-radius);
    padding: 24px;
    text-align: left;
    box-shadow: var(--apple-shadow-md);
  }

  .code-block h4 {
    color: #ffffff;
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 12px;
  }

  .code-block pre {
    margin: 0;
    overflow-x: auto;
  }

  .code-block code {
    color: #e6fffa;
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
    font-size: 14px;
    line-height: 1.5;
  }

  @media (max-width: 768px) {
    .preview-section {
      grid-template-columns: 1fr;
    }
    
    .controls {
      flex-direction: column;
      align-items: stretch;
    }
    
    .control-group {
      min-width: auto;
    }
  }
</style>