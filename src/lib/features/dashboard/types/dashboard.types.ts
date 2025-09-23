// Dashboard types
export interface DashboardProps {
  view?: string;
  onAddDatabase: () => void;
  onEditDatabase: (id: string) => void;
  onCreateTask: () => void;
  onViewTask: (id: string) => void;
}

export interface ChartDataset {
  label: string;
  data: number[];
  borderColor: string;
  backgroundColor: string;
  tension: number;
  fill: boolean;
}

export interface ChartData {
  labels: string[];
  datasets: ChartDataset[];
}