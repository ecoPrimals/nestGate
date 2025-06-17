import React from 'react';
import {
  Card,
  CardHeader,
  CardContent,
  Button,
  Box
} from '@mui/material';
import { Refresh as RefreshIcon } from '@mui/icons-material';
import { NetworkInterface } from '../../../services/telemetry.service';
import InterfacesTable from '../tables/InterfacesTable';
import EditInterfaceModal from '../modals/EditInterfaceModal';
import { NetworkInterfaceUpdate } from '../types';

interface InterfacesTabProps {
  interfaces: NetworkInterface[];
  isLoading: boolean;
  editModalVisible: boolean;
  currentInterface: NetworkInterface | null;
  onRefreshInterfaces: () => Promise<boolean>;
  onEditInterface: (networkInterface: NetworkInterface) => void;
  onToggleInterface: (networkInterface: NetworkInterface) => void;
  onSaveInterface: (values: NetworkInterfaceUpdate) => Promise<boolean>;
  onCloseEditModal: () => void;
}

const InterfacesTab: React.FC<InterfacesTabProps> = ({
  interfaces,
  isLoading,
  editModalVisible,
  currentInterface,
  onRefreshInterfaces,
  onEditInterface,
  onToggleInterface,
  onSaveInterface,
  onCloseEditModal
}) => {
  return (
    <>
      <Card>
        <CardHeader
          title="Network Interfaces"
          action={
            <Button
              variant="contained"
              startIcon={<RefreshIcon />}
              onClick={onRefreshInterfaces}
              disabled={isLoading}
            >
              Refresh
            </Button>
          }
        />
        <CardContent>
          <InterfacesTable
            interfaces={interfaces}
            isLoading={isLoading}
            onEditInterface={onEditInterface}
            onToggleInterface={onToggleInterface}
          />
        </CardContent>
      </Card>
      
      <EditInterfaceModal
        open={editModalVisible}
        interface={currentInterface}
        isLoading={isLoading}
        onClose={onCloseEditModal}
        onSave={onSaveInterface}
      />
    </>
  );
};

export default InterfacesTab; 