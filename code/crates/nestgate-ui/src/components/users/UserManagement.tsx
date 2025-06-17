import React, { useState } from 'react';
import { Card, Dialog, DialogTitle, DialogContent, Tabs, Tab, Typography, Box } from '@mui/material';
import { Person as UserIcon } from '@mui/icons-material';
import { UserList } from './UserList';
import { UserForm } from './UserForm';
import { User } from '../../services/user.service';
import TabPanel from '../common/TabPanel';

export const UserManagement: React.FC = () => {
  const [createModalVisible, setCreateModalVisible] = useState(false);
  const [editModalVisible, setEditModalVisible] = useState(false);
  const [selectedUser, setSelectedUser] = useState<User | undefined>(undefined);
  const [activeTab, setActiveTab] = useState(0);
  
  // Handle opening the create user modal
  const handleAddUser = () => {
    setCreateModalVisible(true);
  };
  
  // Handle opening the edit user modal
  const handleEditUser = (user: User) => {
    setSelectedUser(user);
    setEditModalVisible(true);
  };
  
  // Handle closing modals
  const handleCancel = () => {
    setCreateModalVisible(false);
    setEditModalVisible(false);
    setSelectedUser(undefined);
  };
  
  // Handle saving user (both create and edit)
  const handleSave = () => {
    setCreateModalVisible(false);
    setEditModalVisible(false);
    setSelectedUser(undefined);
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };
  
  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        User Management
      </Typography>
      
      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 2 }}>
        <Tabs value={activeTab} onChange={handleTabChange} aria-label="user management tabs">
          <Tab 
            icon={<UserIcon />}
            label="Users" 
            iconPosition="start"
          />
        </Tabs>
      </Box>
      
      <TabPanel value={activeTab} index={0}>
        <Card>
          <UserList 
            onAddUser={handleAddUser} 
            onEditUser={handleEditUser} 
          />
        </Card>
      </TabPanel>
      
      {/* Create User Dialog */}
      <Dialog
        open={createModalVisible}
        onClose={handleCancel}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>Create New User</DialogTitle>
        <DialogContent>
          <UserForm 
            onSave={handleSave}
            onCancel={handleCancel}
          />
        </DialogContent>
      </Dialog>
      
      {/* Edit User Dialog */}
      <Dialog
        open={editModalVisible}
        onClose={handleCancel}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>Edit User</DialogTitle>
        <DialogContent>
          <UserForm 
            user={selectedUser}
            onSave={handleSave}
            onCancel={handleCancel}
          />
        </DialogContent>
      </Dialog>
    </Box>
  );
}; 