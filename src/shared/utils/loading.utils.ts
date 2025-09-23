// Loading utility functions and configurations
export interface LoadingConfig {
  size?: 'small' | 'medium' | 'large';
  color?: string;
  text?: string;
  style?: 'rings' | 'dots' | 'wave';
  duration?: number;
}

export const defaultLoadingConfig: LoadingConfig = {
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
export function mergeLoadingConfig(
  baseConfig: LoadingConfig,
  overrideConfig?: Partial<LoadingConfig>
): LoadingConfig {
  return {
    ...baseConfig,
    ...overrideConfig
  };
}

// Helper function to get preset configuration
export function getLoadingPreset(
  presetName: keyof typeof loadingPresets,
  overrides?: Partial<LoadingConfig>
): LoadingConfig {
  return mergeLoadingConfig(loadingPresets[presetName] as LoadingConfig, overrides);
}

// Loading state manager for complex scenarios
export class LoadingStateManager {
  private isLoading = false;
  private config: LoadingConfig = defaultLoadingConfig;
  private subscribers: Array<(loading: boolean, config: LoadingConfig) => void> = [];

  startLoading(config?: Partial<LoadingConfig>) {
    this.isLoading = true;
    this.config = { ...this.config, ...config };
    this.notifySubscribers();
  }

  stopLoading() {
    this.isLoading = false;
    this.notifySubscribers();
  }

  updateConfig(config: Partial<LoadingConfig>) {
    this.config = { ...this.config, ...config };
    if (this.isLoading) {
      this.notifySubscribers();
    }
  }

  subscribe(callback: (loading: boolean, config: LoadingConfig) => void) {
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
export const loadingManager = new LoadingStateManager();