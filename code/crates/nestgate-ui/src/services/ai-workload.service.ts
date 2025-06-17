export interface WorkloadType {
  id: string;
  name: string;
  description: string;
}

export interface Recommendation {
  property: string;
  value: string;
  reason: string;
}

export interface WorkloadDetection {
  datasetId: string;
  workloadTypes: string[];
  confidence: number;
  recommendations: Recommendation[];
}

export interface PropertyChange {
  property: string;
  oldValue: string;
  newValue: string;
}

export interface OptimizationResult {
  datasetId: string;
  status: 'success' | 'error';
  changes: PropertyChange[];
}

export class AIWorkloadService {
  static async getWorkloadTypes(): Promise<WorkloadType[]> {
    return [
      {
        id: 'ml',
        name: 'Machine Learning',
        description: 'Optimized for machine learning workloads with large sequential reads and writes'
      },
      {
        id: 'db',
        name: 'Database',
        description: 'Optimized for database workloads with random I/O patterns and low latency requirements'
      },
      {
        id: 'file',
        name: 'File Server',
        description: 'Optimized for file server workloads with mixed read/write patterns'
      },
      {
        id: 'media',
        name: 'Media Streaming',
        description: 'Optimized for media streaming workloads with large sequential reads'
      }
    ];
  }

  static async detectWorkloads(datasetIds: string[]): Promise<WorkloadDetection[]> {
    return datasetIds.map(datasetId => ({
      datasetId,
      workloadTypes: ['ml'],
      confidence: 0.85,
      recommendations: [
        {
          property: 'recordsize',
          value: '128K',
          reason: 'Optimal for ML data access patterns'
        },
        {
          property: 'primarycache',
          value: 'metadata',
          reason: 'Reduce memory pressure'
        },
        {
          property: 'compression',
          value: 'lz4',
          reason: 'Good balance between compression ratio and performance'
        }
      ]
    }));
  }

  static async applyOptimizations(datasetIds: string[]): Promise<OptimizationResult[]> {
    return datasetIds.map(datasetId => ({
      datasetId,
      status: 'success',
      changes: [
        {
          property: 'recordsize',
          oldValue: '4K',
          newValue: '128K'
        },
        {
          property: 'primarycache',
          oldValue: 'all',
          newValue: 'metadata'
        },
        {
          property: 'compression',
          oldValue: 'off',
          newValue: 'lz4'
        }
      ]
    }));
  }
} 