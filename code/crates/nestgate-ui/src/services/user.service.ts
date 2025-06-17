import axios from 'axios';
import { API_BASE_URL } from '../constants';
import { UserPreferences } from './auth.service';

/**
 * User role definitions
 */
export enum UserRole {
  ADMIN = 'admin',
  MANAGER = 'manager',
  USER = 'user',
  GUEST = 'guest'
}

/**
 * User permission definitions
 */
export enum Permission {
  // System permissions
  SYSTEM_ADMIN = 'system:admin',
  SYSTEM_MONITOR = 'system:monitor',
  SYSTEM_CONFIGURE = 'system:configure',
  
  // Storage permissions
  STORAGE_ADMIN = 'storage:admin',
  STORAGE_MANAGE = 'storage:manage',
  STORAGE_READ = 'storage:read',
  
  // User management permissions
  USER_ADMIN = 'user:admin',
  USER_MANAGE = 'user:manage',
  USER_READ = 'user:read',
  
  // Network permissions
  NETWORK_ADMIN = 'network:admin',
  NETWORK_MANAGE = 'network:manage',
  NETWORK_READ = 'network:read',
  
  // Backup permissions
  BACKUP_ADMIN = 'backup:admin',
  BACKUP_MANAGE = 'backup:manage',
  BACKUP_READ = 'backup:read'
}

/**
 * Default role permissions
 */
export const RolePermissions: Record<UserRole, Permission[]> = {
  [UserRole.ADMIN]: Object.values(Permission),
  [UserRole.MANAGER]: [
    Permission.SYSTEM_MONITOR, Permission.SYSTEM_CONFIGURE,
    Permission.STORAGE_MANAGE, Permission.STORAGE_READ,
    Permission.USER_MANAGE, Permission.USER_READ,
    Permission.NETWORK_MANAGE, Permission.NETWORK_READ,
    Permission.BACKUP_MANAGE, Permission.BACKUP_READ
  ],
  [UserRole.USER]: [
    Permission.SYSTEM_MONITOR,
    Permission.STORAGE_READ,
    Permission.USER_READ,
    Permission.NETWORK_READ,
    Permission.BACKUP_READ
  ],
  [UserRole.GUEST]: [
    Permission.SYSTEM_MONITOR,
    Permission.STORAGE_READ
  ]
};

/**
 * User interface
 */
export interface User {
  id: string;
  username: string;
  email: string;
  fullName: string;
  role: UserRole;
  isActive: boolean;
  createdAt: string;
  lastLogin?: string;
  customPermissions?: Permission[];
  isMock?: boolean;
  preferences?: UserPreferences;
}

/**
 * User creation parameters
 */
export interface CreateUserParams {
  username: string;
  password: string;
  email: string;
  fullName: string;
  role: UserRole;
  customPermissions?: Permission[];
}

/**
 * User update parameters
 */
export interface UpdateUserParams {
  email?: string;
  fullName?: string;
  role?: UserRole;
  isActive?: boolean;
  password?: string;
  customPermissions?: Permission[];
}

/**
 * Service for user management
 */
export class UserService {
  private static readonly API_URL = `${API_BASE_URL}/api/users`;
  private static instance: UserService;
  
  // Mock users for development
  private static mockUsers: User[] = [
    {
      id: '1',
      username: 'admin',
      email: 'admin@nestgate.io',
      fullName: 'System Administrator',
      role: UserRole.ADMIN,
      isActive: true,
      createdAt: '2023-01-01T00:00:00Z',
      lastLogin: '2023-07-15T08:30:00Z',
      isMock: true
    },
    {
      id: '2',
      username: 'manager',
      email: 'manager@nestgate.io',
      fullName: 'Storage Manager',
      role: UserRole.MANAGER,
      isActive: true,
      createdAt: '2023-01-15T00:00:00Z',
      lastLogin: '2023-07-14T10:45:00Z',
      isMock: true
    },
    {
      id: '3',
      username: 'user',
      email: 'user@nestgate.io',
      fullName: 'Regular User',
      role: UserRole.USER,
      isActive: true,
      createdAt: '2023-02-01T00:00:00Z',
      lastLogin: '2023-07-10T14:20:00Z',
      isMock: true
    },
    {
      id: '4',
      username: 'guest',
      email: 'guest@nestgate.io',
      fullName: 'Guest User',
      role: UserRole.GUEST,
      isActive: true,
      createdAt: '2023-03-10T00:00:00Z',
      lastLogin: '2023-06-30T09:15:00Z',
      isMock: true
    },
    {
      id: '5',
      username: 'inactive',
      email: 'inactive@nestgate.io',
      fullName: 'Inactive User',
      role: UserRole.USER,
      isActive: false,
      createdAt: '2023-04-05T00:00:00Z',
      lastLogin: '2023-05-02T16:40:00Z',
      isMock: true
    }
  ];
  
