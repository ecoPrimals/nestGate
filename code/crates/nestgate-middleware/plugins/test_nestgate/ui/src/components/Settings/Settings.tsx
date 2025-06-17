import { Box, Paper, Typography, List, ListItem, ListItemText, Switch, Divider } from '@mui/material';
import { useQuery } from '@tanstack/react-query';
import { apiService } from '@services';
import { SystemSettings } from '@types';
import { capitalizeWords } from '@utils';
import { useApiError } from '@hooks';

export function Settings() {
  const { handleError, ErrorComponent } = useApiError();

  const { data: settings } = useQuery<SystemSettings[]>({
    queryKey: ['system-settings'],
    queryFn: () => apiService.get('/system-settings').then((res) => res.data),
    onError: handleError,
  });

  const renderSettingValue = (setting: SystemSettings) => {
    switch (typeof setting.value) {
      case 'boolean':
        return <Switch checked={setting.value} disabled />;
      case 'number':
        return setting.value.toString();
      default:
        return setting.value;
    }
  };

  return (
    <Box>
      <Paper sx={{ p: 2, mb: 2 }}>
        <Typography variant="h4" gutterBottom>
          Settings
        </Typography>
        <Typography variant="body1">
          Configure your NestGate system settings here.
        </Typography>
      </Paper>

      <Paper sx={{ p: 2 }}>
        <Typography variant="h6" gutterBottom>
          System Configuration
        </Typography>
        <List>
          {settings?.map((setting, index) => (
            <Box key={setting.id}>
              <ListItem>
                <ListItemText
                  primary={capitalizeWords(setting.name)}
                  secondary={
                    <>
                      <Typography variant="body2" color="text.secondary">
                        Category: {capitalizeWords(setting.category)}
                      </Typography>
                      {setting.description && (
                        <Typography variant="body2" color="text.secondary">
                          {setting.description}
                        </Typography>
                      )}
                      <Box sx={{ mt: 1 }}>
                        {renderSettingValue(setting)}
                      </Box>
                    </>
                  }
                />
              </ListItem>
              {index < (settings.length - 1) && <Divider />}
            </Box>
          ))}
          {!settings?.length && (
            <ListItem>
              <ListItemText primary="No settings found" />
            </ListItem>
          )}
        </List>
      </Paper>
      <ErrorComponent />
    </Box>
  );
} 