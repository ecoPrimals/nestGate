export interface NfsExport {
  id: string;
  path: string;
  clients: NfsClient[];
  options: NfsExportOptions;
  enabled: boolean;
  description: string;
}

export interface NfsClient {
  host: string;
  options: Record<string, any>;
}

export interface NfsExportOptions {
  rw: string;
  sync: string;
  no_root_squash: string;
  no_subtree_check: string;
  [key: string]: string;
}

export class NfsService {
  static async getExports(): Promise<NfsExport[]> {
    return [
      {
        id: '1',
        path: '/mnt/tank/shared',
        clients: [
          { host: '192.168.1.100', options: {} },
          { host: '192.168.1.101', options: {} }
        ],
        options: {
          rw: 'true',
          sync: 'true',
          no_root_squash: 'true',
          no_subtree_check: 'true'
        },
        enabled: true,
        description: 'Main shared directory'
      },
      {
        id: '2',
        path: '/mnt/tank/backup',
        clients: [
          { host: '192.168.1.200', options: {} }
        ],
        options: {
          rw: 'false',
          sync: 'true',
          no_root_squash: 'false',
          no_subtree_check: 'true'
        },
        enabled: true,
        description: 'Backup directory'
      }
    ];
  }

  static async getExport(id: string): Promise<NfsExport> {
    const exports = await this.getExports();
    const exportItem = exports.find(e => e.id === id);
    if (!exportItem) {
      throw new Error(`Export with ID ${id} not found`);
    }
    return exportItem;
  }

  static async createExport(exportData: Omit<NfsExport, 'id'>): Promise<NfsExport> {
    return {
      id: '3',
      ...exportData
    };
  }

  static async updateExport(id: string, exportData: Partial<NfsExport>): Promise<NfsExport> {
    const exportItem = await this.getExport(id);
    return {
      ...exportItem,
      ...exportData
    };
  }

  static async deleteExport(id: string): Promise<void> {
    // Implementation would delete the export
    return;
  }

  static async getStatus(): Promise<{
    status: 'running' | 'stopped' | 'error';
    version: string;
    exports: number;
    clients: number;
    uptime: string;
  }> {
    return {
      status: 'running',
      version: '2.5.3',
      exports: 7,
      clients: 4,
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