/**
 * EnvironmentIndicator Component
 * 
 * Displays the current environment (development, production) and feature availability.
 * Useful for developers to quickly understand the current system state.
 */

import React, { useState } from 'react';
import { getConfig } from '../../config';
import { Feature, getFeatureStatus, getFeatureDescription } from '../../utils/featureFlags';

const config = getConfig();

interface EnvironmentIndicatorProps {
  expanded?: boolean;
}

/**
 * EnvironmentIndicator Component
 */
const EnvironmentIndicator: React.FC<EnvironmentIndicatorProps> = ({ 
  expanded = false 
}) => {
  const [isExpanded, setIsExpanded] = useState(expanded);
  
  // Check the current environment
  const isDevelopment = process.env.NODE_ENV === 'development';
  const isProduction = process.env.NODE_ENV === 'production';
  
  // Styles for the indicator
  const styles = {
    container: {
      position: 'fixed' as const,
      bottom: '10px',
      right: '10px',
      zIndex: 1000,
      display: 'flex',
      flexDirection: 'column' as const,
      alignItems: 'flex-end'
    },
    badge: {
      padding: '4px 8px',
      borderRadius: '4px',
      fontSize: '12px',
      fontWeight: 'bold' as const,
      color: 'white',
      cursor: 'pointer',
      userSelect: 'none' as const
    },
    developmentBadge: {
      backgroundColor: '#007bff',
    },
    productionBadge: {
      backgroundColor: '#28a745',
    },
    details: {
      marginTop: '5px',
      padding: '10px',
      backgroundColor: '#f8f9fa',
      border: '1px solid #dee2e6',
      borderRadius: '4px',
      fontSize: '12px',
      width: '300px',
      maxHeight: '400px',
      overflowY: 'auto' as const
    },
    heading: {
      fontSize: '14px',
      fontWeight: 'bold' as const,
      marginBottom: '5px'
    },
    section: {
      marginBottom: '10px'
    },
    item: {
      display: 'flex',
      justifyContent: 'space-between',
      marginBottom: '2px'
    },
    key: {
      fontWeight: 'bold' as const
    },
    value: {
      marginLeft: '10px'
    },
    featureEnabled: {
      color: '#28a745'
    },
    featurePreview: {
      color: '#ffc107'
    },
    featureDisabled: {
      color: '#dc3545'
    }
  };
  
  // Combine styles for badge
  const badgeStyles = {
    ...styles.badge,
    ...(isDevelopment ? styles.developmentBadge : styles.productionBadge)
  };
  
  // Toggle expanded state
  const toggleExpanded = () => {
    setIsExpanded(!isExpanded);
  };
  
  // Get style for feature status
  const getFeatureStatusStyle = (status: string) => {
    switch (status) {
      case 'enabled': return styles.featureEnabled;
      case 'preview': return styles.featurePreview;
      case 'disabled': return styles.featureDisabled;
      default: return {};
    }
  };
  
  return (
    <div style={styles.container}>
      <div 
        style={badgeStyles}
        onClick={toggleExpanded}
        role="button"
        aria-expanded={isExpanded}
      >
        {isDevelopment ? 'DEVELOPMENT' : 'PRODUCTION'}
      </div>
      
      {isExpanded && (
        <div style={styles.details}>
          <div style={styles.section}>
            <div style={styles.heading}>Environment</div>
            <div style={styles.item}>
              <span>NODE_ENV:</span>
              <span>{process.env.NODE_ENV}</span>
            </div>
            <div style={styles.item}>
              <span>Server Port:</span>
              <span>{config.SERVER_PORT}</span>
            </div>
            <div style={styles.item}>
              <span>API Port:</span>
              <span>{config.API_PORT}</span>
            </div>
            <div style={styles.item}>
              <span>UI Port:</span>
              <span>{config.UI_PORT}</span>
            </div>
          </div>
          
          <div style={styles.section}>
            <div style={styles.heading}>Features</div>
            {Object.values(Feature).map((feature) => {
              const status = getFeatureStatus(feature as Feature);
              const description = getFeatureDescription(feature as Feature);
              
              return (
                <div key={feature} style={styles.item}>
                  <span title={description}>{feature}:</span>
                  <span style={getFeatureStatusStyle(status)}>{status}</span>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};

export default EnvironmentIndicator; 