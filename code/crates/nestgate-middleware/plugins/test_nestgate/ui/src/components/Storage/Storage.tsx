import { Box, Paper, Typography, List, ListItem, ListItemText, Divider } from '@mui/material';
import { useQuery } from '@tanstack/react-query';
import { apiService } from '@services';
import { StorageSystem } from '@types';
import { formatBytes, formatDate } from '@utils';
import { useApiError } from '@hooks';

export function Storage() {
  const { handleError, ErrorComponent } = useApiError();

  const { data: storageSystems } = useQuery<StorageSystem[]>({
    queryKey: ['storage-systems'],
    queryFn: () => apiService.get('/storage-systems').then((res) => res.data),
    onError: handleError,
  });

  return (
    <Box>
      <Paper sx={{ p: 2, mb: 2 }}>
        <Typography variant="h4" gutterBottom>
          Storage Management
        </Typography>
        <Typography variant="body1">
          Manage and monitor your storage systems from this interface.
        </Typography>
      </Paper>

      <Paper sx={{ p: 2 }}>
        <Typography variant="h6" gutterBottom>
          Storage Systems
        </Typography>
        <List>
          {storageSystems?.map((system, index) => (
            <Box key={system.id}>
              <ListItem>
                <ListItemText
                  primary={system.name}
                  secondary={
                    <>
                      <Typography variant="body2" color="text.secondary">
                        Status: {system.status}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Capacity: {formatBytes(system.capacity.used)} / {formatBytes(system.capacity.total)}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Last Updated: {formatDate(system.lastUpdated)}
                      </Typography>
                    </>
                  }
                />
              </ListItem>
              {index < (storageSystems.length - 1) && <Divider />}
            </Box>
          ))}
          {!storageSystems?.length && (
            <ListItem>
              <ListItemText primary="No storage systems found" />
            </ListItem>
          )}
        </List>
      </Paper>
      <ErrorComponent />
    </Box>
  );
} 