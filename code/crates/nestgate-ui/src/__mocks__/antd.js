const React = require('react');

// Mock components that cause issues in tests
const mockComponent = name => {
  const component = props => {
    return React.createElement(
      'div', 
      { 
        'data-testid': `mock-${name}`, 
        ...props,
        className: `ant-${name.toLowerCase()} ${props.className || ''}`
      }, 
      props.children
    );
  };
  component.displayName = name;
  return component;
};

// Create mock versions of all the components we need
const Progress = props => {
  let text = '';
  if (props.format && typeof props.format === 'function') {
    text = props.format(props.percent);
  } else {
    text = `${props.percent}%`;
  }
  
  return React.createElement(
    'div',
    { 
      'data-testid': 'mock-progress',
      className: `ant-progress ant-progress-${props.type || 'line'}`
    },
    React.createElement('span', {}, text)
  );
};

const TabPane = mockComponent('TabPane');

const Tabs = props => {
  const { activeKey, children } = props;
  return React.createElement(
    'div',
    {
      'data-testid': 'mock-tabs',
      className: 'ant-tabs'
    },
    React.createElement(
      'div',
      { className: 'ant-tabs-nav' },
      React.Children.map(children, child => 
        React.createElement(
          'div',
          { 
            className: `ant-tabs-tab ${child.props.tab === activeKey ? 'ant-tabs-tab-active' : ''}`,
            onClick: () => props.onChange && props.onChange(child.key)
          },
          child.props.tab
        )
      )
    ),
    React.createElement(
      'div',
      { className: 'ant-tabs-content' },
      React.Children.toArray(children).map(child => 
        React.cloneElement(child, { key: child.key })
      )
    )
  );
};

Tabs.TabPane = TabPane;

// Mock all other components
const components = [
  'Alert', 'Button', 'Card', 'Col', 'Divider', 'Drawer', 'Empty', 'List',
  'Popover', 'Row', 'Space', 'Statistic', 'Tag', 'Tooltip', 'Typography',
  'Badge', 'Dropdown', 'Menu', 'Modal'
];

const mocks = components.reduce((acc, name) => {
  acc[name] = mockComponent(name);
  return acc;
}, {});

// Typography components
const Typography = {
  Text: mockComponent('Typography.Text'),
  Title: mockComponent('Typography.Title'),
  Paragraph: mockComponent('Typography.Paragraph'),
  Link: mockComponent('Typography.Link')
};

// Icons
const IconComponents = [
  'LoadingOutlined', 'BellOutlined', 'CloseCircleOutlined', 'CheckCircleOutlined',
  'ExclamationCircleOutlined', 'InfoCircleOutlined', 'SyncOutlined', 'DeleteOutlined',
  'EyeOutlined', 'ClockCircleOutlined', 'DashboardOutlined', 'DesktopOutlined',
  'DatabaseOutlined', 'SwapOutlined', 'ArrowUpOutlined', 'ArrowDownOutlined',
  'ThunderboltOutlined', 'WarningOutlined', 'UserOutlined', 'SettingOutlined',
  'LogoutOutlined', 'MenuOutlined', 'HomeOutlined', 'AppstoreOutlined'
];

const icons = IconComponents.reduce((acc, name) => {
  acc[name] = mockComponent(name);
  return acc;
}, {});

module.exports = {
  ...mocks,
  Progress,
  Tabs,
  Typography,
  ...icons
}; 