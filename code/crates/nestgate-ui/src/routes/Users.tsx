import React from 'react';
import { UserManagement } from '../components/users/UserManagement';
import { Permission } from '../services/user.service';
import { ProtectedRoute } from '../components/auth/ProtectedRoute';

const UsersPage: React.FC = () => {
  return (
    <ProtectedRoute requiredPermission={Permission.USER_READ}>
      <UserManagement />
    </ProtectedRoute>
  );
};

export default UsersPage; 