import React from 'react';
import { 
  Card,
  CardContent,
  CardHeader,
  CircularProgress,
  Box,
  Divider,
  Typography,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper
} from '@mui/material';
import { DatasetService, ZfsDataset } from '../../services/storage/dataset.service';
import { useLiveService } from '../../utils/useLiveService';
import { DataSourceType } from '../../utils/env';
import { DataSourceIndicator, PlaceholderContent } from '../common';
import StatusChip from '../common/StatusChip';

/**
 * Example component that demonstrates proper handling of live data and placeholders
 */
const StorageDatasetExample: React.FC = () => {
  // Use the live service hook to fetch datasets
  const { data, isLoading, error, dataSource, refetch } = useLiveService<ZfsDataset[]>(
    () => DatasetService.getInstance().getDatasets(),
    [] // No dependencies, only fetch once
  );

  // If loading, show spinner
  if (isLoading) {
    return (
      <Card>
        <CardContent>
          <Box sx={{ textAlign: 'center', padding: 3 }}>
            <CircularProgress />
            <Typography variant="body2" sx={{ marginTop: 2 }}>
              Loading datasets...
            </Typography>
          </Box>
        </CardContent>
      </Card>
    );
  }

  // If error or data is placeholder, show placeholder content
  if (error || dataSource === DataSourceType.PLACEHOLDER) {
    return (
      <PlaceholderContent
        title="Dataset Management"
        description="Dataset management functionality is being integrated with live data."
        height={400}
      />
    );
  }

  return (
    <Card>
      <CardHeader 
        title="ZFS Datasets" 
        action={<DataSourceIndicator dataSource={dataSource} />}
      />
      <CardContent>
        <TableContainer component={Paper} variant="outlined">
          <Table size="small">
            <TableHead>
              <TableRow>
                <TableCell>Name</TableCell>
                <TableCell>Type</TableCell>
                <TableCell>Used</TableCell>
                <TableCell>Available</TableCell>
                <TableCell>Compression</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {(data || []).map((dataset) => (
                <TableRow key={dataset.id}>
                  <TableCell>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                      {dataset.name}
                      {dataset.dataSource !== DataSourceType.LIVE && (
                        <DataSourceIndicator dataSource={dataset.dataSource} size="small" />
                      )}
                    </Box>
                  </TableCell>
                  <TableCell>{dataset.type}</TableCell>
                  <TableCell>{dataset.used}</TableCell>
                  <TableCell>{dataset.available}</TableCell>
                  <TableCell>
                    <StatusChip 
                      status="info" 
                      label={dataset.compression} 
                      size="small"
                    />
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
        
        <Divider sx={{ marginY: 2 }} />
        
        <Box sx={{ textAlign: 'center' }}>
          <Typography variant="caption" color="text.secondary">
            This is an example component showing proper live data handling
          </Typography>
        </Box>
      </CardContent>
    </Card>
  );
};

export default StorageDatasetExample; 