  /**
   * Gets the singleton instance of the service
   */
  public static getInstance(): UserService {
    if (!UserService.instance) {
      UserService.instance = new UserService();
    }
    return UserService.instance;
  }
  
  /**
   * Get all users
   */
  public async getUsers(): Promise<User[]> {
    try {
      const response = await axios.get<User[]>(UserService.API_URL);
      return response.data;
    } catch (error) {
      console.error('Error fetching users:', error);
      // Return mock data in case of error
      return UserService.mockUsers;
    }
  }
  
  /**
   * Get a user by ID
   */
  public async getUserById(id: string): Promise<User | null> {
    try {
      const response = await axios.get<User>(`${UserService.API_URL}/${id}`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching user with ID '${id}':`, error);
      // Return mock user if available
      const mockUser = UserService.mockUsers.find(user => user.id === id);
      return mockUser || null;
    }
  }
  
  /**
   * Create a new user
   */
  public async createUser(params: CreateUserParams): Promise<User> {
    try {
      const response = await axios.post<User>(UserService.API_URL, params);
      return response.data;
    } catch (error) {
      console.error('Error creating user:', error);
      // For mock implementation, create a mock user with a generated ID
      const mockUser: User = {
        id: String(UserService.mockUsers.length + 1),
        username: params.username,
        email: params.email,
        fullName: params.fullName,
        role: params.role,
        isActive: true,
        createdAt: new Date().toISOString(),
        customPermissions: params.customPermissions,
        isMock: true
      };
      
      UserService.mockUsers.push(mockUser);
      return mockUser;
    }
  }
  
  /**
   * Update an existing user
   */
  public async updateUser(id: string, params: UpdateUserParams): Promise<User> {
    try {
      const response = await axios.put<User>(`${UserService.API_URL}/${id}`, params);
      return response.data;
    } catch (error) {
      console.error(`Error updating user with ID '${id}':`, error);
      // For mock implementation, update the mock user
      const userIndex = UserService.mockUsers.findIndex(user => user.id === id);
      if (userIndex !== -1) {
        const updatedUser = {
          ...UserService.mockUsers[userIndex],
          ...params
        };
        UserService.mockUsers[userIndex] = updatedUser;
        return updatedUser;
      }
      throw new Error(`User with ID '${id}' not found`);
    }
  }
  
  /**
   * Delete a user
   */
  public async deleteUser(id: string): Promise<boolean> {
    try {
      await axios.delete(`${UserService.API_URL}/${id}`);
      return true;
    } catch (error) {
      console.error(`Error deleting user with ID '${id}':`, error);
      // For mock implementation, remove the mock user
      const userIndex = UserService.mockUsers.findIndex(user => user.id === id);
      if (userIndex !== -1) {
        UserService.mockUsers.splice(userIndex, 1);
        return true;
      }
      return false;
    }
  }
  
  /**
   * Change a user's password
   */
  public async changePassword(id: string, currentPassword: string, newPassword: string): Promise<boolean> {
    try {
      await axios.post(`${UserService.API_URL}/${id}/password`, {
        currentPassword,
        newPassword
      });
      return true;
    } catch (error) {
      console.error(`Error changing password for user with ID '${id}':`, error);
      // For mock implementation, always return true
      return true;
    }
  }
  
  /**
   * Reset a user's password (admin function)
   */
  public async resetPassword(id: string, newPassword: string): Promise<boolean> {
    try {
      await axios.post(`${UserService.API_URL}/${id}/reset-password`, {
        newPassword
      });
      return true;
    } catch (error) {
      console.error(`Error resetting password for user with ID '${id}':`, error);
      // For mock implementation, always return true
      return true;
    }
  }
  
  /**
   * Check if a user has a specific permission
   */
  public static hasPermission(user: User, permission: Permission): boolean {
    // If the user is an admin, they have all permissions
    if (user.role === UserRole.ADMIN) {
      return true;
    }
    
    // Check custom permissions first if they exist
    if (user.customPermissions && user.customPermissions.includes(permission)) {
      return true;
    }
    
    // Check role-based permissions
    return RolePermissions[user.role].includes(permission);
  }
} 