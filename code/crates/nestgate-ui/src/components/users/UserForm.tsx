import React, { useState, useEffect } from 'react';
import { 
  Card, 
  CardContent, 
  TextField, 
  Select, 
  MenuItem, 
  Button, 
  FormControlLabel,
  Checkbox, 
  Switch, 
  Divider, 
  Typography,
  Box,
  InputAdornment,
  FormControl,
  InputLabel,
  FormGroup,
  Stack,
  Alert
} from '@mui/material';
import { 
  Person as UserIcon, 
  Email as MailIcon, 
  Lock as LockIcon, 
  Badge as IdcardIcon 
} from '@mui/icons-material';
// Note: Using console for notifications until notistack is properly configured
import { User, UserRole, Permission, UserService, CreateUserParams, UpdateUserParams, RolePermissions } from '../../services/user.service';

interface UserFormProps {
  user?: User;
  onSave: () => void;
  onCancel: () => void;
}

interface FormData {
  username: string;
  email: string;
  fullName: string;
  password: string;
  role: UserRole;
  isActive: boolean;
  customPermissions: Permission[];
}

export const UserForm: React.FC<UserFormProps> = ({ user, onSave, onCancel }) => {
  const userService = UserService.getInstance();
  const isEditMode = !!user;
  
  const [formData, setFormData] = useState<FormData>({
    username: user?.username || '',
    email: user?.email || '',
    fullName: user?.fullName || '',
    password: '',
    role: user?.role || UserRole.USER,
    isActive: user?.isActive ?? true,
    customPermissions: user?.customPermissions || [],
  });
  
  const [selectedRole, setSelectedRole] = useState<UserRole>(
    user?.role || UserRole.USER
  );
  const [customPermissions, setCustomPermissions] = useState<Permission[]>(
    user?.customPermissions || []
  );
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showCustomPermissions, setShowCustomPermissions] = useState(
    !!user?.customPermissions?.length
  );
  const [errors, setErrors] = useState<Record<string, string>>({});
  
  // Set form values when user prop changes
  useEffect(() => {
    if (user) {
      setFormData({
        username: user.username || '',
        email: user.email || '',
        fullName: user.fullName || '',
        password: '',
        role: user.role || UserRole.USER,
        isActive: user.isActive ?? true,
        customPermissions: user.customPermissions || [],
      });
      setSelectedRole(user.role || UserRole.USER);
      setCustomPermissions(user.customPermissions || []);
      setShowCustomPermissions(!!user.customPermissions?.length);
    }
  }, [user]);
  
  // Handle input changes
  const handleInputChange = (field: keyof FormData) => (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const value = event.target.type === 'checkbox' ? event.target.checked : event.target.value;
    setFormData(prev => ({ ...prev, [field]: value }));
    
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: '' }));
    }
  };
  
  // Handle role change
  const handleRoleChange = (event: any) => {
    const value = event.target.value as UserRole;
    setSelectedRole(value);
    setFormData(prev => ({ ...prev, role: value }));
    
    // Reset custom permissions when role changes
    if (!showCustomPermissions) {
      setCustomPermissions([]);
    }
  };
  
  // Handle custom permission checkbox changes
  const handlePermissionChange = (permission: Permission) => (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const newPermissions = event.target.checked
      ? [...customPermissions, permission]
      : customPermissions.filter(p => p !== permission);
    
    setCustomPermissions(newPermissions);
  };
  
  // Validate form
  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};
    
    if (!formData.username.trim()) {
      newErrors.username = 'Username is required';
    } else if (formData.username.length < 3) {
      newErrors.username = 'Username must be at least 3 characters';
    } else if (!/^[a-zA-Z0-9_-]+$/.test(formData.username)) {
      newErrors.username = 'Username can only contain letters, numbers, underscores, and hyphens';
    }
    
    if (!formData.email.trim()) {
      newErrors.email = 'Email is required';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      newErrors.email = 'Please enter a valid email address';
    }
    
    if (!formData.fullName.trim()) {
      newErrors.fullName = 'Full name is required';
    }
    
    if (!isEditMode && !formData.password) {
      newErrors.password = 'Password is required';
    } else if (formData.password && formData.password.length < 8) {
      newErrors.password = 'Password must be at least 8 characters';
    }
    
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };
  
  // Handle form submission
  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    
    if (!validateForm()) {
      return;
    }
    
    setIsSubmitting(true);
    
    try {
      if (isEditMode && user) {
        // Update existing user
        const updateParams: UpdateUserParams = {
          email: formData.email,
          fullName: formData.fullName,
          role: formData.role,
          isActive: formData.isActive,
          customPermissions: showCustomPermissions ? customPermissions : undefined,
        };
        
        // Add password to update if it was provided
        if (formData.password) {
          updateParams.password = formData.password;
        }
        
        await userService.updateUser(user.id, updateParams);
        console.log('User updated successfully');
      } else {
        // Create new user
        const createParams: CreateUserParams = {
          username: formData.username,
          password: formData.password,
          email: formData.email,
          fullName: formData.fullName,
          role: formData.role,
          customPermissions: showCustomPermissions ? customPermissions : undefined,
        };
        
        await userService.createUser(createParams);
        console.log('User created successfully');
      }
      
      onSave();
    } catch (error) {
      console.error(`Failed to ${isEditMode ? 'update' : 'create'} user:`, error);
      console.error(`Error ${isEditMode ? 'updating' : 'creating'} user:`, error);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  return (
    <Card>
      <CardContent>
        <form onSubmit={handleSubmit}>
        {/* Basic Information */}
          <Typography variant="h6" gutterBottom>
            Basic Information
          </Typography>
        
          <Stack spacing={3} sx={{ marginBottom: 3 }}>
            <TextField
              fullWidth
          label="Username"
              value={formData.username}
              onChange={handleInputChange('username')}
              error={!!errors.username}
              helperText={errors.username}
              disabled={isEditMode}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <UserIcon />
                  </InputAdornment>
                ),
              }}
            placeholder="Enter username"
              required
          />
        
            <TextField
              fullWidth
          label="Email"
              type="email"
              value={formData.email}
              onChange={handleInputChange('email')}
              error={!!errors.email}
              helperText={errors.email}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <MailIcon />
                  </InputAdornment>
                ),
              }}
            placeholder="Enter email address"
              required
          />
        
            <TextField
              fullWidth
          label="Full Name"
              value={formData.fullName}
              onChange={handleInputChange('fullName')}
              error={!!errors.fullName}
              helperText={errors.fullName}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <IdcardIcon />
                  </InputAdornment>
                ),
              }}
            placeholder="Enter full name"
              required
          />
        
            <TextField
              fullWidth
          label="Password"
              type="password"
              value={formData.password}
              onChange={handleInputChange('password')}
              error={!!errors.password}
              helperText={errors.password || (isEditMode ? "Leave blank to keep the current password" : "")}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <LockIcon />
                  </InputAdornment>
                ),
              }}
            placeholder={isEditMode ? "Leave blank to keep current password" : "Enter password"}
              required={!isEditMode}
          />
          </Stack>
        
          <Divider sx={{ marginY: 3 }} />
        
        {/* Role and Permissions */}
          <Typography variant="h6" gutterBottom>
            Role and Permissions
          </Typography>
          
          <Stack spacing={3} sx={{ marginBottom: 3 }}>
            <FormControl fullWidth required>
              <InputLabel>Role</InputLabel>
              <Select
                value={formData.role}
          label="Role"
            onChange={handleRoleChange}
          >
                <MenuItem value={UserRole.ADMIN}>Administrator</MenuItem>
                <MenuItem value={UserRole.MANAGER}>Manager</MenuItem>
                <MenuItem value={UserRole.USER}>Regular User</MenuItem>
                <MenuItem value={UserRole.GUEST}>Guest</MenuItem>
          </Select>
            </FormControl>
        
        {isEditMode && (
              <FormControlLabel
                control={
                  <Switch
                    checked={formData.isActive}
                    onChange={handleInputChange('isActive')}
                  />
                }
            label="Active Status"
            />
        )}
        
            <FormControlLabel
              control={
          <Checkbox 
            checked={showCustomPermissions}
            onChange={(e) => setShowCustomPermissions(e.target.checked)}
                />
              }
              label="Customize permissions for this user"
            />
          </Stack>
        
        {showCustomPermissions && (
            <Card variant="outlined" sx={{ marginBottom: 3 }}>
              <CardContent>
                <Typography variant="subtitle1" gutterBottom>
                  Custom Permissions
                </Typography>
                <Box sx={{ maxHeight: '300px', overflowY: 'auto' }}>
                  <Stack spacing={2}>
                    <div>
                      <Typography variant="subtitle2" gutterBottom>
                        System Permissions
                      </Typography>
                      <FormGroup sx={{ marginLeft: 2 }}>
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.SYSTEM_ADMIN)}
                              onChange={handlePermissionChange(Permission.SYSTEM_ADMIN)}
                            />
                          }
                          label="System Administration"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.SYSTEM_CONFIGURE)}
                              onChange={handlePermissionChange(Permission.SYSTEM_CONFIGURE)}
                            />
                          }
                          label="System Configuration"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.SYSTEM_MONITOR)}
                              onChange={handlePermissionChange(Permission.SYSTEM_MONITOR)}
                            />
                          }
                          label="System Monitoring"
                        />
                      </FormGroup>
                  </div>
                  
                    <div>
                      <Typography variant="subtitle2" gutterBottom>
                        Storage Permissions
                      </Typography>
                      <FormGroup sx={{ marginLeft: 2 }}>
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.STORAGE_ADMIN)}
                              onChange={handlePermissionChange(Permission.STORAGE_ADMIN)}
                            />
                          }
                          label="Storage Administration"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.STORAGE_MANAGE)}
                              onChange={handlePermissionChange(Permission.STORAGE_MANAGE)}
                            />
                          }
                          label="Storage Management"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.STORAGE_READ)}
                              onChange={handlePermissionChange(Permission.STORAGE_READ)}
                            />
                          }
                          label="Storage Read"
                        />
                      </FormGroup>
                  </div>
                  
                    <div>
                      <Typography variant="subtitle2" gutterBottom>
                        User Permissions
                      </Typography>
                      <FormGroup sx={{ marginLeft: 2 }}>
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.USER_ADMIN)}
                              onChange={handlePermissionChange(Permission.USER_ADMIN)}
                            />
                          }
                          label="User Administration"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.USER_MANAGE)}
                              onChange={handlePermissionChange(Permission.USER_MANAGE)}
                            />
                          }
                          label="User Management"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.USER_READ)}
                              onChange={handlePermissionChange(Permission.USER_READ)}
                            />
                          }
                          label="User Read"
                        />
                      </FormGroup>
                  </div>
                  
                    <div>
                      <Typography variant="subtitle2" gutterBottom>
                        Network Permissions
                      </Typography>
                      <FormGroup sx={{ marginLeft: 2 }}>
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.NETWORK_ADMIN)}
                              onChange={handlePermissionChange(Permission.NETWORK_ADMIN)}
                            />
                          }
                          label="Network Administration"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.NETWORK_MANAGE)}
                              onChange={handlePermissionChange(Permission.NETWORK_MANAGE)}
                            />
                          }
                          label="Network Management"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.NETWORK_READ)}
                              onChange={handlePermissionChange(Permission.NETWORK_READ)}
                            />
                          }
                          label="Network Read"
                        />
                      </FormGroup>
                  </div>
                  
                    <div>
                      <Typography variant="subtitle2" gutterBottom>
                        Backup Permissions
                      </Typography>
                      <FormGroup sx={{ marginLeft: 2 }}>
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.BACKUP_ADMIN)}
                              onChange={handlePermissionChange(Permission.BACKUP_ADMIN)}
                            />
                          }
                          label="Backup Administration"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.BACKUP_MANAGE)}
                              onChange={handlePermissionChange(Permission.BACKUP_MANAGE)}
                            />
                          }
                          label="Backup Management"
                        />
                        <FormControlLabel
                          control={
                            <Checkbox
                              checked={customPermissions.includes(Permission.BACKUP_READ)}
                              onChange={handlePermissionChange(Permission.BACKUP_READ)}
                            />
                          }
                          label="Backup Read"
                        />
                      </FormGroup>
                  </div>
                  </Stack>
                </Box>
            
            <Button 
                  variant="text" 
              size="small" 
              onClick={() => {
                // Reset to default permissions for the selected role
                setCustomPermissions(RolePermissions[selectedRole]);
              }}
                  sx={{ marginTop: 2 }}
            >
              Reset to Default Permissions
            </Button>
              </CardContent>
            </Card>
        )}
        
          <Box sx={{ display: 'flex', justifyContent: 'flex-end', gap: 1, marginTop: 3 }}>
            <Button onClick={onCancel} disabled={isSubmitting}>
            Cancel
          </Button>
            <Button 
              type="submit" 
              variant="contained" 
              disabled={isSubmitting}
            >
            {isEditMode ? 'Update User' : 'Create User'}
          </Button>
          </Box>
        </form>
      </CardContent>
    </Card>
  );
}; 