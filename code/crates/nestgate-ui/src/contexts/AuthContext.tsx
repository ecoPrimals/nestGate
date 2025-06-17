import React, { createContext, useState, useEffect, ReactNode } from 'react';
import { User } from '../services/user.service';
import { AuthService, UserPreferences } from '../services/auth.service';

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (username: string, password: string) => Promise<boolean>;
  logout: () => void;
  error: string | null;
  updateProfile: (profileData: Partial<{ email: string; preferences: UserPreferences }>) => Promise<boolean>;
  changePassword: (currentPassword: string, newPassword: string) => Promise<boolean>;
  validatePassword: (password: string) => { valid: boolean; errors: string[] };
  refreshSession: () => Promise<boolean>;
  checkPasswordStrength: (password: string) => { score: number; feedback: string };
}

// Create context with default values
export const AuthContext = createContext<AuthContextType>({
  user: null,
  isAuthenticated: false,
  isLoading: true,
  login: async () => false,
  logout: () => {},
  error: null,
  updateProfile: async () => false,
  changePassword: async () => false,
  validatePassword: () => ({ valid: false, errors: [] }),
  refreshSession: async () => false,
  checkPasswordStrength: () => ({ score: 0, feedback: '' }),
});

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  
  // Check if user is logged in on component mount
  useEffect(() => {
    const checkAuthentication = async () => {
      setIsLoading(true);
      try {
        // Check if there's a stored token
        const isLoggedIn = AuthService.isAuthenticated();
        
        if (isLoggedIn) {
          // Get current user info
          const userInfo = AuthService.getUser();
          setUser(userInfo as User);
          setIsAuthenticated(true);
        } else {
          setUser(null);
          setIsAuthenticated(false);
        }
      } catch (err) {
        console.error('Error checking authentication:', err);
        setUser(null);
        setIsAuthenticated(false);
      } finally {
        setIsLoading(false);
      }
    };
    
    checkAuthentication();
  }, []);
  
  // Handle login
  const login = async (username: string, password: string): Promise<boolean> => {
    setIsLoading(true);
    setError(null);
    
    try {
      const loginResult = await AuthService.login(username, password);
      
      if (loginResult) {
        const userInfo = AuthService.getUser();
        setUser(userInfo as User);
        setIsAuthenticated(true);
        return true;
      } else {
        setUser(null);
        setIsAuthenticated(false);
        setError('Invalid username or password');
        return false;
      }
    } catch (err) {
      console.error('Login error:', err);
      setUser(null);
      setIsAuthenticated(false);
      setError('An error occurred during login');
      return false;
    } finally {
      setIsLoading(false);
    }
  };
  
  // Handle logout
  const logout = () => {
    setIsLoading(true);
    
    try {
      AuthService.logout();
      setUser(null);
      setIsAuthenticated(false);
    } catch (err) {
      console.error('Logout error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Update user profile
  const updateProfile = async (profileData: Partial<{ email: string; preferences: UserPreferences }>): Promise<boolean> => {
    setIsLoading(true);
    setError(null);
    
    try {
      if (!user) {
        throw new Error('User not authenticated');
      }
      
      const updatedUser = await AuthService.updateUserProfile(user.id, profileData);
      setUser(prevUser => ({ ...prevUser, ...updatedUser } as User));
      return true;
    } catch (err) {
      console.error('Profile update error:', err);
      setError('Failed to update profile');
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  // Change password
  const changePassword = async (currentPassword: string, newPassword: string): Promise<boolean> => {
    setIsLoading(true);
    setError(null);
    
    try {
      if (!user) {
        throw new Error('User not authenticated');
      }
      
      const success = await AuthService.changePassword(user.id, currentPassword, newPassword);
      if (!success) {
        setError('Failed to change password');
      }
      return success;
    } catch (err: any) {
      console.error('Password change error:', err);
      setError(err.message || 'Failed to change password');
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  // Validate password against policy
  const validatePassword = (password: string) => {
    return AuthService.validatePassword(password);
  };

  // Refresh session token
  const refreshSession = async (): Promise<boolean> => {
    setIsLoading(true);
    
    try {
      await AuthService.refreshToken();
      return true;
    } catch (err) {
      console.error('Session refresh error:', err);
      setIsAuthenticated(false);
      setUser(null);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  // Check password strength
  const checkPasswordStrength = (password: string): { score: number; feedback: string } => {
    // Simple password strength calculation
    // In a real app, you'd want to use a library like zxcvbn
    let score = 0;
    let feedback = 'Weak password';
    
    if (password.length >= 8) score += 1;
    if (password.length >= 12) score += 1;
    if (/[A-Z]/.test(password)) score += 1;
    if (/[a-z]/.test(password)) score += 1;
    if (/[0-9]/.test(password)) score += 1;
    if (/[^A-Za-z0-9]/.test(password)) score += 1;
    
    if (score === 6) feedback = 'Very strong password';
    else if (score >= 4) feedback = 'Strong password';
    else if (score >= 3) feedback = 'Medium strength password';
    else if (score >= 2) feedback = 'Weak password';
    else feedback = 'Very weak password';
    
    return { score, feedback };
  };
  
  // Provide auth context to children
  return (
    <AuthContext.Provider
      value={{
        user,
        isAuthenticated,
        isLoading,
        login,
        logout,
        error,
        updateProfile,
        changePassword,
        validatePassword,
        refreshSession,
        checkPasswordStrength,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}; 