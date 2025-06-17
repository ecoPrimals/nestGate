import React, { useState } from 'react';
import {
  Card,
  CardContent,
  TextField,
  Button,
  FormControlLabel,
  Switch,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Box,
  Stack,
  CircularProgress,
} from '@mui/material';
import { Save as SaveIcon, Backup as BackupIcon, Restore as RestoreIcon } from '@mui/icons-material';
import { BackupSettings, SettingsFormProps } from '../types';
import { useSettingsForm } from '../hooks/useSettingsForm';

const BackupSettingsTab: React.FC<SettingsFormProps<BackupSettings> & {
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}> = ({ onSave, initialValues, onSuccess, onError }) => {
  const [formData, setFormData] = useState<BackupSettings>({
    automaticBackups: initialValues?.automaticBackups || false,
    backupFrequency: initialValues?.backupFrequency || 'daily',
    backupLocation: initialValues?.backupLocation || '',
    backupsToKeep: initialValues?.backupsToKeep || 5,
  });

  const { loading, handleSave } = useSettingsForm({
    sectionName: 'Backup',
    onSubmit: async (values) => onSave(values),
    onSuccess,
    onError,
  });

  const handleSubmit = () => {
    handleSave(formData);
  };

  const handleSwitchChange = (checked: boolean) => {
    setFormData(prev => ({ ...prev, automaticBackups: checked }));
  };

  const handleFrequencyChange = (value: string) => {
    setFormData(prev => ({ ...prev, backupFrequency: value as 'daily' | 'weekly' | 'monthly' }));
  };

  const handleLocationChange = (value: string) => {
    setFormData(prev => ({ ...prev, backupLocation: value }));
  };

  const handleKeepChange = (value: string) => {
    const numValue = parseInt(value) || 0;
    setFormData(prev => ({ ...prev, backupsToKeep: numValue }));
  };

  const handleCreateBackup = () => {
    console.log('Creating backup...');
    if (onSuccess) {
      onSuccess('Backup created successfully');
    }
  };

  const handleRestoreBackup = () => {
    console.log('Restoring from backup...');
    if (onSuccess) {
      onSuccess('Restore initiated successfully');
    }
  };

  return (
    <Card>
      <CardContent>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
          <FormControlLabel
            control={
              <Switch
                checked={formData.automaticBackups}
                onChange={(e) => handleSwitchChange(e.target.checked)}
              />
            }
            label="Automatic Backups"
          />

          <FormControl fullWidth>
            <InputLabel>Backup Frequency</InputLabel>
            <Select
              value={formData.backupFrequency}
              onChange={(e) => handleFrequencyChange(e.target.value)}
              label="Backup Frequency"
              disabled={!formData.automaticBackups}
            >
              <MenuItem value="daily">Daily</MenuItem>
              <MenuItem value="weekly">Weekly</MenuItem>
              <MenuItem value="monthly">Monthly</MenuItem>
            </Select>
          </FormControl>

          <TextField
            label="Backup Location"
            value={formData.backupLocation}
            onChange={(e) => handleLocationChange(e.target.value)}
            placeholder="/path/to/backup"
            fullWidth
          />

          <TextField
            label="Number of Backups to Keep"
            type="number"
            value={formData.backupsToKeep}
            onChange={(e) => handleKeepChange(e.target.value)}
            inputProps={{ min: 1, max: 100 }}
            placeholder="5"
            fullWidth
          />

          <Stack direction="row" spacing={2} flexWrap="wrap" useFlexGap>
            <Button
              variant="contained"
              startIcon={loading ? <CircularProgress size={20} /> : <SaveIcon />}
              onClick={handleSubmit}
              disabled={loading}
            >
              Save Backup Settings
            </Button>
            <Button
              variant="outlined"
              startIcon={<BackupIcon />}
              onClick={handleCreateBackup}
            >
              Create Backup Now
            </Button>
            <Button
              variant="outlined"
              color="error"
              startIcon={<RestoreIcon />}
              onClick={handleRestoreBackup}
            >
              Restore from Backup
            </Button>
          </Stack>
        </Box>
      </CardContent>
    </Card>
  );
};

export default BackupSettingsTab; 