// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom';

// Mock window.matchMedia - this is needed for Ant Design components
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(), // Deprecated
    removeListener: jest.fn(), // Deprecated
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});

// Mock for ResizeObserver
global.ResizeObserver = jest.fn().mockImplementation(() => ({
  observe: jest.fn(),
  unobserve: jest.fn(),
  disconnect: jest.fn(),
}));

// Mock for Ant Design Grid responsive observer
jest.mock('antd/lib/_util/responsiveObserver', () => {
  return {
    __esModule: true,
    default: {
      subscribe: jest.fn(() => jest.fn()),
      unsubscribe: jest.fn(),
      register: jest.fn(),
      responsiveMap: {
        xs: '(max-width: 575px)',
        sm: '(min-width: 576px)',
        md: '(min-width: 768px)',
        lg: '(min-width: 992px)',
        xl: '(min-width: 1200px)',
        xxl: '(min-width: 1600px)',
      },
    },
  };
});

// Mock IntersectionObserver
window.IntersectionObserver = jest.fn().mockImplementation(() => ({
  root: null,
  rootMargin: '',
  thresholds: [],
  observe: jest.fn(),
  unobserve: jest.fn(),
  disconnect: jest.fn(),
  takeRecords: jest.fn().mockReturnValue([]),
})) as any;

// Mock Ant Design completely
jest.mock('antd', () => require('./__mocks__/antd'));

// Enhanced Ant Design component mocks - use mockReactModule approach
jest.mock('antd', () => {
  // Using a different approach to avoid React reference in the mock factory
  const mockTabNav = (props: any) => {
    return {
      type: 'div',
      props: {
        'data-testid': 'mock-tabs-nav',
        key: 'nav',
        children: props.items?.map((item: any) => ({
          type: 'button',
          props: {
            key: item.key,
            'data-tab-key': item.key,
            'data-testid': `tab-${item.key}`,
            'aria-selected': item.key === props.defaultActiveKey ? 'true' : 'false',
            onClick: () => props.onChange?.(item.key),
            children: item.label,
          },
        })),
      },
    };
  };

  const mockTabContent = (props: any) => {
    return {
      type: 'div',
      props: {
        'data-testid': 'mock-tabs-content',
        key: 'content',
        children: props.items?.find((item: any) => item.key === props.defaultActiveKey)?.children,
      },
    };
  };

  const mockTabs = (props: any) => {
    return {
      type: 'div',
      props: {
        'data-testid': 'mock-tabs',
        children: [mockTabNav(props), mockTabContent(props)],
      },
    };
  };

  mockTabs.TabPane = (props: any) => {
    return {
      type: 'div',
      props: {
        children: props.children,
      },
    };
  };

  const mockSelect = (props: any) => {
    const optionElements = props.options
      ? props.options.map((opt: any) => ({
          type: 'option',
          props: {
            key: opt.value,
            value: opt.value,
            children: opt.label,
          },
        }))
      : Array.isArray(props.children)
        ? props.children.map((child: any) => ({
            type: 'option',
            props: {
              key: child.props.value,
              value: child.props.value,
              children: child.props.children,
            },
          }))
        : [];

    return {
      type: 'select',
      props: {
        ...props,
        'data-testid': 'select',
        children: optionElements,
      },
    };
  };

  mockSelect.Option = (props: any) => {
    return {
      type: 'option',
      props: {
        value: props.value,
        children: props.children,
      },
    };
  };

  return {
    __esModule: true,
    ...jest.requireActual('antd'),
    // Components as functions that return JSX-like objects
    Tabs: mockTabs,
    Typography: {
      Title: (props: any) => ({
        type: `h${props.level || 1}`,
        props: {
          children: props.children,
        },
      }),
      Paragraph: (props: any) => ({
        type: 'p',
        props: {
          children: props.children,
        },
      }),
    },
    Form: {
      Item: (props: any) => ({
        type: 'div',
        props: {
          className: 'form-item',
          children: [
            props.label && {
              type: 'label',
              key: 'label',
              props: {
                htmlFor: props.name,
                children: props.label,
              },
            },
            {
              type: 'div',
              key: 'field',
              props: {
                children: props.children,
              },
            },
          ].filter(Boolean),
        },
      }),
      useForm: () => [{ resetFields: jest.fn() }],
    },
    Input: (props: any) => ({
      type: 'input',
      props: {
        ...props,
      },
    }),
    Select: mockSelect,
    Button: (props: any) => ({
      type: 'button',
      props: {
        type: props.htmlType,
        'data-type': props.type,
        'data-loading': props.loading,
        onClick: props.onClick,
        children: [
          props.icon && {
            type: 'span',
            key: 'icon',
            props: {
              children: props.icon,
            },
          },
          props.children,
        ].filter(Boolean),
      },
    }),
    Card: (props: any) => ({
      type: 'div',
      props: {
        className: 'ant-card',
        children: [
          props.title && {
            type: 'div',
            key: 'header',
            props: {
              className: 'ant-card-head',
              children: props.title,
            },
          },
          {
            type: 'div',
            key: 'body',
            props: {
              className: 'ant-card-body',
              children: props.children,
            },
          },
        ].filter(Boolean),
      },
    }),
    Switch: (props: any) => ({
      type: 'input',
      props: {
        type: 'checkbox',
        checked: props.checked,
        onChange: (e: any) => props.onChange?.(e.target.checked),
      },
    }),
    Space: (props: any) => ({
      type: 'div',
      props: {
        className: 'ant-space',
        style: {
          display: 'flex',
          flexDirection: props.direction === 'vertical' ? 'column' : 'row',
        },
        children: Array.isArray(props.children)
          ? props.children.map((child: any, index: number) => ({
              type: 'div',
              props: {
                className: 'ant-space-item',
                key: index,
                children: child,
              },
            }))
          : props.children
              ? {
                  type: 'div',
                  props: {
                    className: 'ant-space-item',
                    children: props.children,
                  },
                }
              : null,
      },
    }),
    Progress: (props: any) => ({
      type: 'div',
      props: {
        className: 'ant-progress',
        role: 'progressbar',
        'aria-valuenow': props.percent,
        'aria-valuemin': 0,
        'aria-valuemax': 100,
        style: {
          backgroundColor: props.strokeColor,
        },
        children: props.showInfo
          ? {
              type: 'span',
              props: {
                className: 'ant-progress-text',
                children: `${props.percent}%`,
              },
            }
          : null,
      },
    }),
    Row: (props: any) => ({
      type: 'div',
      props: {
        className: 'ant-row',
        style: {
          marginLeft: props.gutter ? -props.gutter[0] / 2 : 0,
          marginRight: props.gutter ? -props.gutter[0] / 2 : 0,
          ...props.style,
        },
        children: props.children,
      },
    }),
    Col: (props: any) => ({
      type: 'div',
      props: {
        className: `ant-col ant-col-${props.span}`,
        style: props.style,
        children: props.children,
      },
    }),
  };
});

