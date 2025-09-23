// Loading utility functions and configurations (for global use)
export interface GlobalLoadingConfig {
  size?: 'small' | 'medium' | 'large';
  color?: string;
  text?: string;
  style?: 'rings' | 'dots' | 'wave';
  duration?: number;
}

export const defaultGlobalLoadingConfig: GlobalLoadingConfig = {
  size: 'medium',
  color: 'var(--apple-primary)',
  text: '',
  style: 'rings',
  duration: 0
};

// Predefined loading configurations for different scenarios
export const loadingPresets = {
  // Application loading states
  appInit: {
    size: 'large',
    color: 'var(--apple-primary)',
    text: '正在初始化应用...',
    style: 'rings' as const
  },
  
  dataLoading: {
    size: 'medium',
    color: 'var(--apple-primary)',
    text: '加载数据中...',
    style: 'dots' as const
  },
  
  processing: {
    size: 'medium',
    color: 'var(--apple-success)',
    text: '处理中...',
    style: 'wave' as const
  },
  
  saving: {
    size: 'small',
    color: 'var(--apple-primary)',
    text: '保存中...',
    style: 'rings' as const
  },
  
  connecting: {
    size: 'small',
    color: 'var(--apple-secondary)',
    text: '连接中...',
    style: 'dots' as const
  },
  
  uploading: {
    size: 'medium',
    color: 'var(--apple-warning)',
    text: '上传中...',
    style: 'wave' as const
  },
  
  // Button loading states
  buttonLoading: {
    size: 'small',
    color: '#ffffff',
    text: '',
    style: 'rings' as const
  },
  
  // Card loading states
  cardLoading: {
    size: 'medium',
    color: 'var(--apple-text-tertiary)',
    text: '',
    style: 'dots' as const
  },
  
  // Full screen loading
  fullscreen: {
    size: 'large',
    color: 'var(--apple-primary)',
    text: '请稍候...',
    style: 'rings' as const
  }
};

// Helper function to merge configurations
export function mergeGlobalLoadingConfig(
  baseConfig: GlobalLoadingConfig,
  overrideConfig?: Partial<GlobalLoadingConfig>
): GlobalLoadingConfig {
  return {
    ...baseConfig,
    ...overrideConfig
  };
}

// Helper function to get preset configuration
export function getLoadingPreset(
  presetName: keyof typeof loadingPresets,
  overrides?: Partial<GlobalLoadingConfig>
): GlobalLoadingConfig {
  return mergeGlobalLoadingConfig(loadingPresets[presetName] as GlobalLoadingConfig, overrides);
}

// Loading state manager for complex scenarios
export class LoadingStateManager {
  private isLoading = false;
  private config: GlobalLoadingConfig = defaultGlobalLoadingConfig;
  private subscribers: Array<(loading: boolean, config: GlobalLoadingConfig) => void> = [];

  setLoading(loading: boolean, config?: Partial<GlobalLoadingConfig>): void {
    this.isLoading = loading;
    if (config) {
      this.config = { ...this.config, ...config };
    }
    this.notifySubscribers();
  }

  stopLoading() {
    this.isLoading = false;
    this.notifySubscribers();
  }

  updateConfig(config: Partial<GlobalLoadingConfig>): void {
    this.config = { ...this.config, ...config };
    this.notifySubscribers();
  }

  subscribe(callback: (loading: boolean, config: GlobalLoadingConfig) => void): () => void {
    this.subscribers.push(callback);
    // Immediately notify with current state
    callback(this.isLoading, this.config);
    
    // Return unsubscribe function
    return () => {
      const index = this.subscribers.indexOf(callback);
      if (index > -1) {
        this.subscribers.splice(index, 1);
      }
    };
  }

  private notifySubscribers() {
    this.subscribers.forEach(callback => {
      callback(this.isLoading, this.config);
    });
  }

  getLoadingState() {
    return {
      isLoading: this.isLoading,
      config: this.config
    };
  }
}

// Export singleton instance
export const globalLoadingManager = new LoadingStateManager();