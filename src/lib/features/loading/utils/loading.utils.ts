// Loading utility functions
export interface LoadingConfig {
  size?: 'small' | 'medium' | 'large';
  color?: string;
  text?: string;
  style?: 'rings' | 'dots' | 'wave';
  type?: 'basic' | 'modern' | 'custom';
  overlay?: boolean;
  fullscreen?: boolean;
  message?: string;
  duration?: number;
}

export interface LoadingOptions extends LoadingConfig {}

export function getLoadingSizeClass(size: string): string {
  const sizeMap = {
    'small': 'loading-sm',
    'medium': 'loading-md',
    'large': 'loading-lg'
  };
  return sizeMap[size as keyof typeof sizeMap] || 'loading-md';
}

export function createLoadingManager() {
  let loadingCount = 0;
  let subscribers: Array<(isLoading: boolean, config: LoadingConfig) => void> = [];
  let currentConfig: LoadingConfig = {};

  const subscribe = (callback: (isLoading: boolean, config: LoadingConfig) => void) => {
    subscribers.push(callback);
    callback(loadingCount > 0, currentConfig);
    
    return () => {
      subscribers = subscribers.filter(sub => sub !== callback);
    };
  };

  const notifySubscribers = () => {
    subscribers.forEach(callback => callback(loadingCount > 0, currentConfig));
  };

  const show = (options: LoadingOptions = {}) => {
    loadingCount++;
    currentConfig = { ...currentConfig, ...options };
    notifySubscribers();
  };

  const hide = () => {
    loadingCount = Math.max(0, loadingCount - 1);
    notifySubscribers();
  };

  const hideAll = () => {
    loadingCount = 0;
    notifySubscribers();
  };

  return {
    show,
    hide,
    hideAll,
    subscribe,
    get isLoading() {
      return loadingCount > 0;
    }
  };
}

// Global loading manager instance
export const loadingManager = createLoadingManager();