import React, { useState } from 'react';
import {
  Card,
  CardHeader,
  CardContent,
  Button,
  TextField,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Typography,
  Switch,
  FormControlLabel,
  Divider,
  List,
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
  IconButton,
  Alert,
  Box,
  Chip,
  Stack,
  CircularProgress,
  InputAdornment,
  Tooltip,
  Pagination
} from '@mui/material';
import {
  Folder as FolderIcon,
  Delete as DeleteIcon,
  Refresh as RefreshIcon,
  Link as LinkIcon
} from '@mui/icons-material';
import { useFileSystemMonitor, FileSystemEvent, WatchedDirectory } from '../../hooks/useFileSystemMonitor';
import EmptyState from '../common/EmptyState';
import StatusChip from '../common/StatusChip';

interface FileSystemMonitorProps {
  showDetailed?: boolean;
}

// Define event kinds to match the backend
enum EventKind {
  Create = 'create',
  Modify = 'modify',
  Delete = 'remove',
  Rename = 'rename',
  Permissions = 'permissions',
  Other = 'other'
}

const FileSystemMonitor: React.FC<FileSystemMonitorProps> = ({ 
  showDetailed = false 
}) => {
  const [directoryPath, setDirectoryPath] = useState<string>('');
  const [recursive, setRecursive] = useState(true);
  const [includeHidden, setIncludeHidden] = useState(false);
  const [extensions, setExtensions] = useState<string>('');
  const [page, setPage] = useState(1);
  const rowsPerPage = 10;

  // Use the filesystem monitor hook
  const {
    isConnected,
    events,
    watchedDirectories,
    isLoading,
    error,
    allocatedPort,
    portManagerConnected,
    portManagerError,
    fetchEvents,
    addWatch,
    removeWatch
  } = useFileSystemMonitor({
    preferredPort: 9500,
    autoConnect: true,
    serviceId: 'fs-monitor-ui'
  });

  const handleAddDirectory = async () => {
    if (!directoryPath) return;

    // Filter params need to be passed in metadata
    // since they're not directly supported by the hook
    const metadata = {
      includeHidden,
      extensions: extensions ? extensions.split(',').map(e => e.trim()) : undefined
    };

    await addWatch(directoryPath, recursive);
    setDirectoryPath('');
  };

  const handleRemoveDirectory = async (watchId: string) => {
    await removeWatch(watchId);
  };

  const handleClearEvents = () => {
    // Re-fetch events to refresh the list
    fetchEvents(0);
  };

  const handlePageChange = (event: React.ChangeEvent<unknown>, value: number) => {
    setPage(value);
  };

  // Filter for events based on pagination
  const paginatedEvents = showDetailed
    ? events.slice((page - 1) * rowsPerPage, page * rowsPerPage)
    : events.slice(0, 10);

  // Map backend event_type to UI EventKind
  const mapEventType = (type: string): EventKind => {
    switch (type) {
      case 'create': return EventKind.Create;
      case 'modify': return EventKind.Modify;
      case 'remove': return EventKind.Delete;
      case 'rename': return EventKind.Rename;
      default: return EventKind.Other;
    }
  };

  // Map event kind to chip color
  const getEventKindChip = (kind: string) => {
    const eventKind = mapEventType(kind);
    
    let color: "success" | "error" | "warning" | "info" | "default" = "default";
    
    switch (eventKind) {
      case EventKind.Create:
        color = "success";
        break;
      case EventKind.Modify:
        color = "info";
        break;
      case EventKind.Delete:
        color = "error";
        break;
      case EventKind.Rename:
        color = "warning";
        break;
      default:
        color = "default";
    }

    return <Chip label={eventKind} color={color} size="small" />;
  };

  // Connection status chip
  const getConnectionStatus = () => {
    if (!portManagerConnected) {
      return <Chip label="Port Manager Disconnected" color="error" size="small" />;
    }
    
    if (!allocatedPort) {
      return <Chip label="Allocating Port..." color="warning" size="small" />;
    }
    
    if (!isConnected) {
      return <Chip label="Connecting..." color="warning" size="small" />;
    }
    
    return <Chip label="Connected" color="success" size="small" />;
  };

  return (
    <Card>
      <CardHeader
        title={
          <Stack direction="row" spacing={1} alignItems="center">
            <FolderIcon />
            <Typography variant="h6">File System Monitor</Typography>
            {allocatedPort && (
              <Tooltip title="Port allocated by Port Manager">
                <Chip 
                  icon={<LinkIcon />} 
                  label={`Port: ${allocatedPort}`} 
                  color="primary" 
                  size="small" 
                />
              </Tooltip>
            )}
            {getConnectionStatus()}
          </Stack>
        }
        action={
          <Stack direction="row" spacing={1}>
            <Button 
              startIcon={<RefreshIcon />} 
              onClick={handleClearEvents} 
              disabled={!isConnected || events.length === 0}
              variant="outlined"
              size="small"
            >
              Clear Events
            </Button>
            {!showDetailed && (
              <Button 
                variant="contained" 
                onClick={() => window.open('/monitoring', '_blank')}
                size="small"
              >
                View Details
              </Button>
            )}
          </Stack>
        }
      />
      <CardContent>
        {(error || portManagerError) && (
          <Alert 
            severity="error" 
            sx={{ mb: 2 }}
          >
            {error?.message || portManagerError?.message || "Error connecting to file system monitor"}
          </Alert>
        )}

        <Box sx={{ mb: 3 }}>
          <Stack spacing={2}>
            <TextField
              fullWidth
              label="Enter directory path to monitor"
              value={directoryPath}
              onChange={(e) => setDirectoryPath(e.target.value)}
              InputProps={{
                endAdornment: (
                  <InputAdornment position="end">
                    <Button 
                      variant="contained" 
                      onClick={handleAddDirectory} 
                      disabled={!directoryPath || isLoading || !isConnected}
                      size="small"
                    >
                      Add
                    </Button>
                  </InputAdornment>
                ),
              }}
            />
            <Stack direction="row" spacing={2}>
              <FormControlLabel
                control={
                  <Switch 
                    checked={recursive} 
                    onChange={(e) => setRecursive(e.target.checked)} 
                  />
                }
                label={recursive ? "Recursive" : "Non-recursive"}
              />
              <FormControlLabel
                control={
                  <Switch 
                    checked={includeHidden} 
                    onChange={(e) => setIncludeHidden(e.target.checked)} 
                  />
                }
                label={includeHidden ? "Include hidden" : "Exclude hidden"}
              />
              <TextField 
                label="File extensions (comma-separated)" 
                value={extensions} 
                onChange={(e) => setExtensions(e.target.value)} 
                size="small"
                sx={{ flexGrow: 1 }}
              />
            </Stack>
          </Stack>
        </Box>

        <Typography variant="h6" gutterBottom>
          Watched Directories
        </Typography>
        <Divider sx={{ mb: 2 }} />
        
        {isLoading ? (
          <Box display="flex" justifyContent="center" p={2}>
            <CircularProgress />
          </Box>
        ) : watchedDirectories.length === 0 ? (
          <EmptyState 
            title="No directories being monitored"
            message="Add a directory path above to start monitoring"
          />
        ) : (
          <List>
            {watchedDirectories.map((dir: WatchedDirectory) => (
              <ListItem key={dir.id} divider>
                <ListItemText 
                  primary={dir.path} 
                  secondary={dir.recursive ? "Recursive monitoring" : "Non-recursive monitoring"}
                />
                <ListItemSecondaryAction>
                  <IconButton 
                    edge="end" 
                    onClick={() => handleRemoveDirectory(dir.id)}
                    color="error"
                  >
                    <DeleteIcon />
                  </IconButton>
                </ListItemSecondaryAction>
              </ListItem>
            ))}
          </List>
        )}

        <Typography variant="h6" sx={{ mt: 4, mb: 1 }}>
          File Events
        </Typography>
        <Divider sx={{ mb: 2 }} />

        {!isConnected ? (
          <EmptyState 
            title="Not connected"
            message="Waiting for connection to filesystem monitor"
            icon={<LinkIcon color="disabled" sx={{ fontSize: 40 }} />}
          />
        ) : events.length === 0 ? (
          <EmptyState 
            title="No events detected"
            message="File system events will appear here when detected"
          />
        ) : (
          <>
            <TableContainer component={Paper} sx={{ maxHeight: showDetailed ? 400 : 300 }}>
              <Table stickyHeader size="small">
                <TableHead>
                  <TableRow>
                    <TableCell width="100">Type</TableCell>
                    <TableCell>Path</TableCell>
                    <TableCell width="180">Timestamp</TableCell>
                    <TableCell width="100">Type</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {paginatedEvents.map((event: FileSystemEvent, idx: number) => (
                    <TableRow key={event.id || idx}>
                      <TableCell>{getEventKindChip(event.event_type)}</TableCell>
                      <TableCell sx={{ maxWidth: 400, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                        {event.path}
                      </TableCell>
                      <TableCell>{new Date(event.timestamp).toLocaleString()}</TableCell>
                      <TableCell>
                        <Chip 
                          label={event.is_directory ? "Directory" : "File"} 
                          size="small" 
                          variant="outlined"
                        />
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
            
            {showDetailed && events.length > rowsPerPage && (
              <Box display="flex" justifyContent="center" mt={2}>
                <Pagination 
                  count={Math.ceil(events.length / rowsPerPage)} 
                  page={page} 
                  onChange={handlePageChange} 
                  color="primary" 
                />
              </Box>
            )}
          </>
        )}
      </CardContent>
    </Card>
  );
};

export default FileSystemMonitor; 