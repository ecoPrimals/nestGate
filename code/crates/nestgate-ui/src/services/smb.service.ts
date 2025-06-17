export interface SmbShare {
  id: string;
  name: string;
  path: string;
  description: string;
  guest_ok: boolean;
  browsable: boolean;
  writable: boolean;
  recyclebin: boolean;
  shadow_copy: boolean;
  status: 'active' | 'inactive' | 'error';
  created_at: string;
  options: Record<string, string>;
}

export interface SmbUser {
  id: string;
  username: string;
  full_name: string;
  groups: string[];
  enabled: boolean;
}

export class SmbService {
  static async getShares(): Promise<SmbShare[]> {
    return [
      {
        id: '1',
        name: 'public',
        path: '/mnt/tank/public',
        description: 'Public share',
        guest_ok: true,
        browsable: true,
        writable: true,
        recyclebin: true,
        shadow_copy: false,
        status: 'active',
        created_at: '2023-01-15T10:00:00Z',
        options: {
          'create mask': '0755',
          'directory mask': '0755'
        }
      },
      {
        id: '2',
        name: 'private',
        path: '/mnt/tank/private',
        description: 'Private share',
        guest_ok: false,
        browsable: false,
        writable: true,
        recyclebin: true,
        shadow_copy: true,
        status: 'active',
        created_at: '2023-01-20T14:30:00Z',
        options: {
          'create mask': '0700',
          'directory mask': '0700',
          'valid users': '@staff'
        }
      }
    ];
  }

  static async getShare(id: string): Promise<SmbShare> {
    const shares = await this.getShares();
    const share = shares.find(s => s.id === id);
    if (!share) {
      throw new Error(`Share with ID ${id} not found`);
    }
    return share;
  }

  static async createShare(shareData: Omit<SmbShare, 'id' | 'created_at' | 'status'>): Promise<SmbShare> {
    return {
      id: '3',
      created_at: new Date().toISOString(),
      status: 'active',
      ...shareData
    };
  }

  static async updateShare(id: string, shareData: Partial<SmbShare>): Promise<SmbShare> {
    const share = await this.getShare(id);
    return {
      ...share,
      ...shareData
    };
  }

  static async deleteShare(id: string): Promise<void> {
    // Implementation would delete the share
    return;
  }

  static async getUsers(): Promise<SmbUser[]> {
    return [
      {
        id: '1',
        username: 'admin',
        full_name: 'Administrator',
        groups: ['administrators', 'users'],
        enabled: true
      },
      {
        id: '2',
        username: 'john',
        full_name: 'John Doe',
        groups: ['users', 'staff'],
        enabled: true
      }
    ];
  }

  static async getUser(id: string): Promise<SmbUser> {
    const users = await this.getUsers();
    const user = users.find(u => u.id === id);
    if (!user) {
      throw new Error(`User with ID ${id} not found`);
    }
    return user;
  }

  static async createUser(userData: Omit<SmbUser, 'id'>): Promise<SmbUser> {
    return {
      id: '3',
      ...userData
    };
  }

  static async updateUser(id: string, userData: Partial<SmbUser>): Promise<SmbUser> {
    const user = await this.getUser(id);
    return {
      ...user,
      ...userData
    };
  }

  static async deleteUser(id: string): Promise<void> {
    // Implementation would delete the user
    return;
  }

  static async getStatus(): Promise<{
    status: 'running' | 'stopped' | 'error';
    version: string;
    shares: number;
    connections: number;
    uptime: string;
  }> {
    return {
      status: 'running',
      version: '4.13.9',
      shares: 5,
      connections: 8,
      uptime: '3d 5h 23m'
    };
  }

  static async startService(): Promise<{ status: string }> {
    return { status: 'running' };
  }

  static async stopService(): Promise<{ status: string }> {
    return { status: 'stopped' };
  }

  static async restartService(): Promise<{ status: string }> {
    return { status: 'running' };
  }
} 