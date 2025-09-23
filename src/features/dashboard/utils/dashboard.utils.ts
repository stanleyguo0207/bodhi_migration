// Dashboard utility functions
export function formatNumber(num: number): string {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M';
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K';
  }
  return num.toString();
}

export function calculateGrowthRate(current: number, previous: number): string {
  if (previous === 0) return current > 0 ? "+100%" : "0%";
  const rate = (((current - previous) / previous) * 100).toFixed(0);
  return Number(rate) > 0 ? `+${rate}%` : `${rate}%`;
}

export function getTimeLabels(): string[] {
  return ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"];
}