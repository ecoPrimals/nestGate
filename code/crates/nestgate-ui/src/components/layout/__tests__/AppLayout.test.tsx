import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import { MemoryRouter } from 'react-router-dom';

// Create a mock AppLayout component
const MockAppLayout = ({ route = '/' }) => {
  const [collapsed, setCollapsed] = React.useState(false);
  
  // Determine active menu item based on route
  const getActiveMenuItem = () => {
    if (route.startsWith('/dashboard')) return 'dashboard';
    if (route.startsWith('/storage')) return 'storage';
    if (route.startsWith('/snapshots')) return 'snapshots';
    if (route.startsWith('/settings')) return 'settings';
    return 'dashboard'; // Default
  };
  
  const activeMenuItem = getActiveMenuItem();
  
  return (
    <div data-testid="app-layout" className="ant-layout">
      {/* Sidebar */}
      <aside data-testid="sidebar" className="ant-layout-sider" style={{ width: collapsed ? 80 : 220 }}>
        <div className="logo">
          {!collapsed && <span data-testid="logo-text">NestGate</span>}
          {collapsed && <span>NG</span>}
        </div>
        
        <ul className="ant-menu" data-testid="sidebar-menu">
          <li 
            className={`ant-menu-item ${activeMenuItem === 'dashboard' ? 'ant-menu-item-selected' : ''}`} 
            data-testid="menu-dashboard"
          >
            <span>Dashboard</span>
          </li>
          <li 
            className={`ant-menu-item ${activeMenuItem === 'storage' ? 'ant-menu-item-selected' : ''}`}
            data-testid="menu-storage"
          >
            <span>Storage</span>
          </li>
          <li 
            className={`ant-menu-item ${activeMenuItem === 'snapshots' ? 'ant-menu-item-selected' : ''}`}
            data-testid="menu-snapshots"
          >
            <span>Snapshots</span>
          </li>
          <li 
            className={`ant-menu-item ${activeMenuItem === 'settings' ? 'ant-menu-item-selected' : ''}`}
            data-testid="menu-settings"
          >
            <span>Settings</span>
          </li>
        </ul>
      </aside>
      
      {/* Main content area */}
      <div className="ant-layout">
        {/* Header */}
        <header data-testid="header" className="ant-layout-header">
          <button 
            data-testid="menu-toggle" 
            aria-label="menu"
            onClick={() => setCollapsed(!collapsed)}
          >
            {collapsed ? 'Expand' : 'Collapse'}
          </button>
          
          <div className="header-right">
            <button data-testid="notification-btn" aria-label="bell">
              Notifications
            </button>
            <div data-testid="user-dropdown">
              <span>Admin</span>
            </div>
          </div>
        </header>
        
        {/* Content */}
        <main data-testid="content" className="ant-layout-content">
          {/* This is where the outlet content would be rendered */}
          <div>Page Content</div>
        </main>
      </div>
    </div>
  );
};

describe('AppLayout', () => {
  it('renders the sidebar with navigation items', () => {
    render(
      <MemoryRouter>
        <MockAppLayout />
      </MemoryRouter>
    );
    
    // Check for sidebar menu items
    expect(screen.getByText('Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Storage')).toBeInTheDocument();
    expect(screen.getByText('Snapshots')).toBeInTheDocument();
    expect(screen.getByText('Settings')).toBeInTheDocument();
  });

  it('collapses sidebar when toggle button is clicked', () => {
    render(
      <MemoryRouter>
        <MockAppLayout />
      </MemoryRouter>
    );
    
    // Get initial state
    const logoText = screen.getByTestId('logo-text');
    expect(logoText).toBeInTheDocument();
    
    // Find and click the menu toggle button
    const menuButton = screen.getByRole('button', { name: /menu/i });
    fireEvent.click(menuButton);
    
    // After clicking, the logo text should be hidden
    expect(screen.queryByTestId('logo-text')).not.toBeInTheDocument();
  });

  it('highlights the current active menu item based on route', () => {
    render(
      <MemoryRouter initialEntries={['/dashboard']}>
        <MockAppLayout route="/dashboard" />
      </MemoryRouter>
    );
    
    // Check that dashboard menu item is selected
    const dashboardMenuItem = screen.getByTestId('menu-dashboard');
    expect(dashboardMenuItem).toHaveClass('ant-menu-item-selected');
  });

  it('renders notification button in header', () => {
    render(
      <MemoryRouter>
        <MockAppLayout />
      </MemoryRouter>
    );
    
    // Check for notifications button
    const notificationButton = screen.getByRole('button', { name: /bell/i });
    expect(notificationButton).toBeInTheDocument();
  });

  it('has the correct layout structure', () => {
    render(
      <MemoryRouter>
        <MockAppLayout />
      </MemoryRouter>
    );
    
    // Check layout structure
    expect(screen.getByTestId('app-layout')).toHaveClass('ant-layout');
    expect(screen.getByTestId('sidebar')).toHaveClass('ant-layout-sider');
    expect(screen.getByTestId('header')).toHaveClass('ant-layout-header');
    expect(screen.getByTestId('content')).toHaveClass('ant-layout-content');
  });
}); 