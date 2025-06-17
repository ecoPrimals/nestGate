import React, { useState, useEffect } from 'react';
import { 
  Box, 
  Button, 
  Tooltip, 
  Dialog, 
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  IconButton,
  Typography,
  LinearProgress
} from '@mui/material';
import { DataGrid, GridColDef, GridRenderCellParams } from '@mui/x-data-grid';
import { 
  Person as UserIcon, 
  Edit as EditIcon, 
  Delete as DeleteIcon, 
  Lock as LockIcon 
} from '@mui/icons-material';
import { User, UserRole, UserService } from '../../services/user.service';
import { formatDate } from '../../utils/format';
import StatusChip from '../common/StatusChip';

interface UserListProps {
  onEditUser: (user: User) => void;
  onAddUser: () => void;
}

export const UserList: React.FC<UserListProps> = ({ onEditUser, onAddUser }) => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [resetPasswordModalOpen, setResetPasswordModalOpen] = useState<boolean>(false);
  const [deleteConfirmOpen, setDeleteConfirmOpen] = useState<boolean>(false);
  const [selectedUserId, setSelectedUserId] = useState<string | null>(null);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [newPassword, setNewPassword] = useState<string>('');
  
  const userService = UserService.getInstance();
  
  // Fetch users on component mount
  useEffect(() => {
    fetchUsers();
  }, []);
  
  // Fetch all users
  const fetchUsers = async () => {
    setLoading(true);
    try {
      const data = await userService.getUsers();
      setUsers(data);
    } catch (error) {
      console.error('Failed to fetch users:', error);
    } finally {
      setLoading(false);
    }
  };
  
  // Handle user deletion
  const handleDeleteUser = async () => {
    if (!selectedUserId) return;
    
    try {
      await userService.deleteUser(selectedUserId);
      console.log('User deleted successfully');
      setDeleteConfirmOpen(false);
      setSelectedUserId(null);
      setSelectedUser(null);
      fetchUsers(); // Refresh the list
    } catch (error) {
      console.error('Failed to delete user:', error);
    }
  };
  
  // Open reset password modal
  const showResetPasswordModal = (id: string) => {
    setSelectedUserId(id);
    setResetPasswordModalOpen(true);
  };
  
  // Open delete confirmation
  const showDeleteConfirm = (user: User) => {
    setSelectedUserId(user.id);
    setSelectedUser(user);
    setDeleteConfirmOpen(true);
  };
  
  // Handle password reset
  const handleResetPassword = async () => {
    if (!selectedUserId || !newPassword) return;
    
    try {
      await userService.resetPassword(selectedUserId, newPassword);
      console.log('Password reset successfully');
      setResetPasswordModalOpen(false);
      setNewPassword('');
      setSelectedUserId(null);
    } catch (error) {
      console.error('Failed to reset password:', error);
    }
  };
  
  // Get role color for StatusChip
  const getRoleStatus = (role: UserRole): string => {
    switch (role) {
      case UserRole.ADMIN:
        return 'error';
      case UserRole.MANAGER:
        return 'warning';
      case UserRole.USER:
        return 'success';
      case UserRole.GUEST:
        return 'default';
      default:
        return 'default';
    }
  };
  
  // User table columns
  const columns: GridColDef[] = [
    {
      field: 'username',
      headerName: 'Username',
      width: 150,
      renderCell: (params: GridRenderCellParams) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <UserIcon fontSize="small" />
          <span>{params.value}</span>
        </Box>
      ),
    },
    {
      field: 'fullName',
      headerName: 'Full Name',
      width: 200,
    },
    {
      field: 'email',
      headerName: 'Email',
      width: 250,
    },
    {
      field: 'role',
      headerName: 'Role',
      width: 120,
      renderCell: (params: GridRenderCellParams) => (
        <StatusChip 
          status={getRoleStatus(params.value as UserRole)} 
          label={params.value.toUpperCase()}
          size="small"
        />
      ),
    },
    {
      field: 'isActive',
      headerName: 'Status',
      width: 100,
      renderCell: (params: GridRenderCellParams) => (
        <StatusChip 
          status={params.value ? 'success' : 'error'} 
          label={params.value ? 'Active' : 'Inactive'}
          size="small"
        />
      ),
    },
    {
      field: 'createdAt',
      headerName: 'Created',
      width: 150,
      renderCell: (params: GridRenderCellParams) => formatDate(params.value as string),
    },
    {
      field: 'lastLogin',
      headerName: 'Last Login',
      width: 150,
      renderCell: (params: GridRenderCellParams) => 
        params.value ? formatDate(params.value as string) : 'Never',
    },
    {
      field: 'actions',
      headerName: 'Actions',
      width: 150,
      sortable: false,
      renderCell: (params: GridRenderCellParams) => (
        <Box sx={{ display: 'flex', gap: 0.5 }}>
          <Tooltip title="Edit User">
            <IconButton 
              size="small" 
              onClick={() => onEditUser(params.row as User)}
            >
              <EditIcon fontSize="small" />
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Reset Password">
            <IconButton 
              size="small"
              onClick={() => showResetPasswordModal(params.row.id)}
            >
              <LockIcon fontSize="small" />
            </IconButton>
          </Tooltip>
          
          <Tooltip title="Delete User">
            <IconButton 
              size="small"
              color="error"
              disabled={params.row.username === 'admin'}
              onClick={() => showDeleteConfirm(params.row as User)}
            >
              <DeleteIcon fontSize="small" />
            </IconButton>
          </Tooltip>
        </Box>
      ),
    },
  ];

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 2 }}>
        <Button variant="contained" onClick={onAddUser}>
          Add User
        </Button>
      </Box>
      
      <Box sx={{ height: 600, width: '100%' }}>
        <DataGrid 
          rows={users} 
        columns={columns} 
        loading={loading}
          pageSizeOptions={[10, 25, 50]}
          initialState={{
            pagination: { paginationModel: { pageSize: 10 } },
          }}
          disableRowSelectionOnClick
          sx={{
            '& .MuiDataGrid-row:hover': {
              backgroundColor: 'action.hover',
            },
          }}
        />
      </Box>
      
      {/* Reset Password Dialog */}
      <Dialog
        open={resetPasswordModalOpen}
        onClose={() => {
          setResetPasswordModalOpen(false);
          setNewPassword('');
          setSelectedUserId(null);
        }}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Reset User Password</DialogTitle>
        <DialogContent>
          <Typography variant="body1" sx={{ marginBottom: 2 }}>
            Enter a new password for this user:
          </Typography>
          <TextField
            fullWidth
            type="password" 
            label="New Password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            variant="outlined"
            autoFocus
          />
        </DialogContent>
        <DialogActions>
          <Button 
            onClick={() => {
              setResetPasswordModalOpen(false);
              setNewPassword('');
              setSelectedUserId(null);
            }}
          >
            Cancel
          </Button>
          <Button 
            onClick={handleResetPassword}
            variant="contained"
            disabled={!newPassword}
          >
            Reset Password
          </Button>
        </DialogActions>
      </Dialog>
      
      {/* Delete Confirmation Dialog */}
      <Dialog
        open={deleteConfirmOpen}
        onClose={() => {
          setDeleteConfirmOpen(false);
          setSelectedUserId(null);
          setSelectedUser(null);
        }}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Delete User</DialogTitle>
        <DialogContent>
          <Typography variant="body1">
            Are you sure you want to delete user "{selectedUser?.username}"?
          </Typography>
          <Typography variant="body2" color="text.secondary" sx={{ marginTop: 1 }}>
            This action cannot be undone.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button 
            onClick={() => {
              setDeleteConfirmOpen(false);
              setSelectedUserId(null);
              setSelectedUser(null);
            }}
          >
            Cancel
          </Button>
          <Button 
            onClick={handleDeleteUser}
            variant="contained"
            color="error"
          >
            Delete
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}; 