import React from 'react';
import StatusChip from './StatusChip';
import { DataSourceType } from '../../utils/env';

interface DataSourceIndicatorProps {
  dataSource?: DataSourceType;
  showLabel?: boolean;
  size?: 'small' | 'medium';
}

/**
 * A component that displays a visual indicator for the data source type
 */
const DataSourceIndicator: React.FC<DataSourceIndicatorProps> = ({
  dataSource = DataSourceType.LIVE,
  showLabel = true,
  size = 'medium'
}) => {
  // Determine status and label based on data source type
  let status: string;
  let label: string;
  
  switch (dataSource) {
    case DataSourceType.LIVE:
      status = 'success';
      label = 'LIVE';
      break;
    case DataSourceType.MOCK:
      status = 'warning';
      label = 'MOCK';
      break;
    case DataSourceType.PLACEHOLDER:
      status = 'info';
      label = 'COMING SOON';
      break;
    case DataSourceType.TEST:
      status = 'secondary';
      label = 'TEST';
      break;
    default:
      status = 'default';
      label = 'UNKNOWN';
  }
  
  return (
    <StatusChip 
      status={status} 
      label={showLabel ? label : ''} 
      size={size}
    />
  );
};

export default DataSourceIndicator; 