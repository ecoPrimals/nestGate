import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { UserList } from '../UserList';
import { UserService, User, UserRole } from '../../../services/user.service';

// Mock the UserService
jest.mock('../../../services/user.service', () => ({
  UserService: {
    getInstance: jest.fn(),
    hasPermission: jest.fn(),
    ...jest.requireActual('../../../services/user.service').UserService
  },
  UserRole: jest.requireActual('../../../services/user.service').UserRole,
}));

// Mock Ant Design components that might be harder to test
jest.mock('antd', () => {
  const actual = jest.requireActual('antd');
  return {
    ...actual,
    message: {
      success: jest.fn(),
      error: jest.fn(),
    },
    Popconfirm: ({ children, onConfirm, title }: any) => (
      <div data-testid="popconfirm" onClick={onConfirm}>
        {title}
        {children}
      </div>
    ),
  };
});

describe('UserList Component', () => {
  const mockUsers: User[] = [
    {
      id: '1',
      username: 'admin',
      email: 'admin@example.com',
      fullName: 'Admin User',
      role: UserRole.ADMIN,
      isActive: true,
      createdAt: '2023-01-01T00:00:00Z',
      lastLogin: '2023-07-15T08:30:00Z',
    },
    {
      id: '2',
      username: 'user',
      email: 'user@example.com',
      fullName: 'Regular User',
      role: UserRole.USER,
      isActive: true,
      createdAt: '2023-02-01T00:00:00Z',
    },
    {
      id: '3',
      username: 'inactive',
      email: 'inactive@example.com',
      fullName: 'Inactive User',
      role: UserRole.USER,
      isActive: false,
      createdAt: '2023-03-01T00:00:00Z',
    },
  ];

  const mockGetUsers = jest.fn().mockResolvedValue(mockUsers);
  const mockDeleteUser = jest.fn().mockResolvedValue(true);
  const mockResetPassword = jest.fn().mockResolvedValue(true);

  const mockUserService = {
    getUsers: mockGetUsers,
    deleteUser: mockDeleteUser,
    resetPassword: mockResetPassword,
  };

  // Mock props
  const onEditUser = jest.fn();
  const onAddUser = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
    // Set up the mock implementation
    (UserService.getInstance as jest.Mock).mockReturnValue(mockUserService);
  });

  it('should render user list with data', async () => {
    render(<UserList onEditUser={onEditUser} onAddUser={onAddUser} />);

    // Should show loading initially
    expect(screen.getByText(/loading/i)).toBeInTheDocument();

    // Wait for data to load
    await waitFor(() => {
      expect(mockGetUsers).toHaveBeenCalledTimes(1);
      expect(screen.getByText('admin')).toBeInTheDocument();
      expect(screen.getByText('Regular User')).toBeInTheDocument();
      expect(screen.getByText('ADMIN')).toBeInTheDocument();
      expect(screen.getByText('USER')).toBeInTheDocument();
    });
  });

  it('should handle add user button click', async () => {
    render(<UserList onEditUser={onEditUser} onAddUser={onAddUser} />);

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Add User')).toBeInTheDocument();
    });

    // Click the Add User button
    fireEvent.click(screen.getByText('Add User'));
    
    expect(onAddUser).toHaveBeenCalledTimes(1);
  });

  it('should handle edit user button click', async () => {
    render(<UserList onEditUser={onEditUser} onAddUser={onAddUser} />);

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('admin')).toBeInTheDocument();
    });

    // Find and click the first edit button
    const editButtons = screen.getAllByRole('button');
    // Find the edit button (first button in each row)
    const editButton = editButtons[0];
    fireEvent.click(editButton);
    
    expect(onEditUser).toHaveBeenCalledTimes(1);
    expect(onEditUser).toHaveBeenCalledWith(mockUsers[0]);
  });

  it('should handle delete user', async () => {
    const { container } = render(<UserList onEditUser={onEditUser} onAddUser={onAddUser} />);

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Regular User')).toBeInTheDocument();
    });

    // Find delete buttons
    const deleteButtons = container.querySelectorAll('[data-testid="popconfirm"]');
    // Click the delete button for the second user (non-admin)
    fireEvent.click(deleteButtons[1]);
    
    await waitFor(() => {
      expect(mockDeleteUser).toHaveBeenCalledTimes(1);
      expect(mockDeleteUser).toHaveBeenCalledWith('2');
      expect(mockGetUsers).toHaveBeenCalledTimes(2); // Initial load + refresh
    });
  });

  it('should show error message when deletion fails', async () => {
    // Mock an error for this test case
    mockDeleteUser.mockRejectedValueOnce(new Error('Failed to delete'));
    
    const { container } = render(<UserList onEditUser={onEditUser} onAddUser={onAddUser} />);

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Regular User')).toBeInTheDocument();
    });

    // Find delete buttons
    const deleteButtons = container.querySelectorAll('[data-testid="popconfirm"]');
    // Click the delete button for the second user
    fireEvent.click(deleteButtons[1]);
    
    await waitFor(() => {
      expect(mockDeleteUser).toHaveBeenCalledTimes(1);
      expect(require('antd').message.error).toHaveBeenCalledWith('Failed to delete user');
    });
  });

  it('should handle reset password modal', async () => {
    const { container } = render(<UserList onEditUser={onEditUser} onAddUser={onAddUser} />);

    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Regular User')).toBeInTheDocument();
    });

    // Find and click reset password button (second button for each user)
    const buttons = container.querySelectorAll('button');
    const resetPasswordButtons = Array.from(buttons).filter(
      button => button.innerHTML.includes('LockOutlined')
    );
    fireEvent.click(resetPasswordButtons[0]);
    
    // Wait for modal to appear
    await waitFor(() => {
      expect(screen.getByText('Reset User Password')).toBeInTheDocument();
      expect(screen.getByText('Enter a new password for this user:')).toBeInTheDocument();
    });

    // Enter new password
    const input = screen.getByRole('textbox');
    fireEvent.change(input, { target: { value: 'newpassword123' } });
    
    // Click OK
    const okButton = screen.getByRole('button', { name: /ok/i });
    fireEvent.click(okButton);
    
    await waitFor(() => {
      expect(mockResetPassword).toHaveBeenCalledTimes(1);
      expect(mockResetPassword).toHaveBeenCalledWith('1', 'newpassword123');
      expect(require('antd').message.success).toHaveBeenCalledWith('Password reset successfully');
    });
  });

  it('should handle fetching users failure', async () => {
    // Mock error for getUsers
    mockGetUsers.mockRejectedValueOnce(new Error('Failed to fetch'));
    
    render(<UserList onEditUser={onEditUser} onAddUser={onAddUser} />);

    await waitFor(() => {
      expect(mockGetUsers).toHaveBeenCalledTimes(1);
      expect(require('antd').message.error).toHaveBeenCalledWith('Failed to fetch users');
    });
  });
}); 