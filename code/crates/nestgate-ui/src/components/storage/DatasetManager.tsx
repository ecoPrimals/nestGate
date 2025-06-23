import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  CardHeader,
  Button,
  IconButton,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Chip,
  LinearProgress,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Tooltip,
  Alert,

  Menu,
  Divider,
  Grid,
  Accordion,
  AccordionSummary,
  AccordionDetails,
} from '@mui/material';
import { SimpleTreeView } from '@mui/x-tree-view/SimpleTreeView';
import { TreeItem } from '@mui/x-tree-view/TreeItem';
import {
  Add as AddIcon,
  Refresh as RefreshIcon,
  MoreVert as MoreVertIcon,
  FolderOpen as FolderOpenIcon,
  Folder as FolderIcon,
  Storage as StorageIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  FileCopy as CloneIcon,
  Settings as SettingsIcon,
  ExpandMore as ExpandMoreIcon,
  ChevronRight as ChevronRightIcon,
  Info as InfoIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
} from '@mui/icons-material';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { ZfsPoolService } from '../../services/zfs-pool.service';
import { formatBytes, formatPercent } from '../../utils/format.utils';
import { StorageTier } from '../../types/storage';

// Types
interface Dataset {
  id: string;
  name: string;
  fullName: string;
  pool: string;
  parent?: string;
  children: Dataset[];
  type: 'filesystem' | 'volume';
  mountpoint: string;
  mounted: boolean;
  properties: {
    used: number;
    available: number;
    referenced: number;
    compression: string;
    recordsize: string;
    quota: number;
    reservation: number;
    readonly: boolean;
    atime: boolean;
    tier: StorageTier;
  };
  health: 'healthy' | 'warning' | 'error';
  lastModified: string;
}

interface DatasetAction {
  id: string;
  label: string;
  icon: React.ReactNode;
  action: (dataset: Dataset) => void;
  color?: 'primary' | 'secondary' | 'error' | 'warning';
  disabled?: (dataset: Dataset) => boolean;
}

interface DatasetManagerProps {
  poolName?: string; // Optional: manage datasets for specific pool
  showCreateButton?: boolean;
  onDatasetSelect?: (dataset: Dataset) => void;
}

