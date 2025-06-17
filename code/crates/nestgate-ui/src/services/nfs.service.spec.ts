import { NfsService } from './nfs.service';

// Mock the Tauri API
jest.mock('@tauri-apps/api/tauri', () => ({
  invoke: jest.fn(),
}), { virtual: true });

describe('NfsService', () => {
  const mockExports = [
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

  const mockStatus = {
    status: 'running',
    version: '2.5.3',
    exports: 7,
    clients: 4,
    uptime: '3d 5h 23m'
  };

  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('getExports', () => {
    it('should return all NFS exports', async () => {
      const exports = await NfsService.getExports();
      expect(exports.length).toBeGreaterThan(0);
      expect(exports[0]).toHaveProperty('id');
      expect(exports[0]).toHaveProperty('path');
      expect(exports[0]).toHaveProperty('clients');
      expect(exports[0]).toHaveProperty('options');
    });
  });

  describe('getExport', () => {
    it('should return a specific NFS export by ID', async () => {
      const exportId = '1';
      const exportItem = await NfsService.getExport(exportId);
      expect(exportItem).toHaveProperty('id', exportId);
    });

    it('should throw error for nonexistent export', async () => {
      const exportId = 'nonexistent';
      await expect(NfsService.getExport(exportId)).rejects.toThrow();
    });
  });

  describe('createExport', () => {
    it('should create a new NFS export', async () => {
      const newExport = {
        path: '/mnt/tank/new',
        clients: [
          { host: '192.168.1.300', options: {} }
        ],
        options: {
          rw: 'true',
          sync: 'true',
          no_root_squash: 'false',
          no_subtree_check: 'true'
        },
        enabled: true,
        description: 'New export'
      };

      const result = await NfsService.createExport(newExport);
      expect(result).toHaveProperty('id');
      expect(result.path).toBe(newExport.path);
      expect(result.description).toBe(newExport.description);
    });
  });

  describe('updateExport', () => {
    it('should update an existing NFS export', async () => {
      const exportId = '1';
      const updatedData = {
        description: 'Updated description'
      };

      const result = await NfsService.updateExport(exportId, updatedData);
      expect(result).toHaveProperty('id', exportId);
      expect(result.description).toBe(updatedData.description);
    });
  });

  describe('deleteExport', () => {
    it('should delete an NFS export', async () => {
      const exportId = '1';
      await expect(NfsService.deleteExport(exportId)).resolves.not.toThrow();
    });
  });

  describe('getStatus', () => {
    it('should get NFS service status', async () => {
      const status = await NfsService.getStatus();
      expect(status).toHaveProperty('status');
      expect(status).toHaveProperty('version');
      expect(status).toHaveProperty('exports');
      expect(status).toHaveProperty('clients');
      expect(status).toHaveProperty('uptime');
    });
  });

  describe('startService', () => {
    it('should start the NFS service', async () => {
      const result = await NfsService.startService();
      expect(result).toHaveProperty('status', 'running');
    });
  });

  describe('stopService', () => {
    it('should stop the NFS service', async () => {
      const result = await NfsService.stopService();
      expect(result).toHaveProperty('status', 'stopped');
    });
  });

  describe('restartService', () => {
    it('should restart the NFS service', async () => {
      const result = await NfsService.restartService();
      expect(result).toHaveProperty('status', 'running');
    });
  });
}); 