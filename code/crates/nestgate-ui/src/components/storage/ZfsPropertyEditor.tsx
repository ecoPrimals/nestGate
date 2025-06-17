import React, { useState } from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Button,
  FormHelperText,
  Alert,
  Snackbar,
  SelectChangeEvent,
} from '@mui/material';
import SaveIcon from '@mui/icons-material/Save';

interface ZfsPropertyEditorProps {
  tierId: string;
  properties: Record<string, string>;
  onPropertyUpdate: (property: string, value: string) => Promise<void>;
}

interface PropertyOption {
  value: string;
  label: string;
  description: string;
}

const ZfsPropertyEditor: React.FC<ZfsPropertyEditorProps> = ({
  tierId,
  properties,
  onPropertyUpdate,
}) => {
  const [loading, setLoading] = useState<Record<string, boolean>>({});
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [editedProperties, setEditedProperties] = useState<Record<string, string>>(properties);

  // Compression options
  const compressionOptions: PropertyOption[] = [
    { value: 'off', label: 'Off', description: 'No compression' },
    { value: 'lz4', label: 'LZ4', description: 'Fast compression algorithm, good for hot tier' },
    { value: 'zstd', label: 'ZSTD', description: 'Balanced compression, good for warm tier' },
    { value: 'gzip', label: 'GZIP', description: 'Better compression ratio but slower' },
    { value: 'gzip-9', label: 'GZIP-9', description: 'Maximum compression, good for cold tier' },
  ];

  // Record size options
  const recordSizeOptions: PropertyOption[] = [
    { value: '4K', label: '4K', description: 'Small record size, good for databases with small records' },
    { value: '8K', label: '8K', description: 'Good for general-purpose file systems' },
    { value: '16K', label: '16K', description: 'Balanced for mixed workloads' },
    { value: '32K', label: '32K', description: 'Better for larger files' },
    { value: '64K', label: '64K', description: 'Good for streaming data' },
    { value: '128K', label: '128K', description: 'Default, good for general purpose' },
    { value: '1M', label: '1M', description: 'Large record size, good for large files and archives' },
  ];

  // Access time options
  const atimeOptions: PropertyOption[] = [
    { value: 'on', label: 'On', description: 'Update access time for all file reads' },
    { value: 'off', label: 'Off', description: 'Do not update access time, better performance' },
    { value: 'relative', label: 'Relative', description: 'Update access time only if more recent than mtime' },
  ];

  // Primary cache options
  const primaryCacheOptions: PropertyOption[] = [
    { value: 'all', label: 'All', description: 'Cache both data and metadata' },
    { value: 'metadata', label: 'Metadata', description: 'Cache only metadata' },
    { value: 'none', label: 'None', description: 'Disable caching' },
  ];

  // Handle property change
  const handlePropertyChange = (property: string, event: SelectChangeEvent) => {
    const value = event.target.value;
    setEditedProperties({
      ...editedProperties,
      [property]: value,
    });
  };

  // Handle save
  const handleSave = async (property: string) => {
    setLoading({ ...loading, [property]: true });
    setError(null);
    setSuccess(null);

    try {
      await onPropertyUpdate(property, editedProperties[property]);
      setSuccess(`Updated ${property} to ${editedProperties[property]}`);
    } catch (err) {
      console.error(`Error updating ${property}:`, err);
      setError(`Failed to update ${property}. Please try again.`);
    } finally {
      setLoading({ ...loading, [property]: false });
    }
  };

  // Render property editor for a specific property
  const renderPropertyEditor = (
    property: string,
    options: PropertyOption[],
    title: string,
    description: string
  ) => {
    const currentValue = editedProperties[property] || '';
    const hasChanged = properties[property] !== currentValue;
    
    return (
      <Box sx={{ flex: '1 1 400px', minWidth: { xs: '100%', md: '400px' } }}>
        <Card variant="outlined">
          <CardContent>
            <Typography variant="h6" gutterBottom>
              {title}
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              {description}
            </Typography>
            
            <FormControl fullWidth>
              <InputLabel id={`${property}-label`}>{title}</InputLabel>
              <Select
                labelId={`${property}-label`}
                id={property}
                value={currentValue}
                label={title}
                onChange={(e) => handlePropertyChange(property, e)}
              >
                {options.map((option) => (
                  <MenuItem key={option.value} value={option.value}>
                    <Box>
                      <Typography variant="body1">{option.label}</Typography>
                      <Typography variant="caption" color="text.secondary">
                        {option.description}
                      </Typography>
                    </Box>
                  </MenuItem>
                ))}
              </Select>
              <FormHelperText>
                Current setting: {properties[property]}
              </FormHelperText>
            </FormControl>
            
            <Box mt={2}>
              <Button
                variant="contained"
                color="primary"
                startIcon={<SaveIcon />}
                disabled={!hasChanged || loading[property]}
                onClick={() => handleSave(property)}
              >
                {loading[property] ? 'Saving...' : 'Apply Change'}
              </Button>
            </Box>
          </CardContent>
        </Card>
      </Box>
    );
  };

  return (
    <Box>
      <Typography variant="h5" gutterBottom>
        ZFS Properties
      </Typography>
      
      <Typography variant="body1" paragraph>
        Modify ZFS properties for this storage tier. Changes will be applied immediately.
      </Typography>
      
      <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 3 }}>
        {renderPropertyEditor(
          'compression',
          compressionOptions,
          'Compression',
          'Set the compression algorithm for this tier. Higher compression saves space but may impact performance.'
        )}
        
        {renderPropertyEditor(
          'recordsize',
          recordSizeOptions,
          'Record Size',
          'Set the maximum size of a logical block in the file system. Choose based on your workload.'
        )}
        
        {renderPropertyEditor(
          'atime',
          atimeOptions,
          'Access Time',
          'Control whether access time is updated when files are read. Disabling improves performance.'
        )}
        
        {renderPropertyEditor(
          'primarycache',
          primaryCacheOptions,
          'Primary Cache',
          'Control what data is cached in ARC (Adaptive Replacement Cache).'
        )}
      </Box>
      
      <Snackbar
        open={!!error}
        autoHideDuration={6000}
        onClose={() => setError(null)}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
      >
        <Alert severity="error" onClose={() => setError(null)}>
          {error}
        </Alert>
      </Snackbar>
      
      <Snackbar
        open={!!success}
        autoHideDuration={3000}
        onClose={() => setSuccess(null)}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
      >
        <Alert severity="success" onClose={() => setSuccess(null)}>
          {success}
        </Alert>
      </Snackbar>
    </Box>
  );
};

export default ZfsPropertyEditor; 