const DatasetManager: React.FC<DatasetManagerProps> = ({
  poolName,
  showCreateButton = true,
  onDatasetSelect
}) => {
  // State
  const [selectedDataset, setSelectedDataset] = useState<Dataset | null>(null);
  const [actionMenuAnchor, setActionMenuAnchor] = useState<null | HTMLElement>(null);
  const [viewMode, setViewMode] = useState<'tree' | 'table'>('tree');
  const [expandedNodes, setExpandedNodes] = useState<string[]>([]);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [confirmDialog, setConfirmDialog] = useState<{
    open: boolean;
    title: string;
    message: string;
    action: () => void;
  }>({ open: false, title: '', message: '', action: () => {} });

  const queryClient = useQueryClient();

  // Queries
  const {
    data: datasets = [],
    isLoading,
    error,
    refetch
  } = useQuery({
    queryKey: ['datasets', poolName],
    queryFn: async () => {
      if (poolName) {
        return await ZfsPoolService.getDatasets(poolName);
      } else {
        // Get datasets from all pools
        const pools = await ZfsPoolService.getPools();
        const allDatasets = [];
        for (const pool of pools) {
          const poolDatasets = await ZfsPoolService.getDatasets(pool.name);
          allDatasets.push(...poolDatasets);
        }
        return allDatasets;
      }
    },
    refetchInterval: 30000,
  });

  // Build hierarchical dataset structure
  const buildDatasetTree = (datasets: any[]): Dataset[] => {
    const datasetMap = new Map<string, Dataset>();
    const rootDatasets: Dataset[] = [];

    // Convert raw datasets to Dataset objects
    datasets.forEach(ds => {
      const dataset: Dataset = {
        id: ds.id || ds.name,
        name: ds.name.split('/').pop() || ds.name,
        fullName: ds.name,
        pool: ds.pool || ds.name.split('/')[0],
        children: [],
        type: ds.type || 'filesystem',
        mountpoint: ds.mountpoint || '/',
        mounted: ds.mounted !== false,
        properties: {
          used: parseInt(ds.used) || 0,
          available: parseInt(ds.available) || 0,
          referenced: parseInt(ds.referenced) || 0,
          compression: ds.compression || 'lz4',
          recordsize: ds.recordsize || '1M',
          quota: parseInt(ds.quota) || 0,
          reservation: parseInt(ds.reservation) || 0,
          readonly: ds.readonly || false,
          atime: ds.atime !== false,
          tier: ds.tier || StorageTier.Warm,
        },
        health: ds.health || 'healthy',
        lastModified: ds.lastModified || new Date().toISOString(),
      };
      datasetMap.set(dataset.fullName, dataset);
    });

    // Build hierarchy
    datasetMap.forEach(dataset => {
      const pathParts = dataset.fullName.split('/');
      if (pathParts.length === 1) {
        // Root dataset (pool)
        rootDatasets.push(dataset);
      } else {
        // Child dataset
        const parentPath = pathParts.slice(0, -1).join('/');
        const parent = datasetMap.get(parentPath);
        if (parent) {
          parent.children.push(dataset);
          dataset.parent = parent.fullName;
        } else {
          // Parent not found, treat as root
          rootDatasets.push(dataset);
        }
      }
    });

    return rootDatasets;
  };

  const datasetTree = buildDatasetTree(datasets);

  // Dataset actions
  const datasetActions: DatasetAction[] = [
    {
      id: 'edit',
      label: 'Edit Properties',
      icon: <EditIcon />,
      action: (dataset) => handleEditDataset(dataset),
    },
    {
      id: 'clone',
      label: 'Clone Dataset',
      icon: <CloneIcon />,
      action: (dataset) => handleCloneDataset(dataset),
    },
    {
      id: 'snapshot',
      label: 'Create Snapshot',
      icon: <StorageIcon />,
      action: (dataset) => handleCreateSnapshot(dataset),
    },
    {
      id: 'destroy',
      label: 'Destroy Dataset',
      icon: <DeleteIcon />,
      color: 'error' as const,
      action: (dataset) => handleDestroyDataset(dataset),
      disabled: (dataset) => dataset.children.length > 0,
    },
  ];

  // Event handlers
  const handleDatasetClick = (dataset: Dataset) => {
    setSelectedDataset(dataset);
    onDatasetSelect?.(dataset);
  };

  const handleActionMenuOpen = (event: React.MouseEvent<HTMLElement>, dataset: Dataset) => {
    event.stopPropagation();
    setActionMenuAnchor(event.currentTarget);
    setSelectedDataset(dataset);
  };

  const handleActionMenuClose = () => {
    setActionMenuAnchor(null);
  };

  const handleEditDataset = (dataset: Dataset) => {
    setSelectedDataset(dataset);
    setEditDialogOpen(true);
    setActionMenuAnchor(null);
  };

  const handleCloneDataset = (dataset: Dataset) => {
    // TODO: Implement clone functionality
    console.log('Cloning dataset:', dataset.fullName);
    setActionMenuAnchor(null);
  };

  const handleCreateSnapshot = (dataset: Dataset) => {
    // TODO: Implement snapshot creation
    console.log('Creating snapshot for dataset:', dataset.fullName);
    setActionMenuAnchor(null);
  };

  const handleDestroyDataset = (dataset: Dataset) => {
    setConfirmDialog({
      open: true,
      title: 'Destroy Dataset',
      message: `Are you sure you want to destroy dataset "${dataset.fullName}"? This action cannot be undone and all data will be lost.`,
      action: () => {
        // TODO: Implement dataset destruction
        console.log('Destroying dataset:', dataset.fullName);
        setConfirmDialog(prev => ({ ...prev, open: false }));
      }
    });
    setActionMenuAnchor(null);
  };

  const handleToggleExpanded = (nodeId: string) => {
    setExpandedNodes(prev => 
      prev.includes(nodeId) 
        ? prev.filter(id => id !== nodeId)
        : [...prev, nodeId]
    );
  };

  const getTierColor = (tier: StorageTier) => {
    switch (tier) {
      case StorageTier.Hot: return 'error';
      case StorageTier.Warm: return 'warning';
      case StorageTier.Cold: return 'info';
      default: return 'default';
    }
  };

  const getHealthIcon = (health: string) => {
    switch (health) {
      case 'healthy': return <CheckCircleIcon color="success" />;
      case 'warning': return <WarningIcon color="warning" />;
      case 'error': return <WarningIcon color="error" />;
      default: return <InfoIcon />;
    }
  };

  // Render dataset tree item
  const renderTreeItem = (dataset: Dataset, level: number = 0) => {
    const isExpanded = expandedNodes.includes(dataset.fullName);
    const hasChildren = dataset.children.length > 0;
    const usagePercent = dataset.properties.quota > 0 
      ? (dataset.properties.used / dataset.properties.quota) * 100 
      : 0;

    return (
      <Box key={dataset.fullName}>
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            py: 1,
            px: 2,
            ml: level * 2,
            cursor: 'pointer',
            '&:hover': { backgroundColor: 'action.hover' },
            backgroundColor: selectedDataset?.fullName === dataset.fullName ? 'action.selected' : 'transparent',
          }}
          onClick={() => handleDatasetClick(dataset)}
        >
          {/* Expand/Collapse Button */}
          <IconButton
            size="small"
            onClick={(e) => {
              e.stopPropagation();
              if (hasChildren) handleToggleExpanded(dataset.fullName);
            }}
            sx={{ mr: 1, visibility: hasChildren ? 'visible' : 'hidden' }}
          >
            {hasChildren && isExpanded ? <ExpandMoreIcon /> : <ChevronRightIcon />}
          </IconButton>

          {/* Dataset Icon */}
          <Box sx={{ mr: 2 }}>
            {hasChildren ? (
              isExpanded ? <FolderOpenIcon /> : <FolderIcon />
            ) : (
              <StorageIcon />
            )}
          </Box>

          {/* Dataset Info */}
          <Box sx={{ flexGrow: 1, minWidth: 0 }}>
            <Box display="flex" alignItems="center" gap={1}>
              <Typography variant="subtitle2" noWrap>
                {dataset.name}
              </Typography>
              <Chip
                label={dataset.properties.tier}
                size="small"
                color={getTierColor(dataset.properties.tier)}
                variant="outlined"
              />
              {getHealthIcon(dataset.health)}
            </Box>
            <Typography variant="caption" color="text.secondary" noWrap>
              {dataset.mountpoint} • {formatBytes(dataset.properties.used)} used
            </Typography>
          </Box>

          {/* Usage Bar */}
          {dataset.properties.quota > 0 && (
            <Box sx={{ width: 100, mr: 2 }}>
              <LinearProgress
                variant="determinate"
                value={usagePercent}
                color={usagePercent > 80 ? 'error' : usagePercent > 60 ? 'warning' : 'primary'}
                sx={{ height: 6, borderRadius: 1 }}
              />
            </Box>
          )}

          {/* Actions Menu */}
          <IconButton
            size="small"
            onClick={(e) => handleActionMenuOpen(e, dataset)}
          >
            <MoreVertIcon />
          </IconButton>
        </Box>

        {/* Render children */}
        {hasChildren && isExpanded && (
          <Box>
            {dataset.children.map(child => renderTreeItem(child, level + 1))}
          </Box>
        )}
      </Box>
    );
  };

  if (error) {
    return (
      <Alert severity="error" sx={{ m: 2 }}>
        Failed to load datasets: {error instanceof Error ? error.message : 'Unknown error'}
      </Alert>
    );
  }

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h5" component="h2">
          Dataset Management
          {poolName && (
            <Chip label={poolName} size="small" sx={{ ml: 1 }} />
          )}
        </Typography>
        <Box display="flex" gap={1}>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={() => refetch()}
            disabled={isLoading}
          >
            Refresh
          </Button>
          {showCreateButton && (
            <Button
              variant="contained"
              startIcon={<AddIcon />}
              onClick={() => setCreateDialogOpen(true)}
            >
              Create Dataset
            </Button>
          )}
        </Box>
      </Box>

      {/* Loading State */}
      {isLoading && (
        <Box display="flex" justifyContent="center" py={4}>
          <LinearProgress sx={{ width: '100%' }} />
        </Box>
      )}

      {/* Dataset Tree */}
      {!isLoading && datasetTree.length > 0 && (
        <Card>
          <CardHeader
            title="Dataset Hierarchy"
            subheader={`${datasets.length} datasets across ${poolName ? '1 pool' : 'all pools'}`}
          />
          <CardContent sx={{ p: 0 }}>
            <Box sx={{ maxHeight: 600, overflow: 'auto' }}>
              {datasetTree.map(dataset => renderTreeItem(dataset))}
            </Box>
          </CardContent>
        </Card>
      )}

      {/* Empty State */}
      {!isLoading && datasetTree.length === 0 && (
        <Card sx={{ textAlign: 'center', py: 8 }}>
          <CardContent>
            <StorageIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
            <Typography variant="h6" gutterBottom>
              No Datasets Found
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              {poolName 
                ? `No datasets found in pool "${poolName}". Create your first dataset to get started.`
                : 'No datasets found in any pool. Create a pool first, then add datasets.'
              }
            </Typography>
            {showCreateButton && (
              <Button
                variant="contained"
                startIcon={<AddIcon />}
                onClick={() => setCreateDialogOpen(true)}
              >
                Create Dataset
              </Button>
            )}
          </CardContent>
        </Card>
      )}

      {/* Selected Dataset Details */}
      {selectedDataset && (
        <Card sx={{ mt: 3 }}>
          <CardHeader
            title={`Dataset: ${selectedDataset.fullName}`}
            subheader={`Type: ${selectedDataset.type} • Tier: ${selectedDataset.properties.tier}`}
          />
          <CardContent>
            <Box display="flex" flexWrap="wrap">
              <Box>
                <Typography variant="subtitle2" gutterBottom>Storage Usage</Typography>
                <Typography variant="body2">
                  <strong>Used:</strong> {formatBytes(selectedDataset.properties.used)}<br/>
                  <strong>Available:</strong> {formatBytes(selectedDataset.properties.available)}<br/>
                  <strong>Referenced:</strong> {formatBytes(selectedDataset.properties.referenced)}
                </Typography>
              </Box>
              <Box>
                <Typography variant="subtitle2" gutterBottom>Properties</Typography>
                <Typography variant="body2">
                  <strong>Compression:</strong> {selectedDataset.properties.compression}<br/>
                  <strong>Record Size:</strong> {selectedDataset.properties.recordsize}<br/>
                  <strong>Mountpoint:</strong> {selectedDataset.mountpoint}
                </Typography>
              </Box>
            </Box>
          </CardContent>
        </Card>
      )}

      {/* Action Menu */}
      <Menu
        anchorEl={actionMenuAnchor}
        open={Boolean(actionMenuAnchor)}
        onClose={handleActionMenuClose}
      >
        {selectedDataset && datasetActions.map((action) => (
          <MenuItem
            key={action.id}
            onClick={() => action.action(selectedDataset)}
            disabled={action.disabled?.(selectedDataset)}
            sx={{ color: action.color === 'error' ? 'error.main' : 'inherit' }}
          >
            <Box display="flex" alignItems="center" gap={1}>
              {action.icon}
              {action.label}
            </Box>
          </MenuItem>
        ))}
      </Menu>

      {/* Confirmation Dialog */}
      <Dialog
        open={confirmDialog.open}
        onClose={() => setConfirmDialog(prev => ({ ...prev, open: false }))}
      >
        <DialogTitle>{confirmDialog.title}</DialogTitle>
        <DialogContent>
          <Typography>{confirmDialog.message}</Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setConfirmDialog(prev => ({ ...prev, open: false }))}>
            Cancel
          </Button>
          <Button
            onClick={confirmDialog.action}
            color="error"
            variant="contained"
          >
            Confirm
          </Button>
        </DialogActions>
      </Dialog>

      {/* TODO: Add Create Dataset Dialog */}
      {/* TODO: Add Edit Dataset Dialog */}
    </Box>
  );
};

export default DatasetManager; 