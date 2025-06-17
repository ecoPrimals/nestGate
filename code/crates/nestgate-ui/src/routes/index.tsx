import React from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider } from '../contexts/AuthContext';
import Dashboard from './Dashboard';
import Storage from './Storage';
import Snapshots from './Snapshots';
import Performance from './Performance';
import NetworkSettings from './NetworkSettings';
import Users from './Users';
import Login from './Login';
import NotFound from './NotFound';
import AppLayout from '../components/layout/AppLayout';
import Settings from './Settings';
import LiveDataExample from './examples/LiveDataExample';
import Monitoring from './Monitoring';

const AppRoutes = () => {
  return (
    <AuthProvider>
      <Routes>
        <Route path="/login" element={<Login />} />
        
        {/* Routes with layout */}
        <Route element={<AppLayout />}>
          <Route path="/" element={<Navigate to="/dashboard" replace />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/storage" element={<Storage />} />
          <Route path="/snapshots" element={<Snapshots />} />
          <Route path="/performance" element={<Performance />} />
          <Route path="/network" element={<NetworkSettings />} />
          <Route path="/users" element={<Users />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/monitoring" element={<Monitoring />} />
          <Route path="/examples/live-data" element={<LiveDataExample />} />
        </Route>
        
        {/* Not found */}
        <Route path="*" element={<NotFound />} />
      </Routes>
    </AuthProvider>
  );
};

export default AppRoutes; 