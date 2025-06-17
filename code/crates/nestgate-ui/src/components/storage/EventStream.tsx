import React, { useState, useEffect } from 'react';
import { 
  Box, 
  Typography, 
  List, 
  ListItem, 
  ListItemText, 
  Chip, 
  FormControl, 
  FormControlLabel, 
  Checkbox, 
  Paper, 
  IconButton, 
  InputBase, 
  FormGroup, 
  CircularProgress, 
  Alert
} from '@mui/material';
import { Search as SearchIcon, Refresh as RefreshIcon } from '@mui/icons-material';
import { tieredStorageService, FsEvent, EventFilter } from '../../services/storage/tieredStorageService';

interface EventStreamProps {
  tierId: string;
  refreshInterval?: number;
}

const EventStream: React.FC<EventStreamProps> = ({ tierId, refreshInterval = 10000 }) => {
  const [events, setEvents] = useState<FsEvent[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<Partial<EventFilter>>({
    includeDirectories: true,
    includeHidden: false,
    eventTypes: ['CREATE', 'MODIFY', 'DELETE'],
  });
  const [searchQuery, setSearchQuery] = useState<string>('');

  // Load events on mount and when tierId or filter changes
  useEffect(() => {
    const fetchEvents = async () => {
      if (!tierId) return;
      
      try {
        setLoading(true);
        const data = await tieredStorageService.getEvents(tierId, filter);
        setEvents(data);
        setError(null);
      } catch (err) {
        console.error('Failed to load filesystem events:', err);
        setError('Failed to load events. Please try again later.');
      } finally {
        setLoading(false);
      }
    };

    fetchEvents();

    // Set up refresh interval
    const intervalId = setInterval(fetchEvents, refreshInterval);
    
    return () => clearInterval(intervalId);
  }, [tierId, filter, refreshInterval]);

  // Handle filter changes
  const handleFilterChange = (key: keyof EventFilter, value: any) => {
    setFilter(prev => ({
      ...prev,
      [key]: value,
    }));
  };

  // Handle event type toggle
  const handleEventTypeToggle = (eventType: string) => {
    setFilter(prev => {
      const eventTypes = prev.eventTypes || [];
      const updatedTypes = eventTypes.includes(eventType)
        ? eventTypes.filter(type => type !== eventType)
        : [...eventTypes, eventType];
      
      return {
        ...prev,
        eventTypes: updatedTypes,
      };
    });
  };

  // Handle manual refresh
  const handleRefresh = () => {
    setLoading(true);
    tieredStorageService.getEvents(tierId, filter)
      .then(data => {
        setEvents(data);
        setError(null);
      })
      .catch(err => {
        console.error('Failed to refresh events:', err);
        setError('Failed to refresh events. Please try again.');
      })
      .finally(() => setLoading(false));
  };

  // Filter events by search query
  const filteredEvents = events.filter(event => 
    searchQuery ? event.path.toLowerCase().includes(searchQuery.toLowerCase()) : true
  );

  // Format timestamp to readable format
  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };

  // Get event type color
  const getEventTypeColor = (eventType: string) => {
    switch (eventType) {
      case 'CREATE':
        return 'success';
      case 'MODIFY':
        return 'primary';
      case 'DELETE':
        return 'error';
      default:
        return 'default';
    }
  };

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
        <Typography variant="h5">Filesystem Events</Typography>
        <IconButton onClick={handleRefresh} disabled={loading}>
          <RefreshIcon />
        </IconButton>
      </Box>

      {/* Search and Filters */}
      <Box mb={3}>
        <Paper
          component="form"
          sx={{ p: '2px 4px', display: 'flex', alignItems: 'center', mb: 2 }}
        >
          <InputBase
            sx={{ ml: 1, flex: 1 }}
            placeholder="Search file paths"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          <IconButton type="button" sx={{ p: '10px' }}>
            <SearchIcon />
          </IconButton>
        </Paper>

        <Typography variant="subtitle1" gutterBottom>Filters</Typography>
        <FormGroup row>
          <FormControlLabel
            control={
              <Checkbox
                checked={filter.includeDirectories}
                onChange={(e) => handleFilterChange('includeDirectories', e.target.checked)}
              />
            }
            label="Include Directories"
          />
          <FormControlLabel
            control={
              <Checkbox
                checked={filter.includeHidden}
                onChange={(e) => handleFilterChange('includeHidden', e.target.checked)}
              />
            }
            label="Include Hidden Files"
          />
        </FormGroup>

        <Box mt={1}>
          <Typography variant="subtitle2" gutterBottom>Event Types</Typography>
          <Box display="flex" gap={1}>
            <Chip
              label="CREATE"
              color={filter.eventTypes?.includes('CREATE') ? 'success' : 'default'}
              onClick={() => handleEventTypeToggle('CREATE')}
              variant={filter.eventTypes?.includes('CREATE') ? 'filled' : 'outlined'}
            />
            <Chip
              label="MODIFY"
              color={filter.eventTypes?.includes('MODIFY') ? 'primary' : 'default'}
              onClick={() => handleEventTypeToggle('MODIFY')}
              variant={filter.eventTypes?.includes('MODIFY') ? 'filled' : 'outlined'}
            />
            <Chip
              label="DELETE"
              color={filter.eventTypes?.includes('DELETE') ? 'error' : 'default'}
              onClick={() => handleEventTypeToggle('DELETE')}
              variant={filter.eventTypes?.includes('DELETE') ? 'filled' : 'outlined'}
            />
          </Box>
        </Box>
      </Box>

      {/* Event List */}
      {loading && events.length === 0 ? (
        <Box display="flex" justifyContent="center" p={3}>
          <CircularProgress />
        </Box>
      ) : error ? (
        <Alert severity="error" sx={{ mt: 2 }}>
          {error}
        </Alert>
      ) : filteredEvents.length === 0 ? (
        <Alert severity="info">No events matching the current filters</Alert>
      ) : (
        <List>
          {filteredEvents.map((event) => (
            <ListItem
              key={event.id}
              divider
              secondaryAction={
                <Chip
                  label={event.kind}
                  color={getEventTypeColor(event.kind) as any}
                  size="small"
                />
              }
            >
              <ListItemText
                primary={<Typography noWrap>{event.path}</Typography>}
                secondary={
                  <Box>
                    <Typography variant="body2" color="text.secondary">
                      {formatTimestamp(event.timestamp)}
                    </Typography>
                    {event.isDirectory && (
                      <Chip label="Directory" size="small" variant="outlined" sx={{ ml: 1 }} />
                    )}
                  </Box>
                }
              />
            </ListItem>
          ))}
        </List>
      )}
    </Box>
  );
};

export default EventStream; 