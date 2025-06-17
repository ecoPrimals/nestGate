export const environment = {
  production: false,
  apiUrl: '/api/v2.0',
  telemetryInterval: 60000, // 1 minute
  defaultRefreshInterval: 30000, // 30 seconds
  maxHistoryDays: 30,
  defaultChartOptions: {
    responsive: true,
    maintainAspectRatio: false,
    animation: {
      duration: 750,
      easing: 'easeInOutQuart'
    }
  }
}; 