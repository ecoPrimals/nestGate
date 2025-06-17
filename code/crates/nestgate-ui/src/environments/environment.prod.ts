export const environment = {
  production: true,
  apiUrl: '/api/v2.0',
  telemetryInterval: 300000, // 5 minutes
  defaultRefreshInterval: 60000, // 1 minute
  maxHistoryDays: 30,
  defaultChartOptions: {
    responsive: true,
    maintainAspectRatio: false,
    animation: {
      duration: 500,
      easing: 'easeInOutQuart'
    }
  }
}; 