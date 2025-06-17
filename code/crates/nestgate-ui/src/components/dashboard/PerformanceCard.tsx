import React from 'react';
import { Card, CardHeader, CardContent, Typography, Box, Paper } from '@mui/material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { 
  Dashboard as DashboardIcon, 
  TrendingUp as ArrowUpIcon, 
  TrendingDown as ArrowDownIcon 
} from '@mui/icons-material';

interface PerformanceMetrics {
  iops: {
    read: number;
    write: number;
  };
  throughput: {
    read: number;
    write: number;
  };
  latency: {
    read: number;
    write: number;
  };
}

interface PerformanceCardProps {
  performance: PerformanceMetrics;
  timeSeriesData?: Array<{
    time: string;
    readSpeed: number;
    writeSpeed: number;
    iops: number;
    readLatency: number;
    writeLatency: number;
  }>;
}

const PerformanceCard: React.FC<PerformanceCardProps> = ({ performance, timeSeriesData = [] }) => {
  const StatisticItem: React.FC<{
    title: string;
    value: number;
    precision?: number;
    suffix?: string;
    icon?: React.ReactNode;
  }> = ({ title, value, precision = 0, suffix, icon }) => (
    <Box sx={{ textAlign: 'center', padding: 1 }}>
      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', marginBottom: 1 }}>
        {icon}
        <Typography variant="body2" color="text.secondary" sx={{ marginLeft: icon ? 1 : 0 }}>
          {title}
        </Typography>
      </Box>
      <Typography variant="h6" component="div">
        {value.toFixed(precision)}{suffix}
      </Typography>
    </Box>
  );

  return (
    <Card>
      <CardHeader 
        title={
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <DashboardIcon />
            Performance Metrics
          </Box>
        }
      />
      <CardContent>
        <Box sx={{ display: 'flex', gap: 2, marginBottom: 2, flexWrap: 'wrap' }}>
          <Paper variant="outlined" sx={{ flex: 1, minWidth: 250 }}>
            <Box sx={{ padding: 2 }}>
              <Typography variant="subtitle1" align="center" gutterBottom>
                I/O Operations
              </Typography>
              <Box sx={{ display: 'flex', gap: 2 }}>
                <StatisticItem
                  title="Read IOPS"
                  value={performance.iops.read}
                  icon={<ArrowDownIcon color="primary" />}
                />
                <StatisticItem
                  title="Write IOPS"
                  value={performance.iops.write}
                  icon={<ArrowUpIcon color="secondary" />}
                />
              </Box>
            </Box>
          </Paper>
          
          <Paper variant="outlined" sx={{ flex: 1, minWidth: 250 }}>
            <Box sx={{ padding: 2 }}>
              <Typography variant="subtitle1" align="center" gutterBottom>
                Latency
              </Typography>
              <Box sx={{ display: 'flex', gap: 2 }}>
                <StatisticItem
                  title="Read Latency"
                  value={performance.latency.read}
                  precision={1}
                  suffix="ms"
                />
                <StatisticItem
                  title="Write Latency"
                  value={performance.latency.write}
                  precision={1}
                  suffix="ms"
                />
              </Box>
            </Box>
          </Paper>
        </Box>
        
        {timeSeriesData.length > 0 && (
          <Paper variant="outlined" sx={{ padding: 2 }}>
            <Typography variant="subtitle1" gutterBottom>
              Performance Trends
            </Typography>
            <ResponsiveContainer width="100%" height={200}>
              <LineChart data={timeSeriesData}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="time" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Line type="monotone" dataKey="readSpeed" stroke="#8884d8" name="Read MB/s" />
                <Line type="monotone" dataKey="writeSpeed" stroke="#82ca9d" name="Write MB/s" />
              </LineChart>
            </ResponsiveContainer>
          </Paper>
        )}
      </CardContent>
    </Card>
  );
};

export default PerformanceCard; 