// Mock Ant Design icons using a simple factory approach
const createIconMock = (iconName: string) => {
  return function MockIcon() {
    return {
      type: 'span',
      props: {
        'data-icon': iconName,
        children: null,
      },
    };
  };
};

jest.mock('@ant-design/icons', () => ({
  __esModule: true,
  default: {},
  DashboardOutlined: createIconMock('dashboard'),
  SettingOutlined: createIconMock('setting'),
  UserOutlined: createIconMock('user'),
  LockOutlined: createIconMock('lock'),
  MenuOutlined: createIconMock('menu'),
  BellOutlined: createIconMock('bell'),
  DatabaseOutlined: createIconMock('database'),
  HomeOutlined: createIconMock('home'),
  PieChartOutlined: createIconMock('pie-chart'),
  DownOutlined: createIconMock('down'),
  FileOutlined: createIconMock('file'),
  SaveOutlined: createIconMock('save'),
  ReloadOutlined: createIconMock('reload'),
  CloudSyncOutlined: createIconMock('cloud-sync'),
  CloudOutlined: createIconMock('cloud'),
  MailOutlined: createIconMock('mail'),
  QuestionCircleOutlined: createIconMock('question-circle'),
  UploadOutlined: createIconMock('upload'),
  ClockCircleOutlined: createIconMock('clock-circle'),
  FileProtectOutlined: createIconMock('file-protect'),
  GlobalOutlined: createIconMock('global'),
  SafetyOutlined: createIconMock('safety'),
  createFromIconfontCN: () => createIconMock('custom'),
}));

// Mock scrollTo with proper typing
window.scrollTo = jest.fn((x?: any, y?: any) => {}) as unknown as typeof window.scrollTo;

// Global mocks for chart libraries if needed
jest.mock('recharts', () => ({
  LineChart: () => null,
  Line: () => null,
  XAxis: () => null,
  YAxis: () => null,
  CartesianGrid: () => null,
  Tooltip: () => null,
  Legend: () => null,
  ResponsiveContainer: (props: { children: any }) => props.children,
  PieChart: () => null,
  Pie: () => null,
  Cell: () => null,
  BarChart: () => null,
  Bar: () => null,
}));

// Mock for Tauri API
jest.mock('@tauri-apps/api/tauri', () => ({
  invoke: jest.fn().mockResolvedValue({}),
}), { virtual: true });

jest.mock('@tauri-apps/api/http', () => ({
  fetch: jest.fn().mockResolvedValue({ data: {} }),
}), { virtual: true }); 