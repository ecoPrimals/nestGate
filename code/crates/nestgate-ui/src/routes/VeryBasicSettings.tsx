import React from 'react';

const VeryBasicSettings: React.FC = () => {
  return (
    <div>
      <h1>System Settings</h1>
      <p>Configure system-wide settings for your NestGate storage system.</p>
      
      <div>
        <h2>General Settings</h2>
        <div>
          <h3>System Configuration</h3>
          <p>General settings content would go here</p>
        </div>
      </div>
      
      <div>
        <h2>Network Settings</h2>
        <div>
          <h3>Network Configuration</h3>
          <p>Network settings content would go here</p>
        </div>
      </div>
      
      <div>
        <h2>User Management</h2>
        <div>
          <h3>User Management</h3>
          <p>User management content would go here</p>
        </div>
      </div>
    </div>
  );
};

export default VeryBasicSettings; 