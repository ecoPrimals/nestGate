import axios from 'axios';
import { UserService, User, UserRole, Permission, RolePermissions } from '../user.service';

jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('UserService', () => {
  // Reset mocks before each test
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('getInstance', () => {
    it('should return the same instance on multiple calls', () => {
      const instance1 = UserService.getInstance();
      const instance2 = UserService.getInstance();
      expect(instance1).toBe(instance2);
    });
  });

  describe('getUsers', () => {
    it('should fetch users from API', async () => {
      const mockUsers = [
        { id: '1', username: 'test1', email: 'test1@example.com', role: UserRole.ADMIN },
        { id: '2', username: 'test2', email: 'test2@example.com', role: UserRole.USER }
      ];
      
      mockedAxios.get.mockResolvedValueOnce({ data: mockUsers });
      
      const userService = UserService.getInstance();
      const result = await userService.getUsers();
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toEqual(mockUsers);
    });

    it('should return mock users when API fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
      
      const userService = UserService.getInstance();
      const result = await userService.getUsers();
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result.length).toBeGreaterThan(0); // Should return mock users
      expect(result[0].isMock).toBe(true);
    });
  });

  describe('getUserById', () => {
    it('should fetch a user by ID from API', async () => {
      const mockUser = { 
        id: '1', 
        username: 'test1', 
        email: 'test1@example.com', 
        role: UserRole.ADMIN 
      };
      
      mockedAxios.get.mockResolvedValueOnce({ data: mockUser });
      
      const userService = UserService.getInstance();
      const result = await userService.getUserById('1');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(mockedAxios.get).toHaveBeenCalledWith(expect.stringContaining('/1'));
      expect(result).toEqual(mockUser);
    });

    it('should return mock user when API fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
      
      const userService = UserService.getInstance();
      const result = await userService.getUserById('1');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result?.isMock).toBe(true);
    });

    it('should return null when user not found', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('User not found'));
      
      const userService = UserService.getInstance();
      const result = await userService.getUserById('999');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toBeNull();
    });
  });

  describe('createUser', () => {
    it('should create a user through API', async () => {
      const newUser = { 
        username: 'newuser', 
        password: 'password123', 
        email: 'new@example.com', 
        fullName: 'New User',
        role: UserRole.USER 
      };
      
      const createdUser = { 
        id: '6', 
        username: 'newuser', 
        email: 'new@example.com',
        fullName: 'New User',
        role: UserRole.USER,
        isActive: true,
        createdAt: '2023-07-20T00:00:00Z'
      };
      
      mockedAxios.post.mockResolvedValueOnce({ data: createdUser });
      
      const userService = UserService.getInstance();
      const result = await userService.createUser(newUser);
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(mockedAxios.post).toHaveBeenCalledWith(
        expect.any(String),
        newUser
      );
      expect(result).toEqual(createdUser);
    });

    it('should create a mock user when API fails', async () => {
      const newUser = { 
        username: 'newuser', 
        password: 'password123', 
        email: 'new@example.com', 
        fullName: 'New User',
        role: UserRole.USER 
      };
      
      mockedAxios.post.mockRejectedValueOnce(new Error('Network error'));
      
      const userService = UserService.getInstance();
      const result = await userService.createUser(newUser);
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(result.isMock).toBe(true);
      expect(result.username).toBe(newUser.username);
      expect(result.email).toBe(newUser.email);
    });
  });

  describe('updateUser', () => {
    it('should update a user through API', async () => {
      const updateData = { 
        email: 'updated@example.com', 
        fullName: 'Updated User',
        role: UserRole.MANAGER 
      };
      
      const updatedUser = { 
        id: '1', 
        username: 'test1', 
        email: 'updated@example.com',
        fullName: 'Updated User',
        role: UserRole.MANAGER,
        isActive: true,
        createdAt: '2023-01-01T00:00:00Z',
        lastLogin: '2023-07-15T08:30:00Z'
      };
      
      mockedAxios.put.mockResolvedValueOnce({ data: updatedUser });
      
      const userService = UserService.getInstance();
      const result = await userService.updateUser('1', updateData);
      
      expect(mockedAxios.put).toHaveBeenCalledTimes(1);
      expect(mockedAxios.put).toHaveBeenCalledWith(
        expect.stringContaining('/1'),
        updateData
      );
      expect(result).toEqual(updatedUser);
    });

    it('should update a mock user when API fails', async () => {
      const updateData = { 
        email: 'updated@example.com', 
        fullName: 'Updated User' 
      };
      
      mockedAxios.put.mockRejectedValueOnce(new Error('Network error'));
      
      // Get a reference to the mock users
      const mockUsers = await UserService.getInstance().getUsers();
      const userToUpdate = mockUsers[0];
      
      const userService = UserService.getInstance();
      const result = await userService.updateUser(userToUpdate.id, updateData);
      
      expect(mockedAxios.put).toHaveBeenCalledTimes(1);
      expect(result.email).toBe(updateData.email);
      expect(result.fullName).toBe(updateData.fullName);
    });

    it('should throw error when user not found', async () => {
      const updateData = { email: 'updated@example.com' };
      
      mockedAxios.put.mockRejectedValueOnce(new Error('User not found'));
      
      const userService = UserService.getInstance();
      
      await expect(userService.updateUser('999', updateData))
        .rejects
        .toThrow('User with ID \'999\' not found');
    });
  });

  describe('deleteUser', () => {
    it('should delete a user through API', async () => {
      mockedAxios.delete.mockResolvedValueOnce({});
      
      const userService = UserService.getInstance();
      const result = await userService.deleteUser('1');
      
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
      expect(mockedAxios.delete).toHaveBeenCalledWith(expect.stringContaining('/1'));
      expect(result).toBe(true);
    });

    it('should delete a mock user when API fails', async () => {
      mockedAxios.delete.mockRejectedValueOnce(new Error('Network error'));
      
      // Get a reference to the mock users
      const mockUsers = await UserService.getInstance().getUsers();
      const userToDelete = mockUsers[0];
      const initialCount = mockUsers.length;
      
      const userService = UserService.getInstance();
      const result = await userService.deleteUser(userToDelete.id);
      
      // Get users again to check if one was removed
      const updatedMockUsers = await userService.getUsers();
      
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
      expect(result).toBe(true);
      expect(updatedMockUsers.length).toBe(initialCount - 1);
    });

    it('should return false when user not found', async () => {
      mockedAxios.delete.mockRejectedValueOnce(new Error('User not found'));
      
      const userService = UserService.getInstance();
      const result = await userService.deleteUser('999');
      
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
      expect(result).toBe(false);
    });
  });

  describe('changePassword', () => {
    it('should change a user password through API', async () => {
      mockedAxios.post.mockResolvedValueOnce({});
      
      const userService = UserService.getInstance();
      const result = await userService.changePassword('1', 'oldpassword', 'newpassword');
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(mockedAxios.post).toHaveBeenCalledWith(
        expect.stringContaining('/1/password'),
        {
          currentPassword: 'oldpassword',
          newPassword: 'newpassword'
        }
      );
      expect(result).toBe(true);
    });

    it('should return true even when API fails in mock mode', async () => {
      mockedAxios.post.mockRejectedValueOnce(new Error('Network error'));
      
      const userService = UserService.getInstance();
      const result = await userService.changePassword('1', 'oldpassword', 'newpassword');
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(result).toBe(true);
    });
  });

  describe('resetPassword', () => {
    it('should reset a user password through API', async () => {
      mockedAxios.post.mockResolvedValueOnce({});
      
      const userService = UserService.getInstance();
      const result = await userService.resetPassword('1', 'newpassword');
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(mockedAxios.post).toHaveBeenCalledWith(
        expect.stringContaining('/1/reset-password'),
        {
          newPassword: 'newpassword'
        }
      );
      expect(result).toBe(true);
    });

    it('should return true even when API fails in mock mode', async () => {
      mockedAxios.post.mockRejectedValueOnce(new Error('Network error'));
      
      const userService = UserService.getInstance();
      const result = await userService.resetPassword('1', 'newpassword');
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(result).toBe(true);
    });
  });

  describe('hasPermission', () => {
    const testUser: User = {
      id: '1',
      username: 'testuser',
      email: 'test@example.com',
      fullName: 'Test User',
      role: UserRole.USER,
      isActive: true,
      createdAt: '2023-01-01T00:00:00Z'
    };

    it('should return true for admin regardless of permission', () => {
      const adminUser = { ...testUser, role: UserRole.ADMIN };
      
      expect(UserService.hasPermission(adminUser, Permission.SYSTEM_ADMIN)).toBe(true);
      expect(UserService.hasPermission(adminUser, Permission.STORAGE_ADMIN)).toBe(true);
      expect(UserService.hasPermission(adminUser, Permission.USER_ADMIN)).toBe(true);
    });

    it('should check role-based permissions for regular users', () => {
      const userPermissions = RolePermissions[UserRole.USER];
      
      // Should return true for permissions assigned to USER role
      for (const permission of userPermissions) {
        expect(UserService.hasPermission(testUser, permission)).toBe(true);
      }
      
      // Should return false for admin permissions
      expect(UserService.hasPermission(testUser, Permission.SYSTEM_ADMIN)).toBe(false);
      expect(UserService.hasPermission(testUser, Permission.STORAGE_ADMIN)).toBe(false);
      expect(UserService.hasPermission(testUser, Permission.USER_ADMIN)).toBe(false);
    });

    it('should respect custom permissions', () => {
      const userWithCustomPermissions = {
        ...testUser,
        customPermissions: [Permission.STORAGE_ADMIN]
      };
      
      // Should have standard USER permissions
      expect(UserService.hasPermission(userWithCustomPermissions, Permission.SYSTEM_MONITOR)).toBe(true);
      
      // Should have custom permission not normally in USER role
      expect(UserService.hasPermission(userWithCustomPermissions, Permission.STORAGE_ADMIN)).toBe(true);
      
      // Should still not have other admin permissions
      expect(UserService.hasPermission(userWithCustomPermissions, Permission.SYSTEM_ADMIN)).toBe(false);
    });
  });
}); 