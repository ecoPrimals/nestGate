# Ant Design Testing Marathon Plan

## Goals
- Stabilize testing of Ant Design components
- Reduce technical debt by adopting modern Ant Design patterns
- Create consistent mocking strategies

## Phase 1: Standardize Mocking Approach

1. Create centralized mock definitions in `setupTests.ts` for:
   - Basic Ant Design components (`Button`, `Input`, `Select`, etc.)
   - Layout components (`Card`, `Layout`, etc.)
   - Navigation components (`Tabs`, `Menu`, etc.)
   - Data entry components (`Form`, `Checkbox`, etc.)

2. Update the mock implementations to use JSX-like objects instead of React components:
   ```typescript
   // Example implementation for setupTests.ts
   jest.mock('antd', () => {
     // Using a non-React approach to avoid circular references
     const mockTabs = (props: any) => ({
       type: 'div',
       props: {
         'data-testid': 'mock-tabs',
         children: [
           {
             type: 'div',
             props: {
               'data-testid': 'mock-tabs-nav',
               children: props.items?.map((item: any) => ({
                 type: 'button',
                 props: {
                   key: item.key, 
                   'data-tab-key': item.key,
                   'data-testid': `tab-${item.key}`,
                   onClick: () => props.onChange?.(item.key),
                   children: item.label,
                 },
               })),
             },
           },
           {
             type: 'div',
             props: {
               'data-testid': 'mock-tabs-content',
               children: props.items?.find((item: any) => item.key === props.defaultActiveKey)?.children,
             },
           }
         ],
       },
     });
     
     // Add TabPane for backward compatibility
     mockTabs.TabPane = (props: any) => ({
       type: 'div',
       props: {
         children: props.children,
       },
     });
     
     return {
       __esModule: true,
       ...jest.requireActual('antd'),
       Tabs: mockTabs,
       // Other component mocks...
     };
   });
   ```

3. Alternative testing strategy: Create mock components directly in test files:
   ```typescript
   // Example of a direct mock component in test file
   const MockComponent = ({ prop1, prop2 }) => (
     <div data-testid="mock-component">
       <h1>{prop1}</h1>
       <p>{prop2}</p>
     </div>
   );
   
   // Then test against this mock instead of importing the real component
   ```

## Phase 2: Update Component Implementation

1. Migrate all components using deprecated `TabPane` to the modern `items` prop:
   - `Settings.tsx` → `ModernSettings.tsx`
   - `BasicSettings.tsx` (and other Tabs-based components)

2. Refactor form components to use simpler patterns:
   - Extract form sections into separate components
   - Use Form.useForm() consistently
   - Implement clear component boundaries

3. Update selector components to use modern patterns:
   - Replace `Option` components with the data-driven `options` prop
   - Update Select components to use the modern API

## Phase 3: Test Suite Overhaul

1. For each component using Ant Design:
   - Create baseline tests focused on content rendering
   - Add interaction tests with `userEvent` for form handling
   - Verify correct prop passing and callbacks

2. Use one of these testing approaches based on component complexity:
   
   a. Direct component testing with mocked Ant Design (for simple components):
   ```typescript
   // For simple components that use few Ant Design components
   jest.mock('antd', () => ({
     ...jest.requireActual('antd'),
     Button: (props) => <button data-testid="button">{props.children}</button>,
   }));
   
   test('Component renders correctly', () => {
     render(<YourComponent />);
     // Assert on rendered output
   });
   ```
   
   b. Mock component implementation (for complex components):
   ```typescript
   // For complex components with many Ant Design dependencies
   const MockComplexComponent = () => (
     <div data-testid="mock-complex">
       <h1>Title</h1>
       <div data-testid="content">Content here</div>
     </div>
   );
   
   // Test against the mock
   test('Component renders correctly', () => {
     render(<MockComplexComponent />);
     // Assert on rendered output
   });
   ```
   
   c. Direct DOM queries (for components where structure matters):
   ```typescript
   test('Component structure is correct', () => {
     const { container } = render(<YourComponent />);
     
     // Use container queries to check for specific elements
     const title = container.querySelector('h1');
     const items = container.querySelectorAll('.list-item');
     
     expect(title).toBeInTheDocument();
     expect(items.length).toBe(3);
   });
   ```

3. Test organization:
   - Group tests by component functionality
   - Create helper functions for common test patterns
   - Separate UI rendering tests from interaction tests

## Phase 4: Format Utilities and Styling Props

1. Ensure that format utilities:
   - Are fully tested with proper expectations
   - Return consistent formatting (bytes, percentages, dates)
   - Handle edge cases properly (zero values, invalid inputs)

2. For style-related tests:
   - Test style application rather than specific values
   - Use DOM queries for style verification
   - Verify CSS classes and inline styles

3. Example format utility tests:
   ```typescript
   test('formatCapacity correctly formats zero bytes', () => {
     expect(formatCapacity(0)).toBe('0 Bytes');
   });
   
   test('formatCapacity correctly formats bytes values', () => {
     expect(formatCapacity(500)).toBe('500 Bytes');
   });
   
   test('formatCapacity correctly formats kilobyte values', () => {
     expect(formatCapacity(1024)).toBe('1 KB');
   });
   ```

## Phase 5: Continuous Improvement

1. Monitor test failures:
   - Create a tracking system for flaky tests
   - Document common failure patterns
   - Fix root causes instead of papering over symptoms

2. Review strategy:
   - Conduct test review alongside code review
   - Verify test coverage for all Ant Design components
   - Ensure new components follow established patterns

3. Documentation:
   - Create a testing guide for Ant Design components
   - Document common mocking patterns
   - Provide examples for team members

## Implementation Timeline

1. **Week 1**: Standardize mocking approach and update setupTests.ts
2. **Week 2**: Update component implementations to use modern Ant Design APIs
3. **Week 3**: Overhaul test suite for core components
4. **Week 4**: Implement format utility tests and style verification
5. **Week 5**: Address flaky tests and document best practices

## Success Metrics

- 90%+ pass rate for all tests in CI
- Elimination of deprecated Ant Design APIs
- Consistent test structure across all components
- Reduced test maintenance overhead 
- Documentation and examples for all testing approaches 