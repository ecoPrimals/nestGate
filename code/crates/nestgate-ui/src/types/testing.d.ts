// Type definitions for testing utilities

declare namespace jest {
  interface Matchers<R> {
    toBeInTheDocument(): R;
    toHaveTextContent(text: string | RegExp): R;
    toHaveClass(className: string): R;
    toHaveStyle(style: Record<string, any>): R;
    toHaveValue(value: any): R;
    toHaveAttribute(attr: string, value?: any): R;
    toBeVisible(): R;
    toBeDisabled(): R;
    toBeEnabled(): R;
    toBeChecked(): R;
  }

  // Mock Functions
  type Mock<T = any, Y extends any[] = any> = {
    (...args: Y): T;
    mockClear(): void;
    mockReset(): void;
    mockRestore(): void;
    mockImplementation(fn: (...args: Y) => T): this;
    mockImplementationOnce(fn: (...args: Y) => T): this;
    mockReturnThis(): this;
    mockReturnValue(value: T): this;
    mockReturnValueOnce(value: T): this;
    mockResolvedValue(value: Awaited<T>): this;
    mockResolvedValueOnce(value: Awaited<T>): this;
    mockRejectedValue(value: any): this;
    mockRejectedValueOnce(value: any): this;
    getMockName(): string;
    mockName(name: string): this;
    mock: {
      calls: Y[];
      instances: T[];
      invocationCallOrder: number[];
      results: Array<{ type: string, value: T }>;
    };
  };

  type SpyInstance<T = any, Y extends any[] = any> = {
    (...args: Y): T;
    mockClear(): void;
    mockReset(): void;
    mockRestore(): void;
    mockImplementation(fn: (...args: Y) => T): this;
    mockImplementationOnce(fn: (...args: Y) => T): this;
    mockReturnThis(): this;
    mockReturnValue(value: T): this;
    mockReturnValueOnce(value: T): this;
    mockResolvedValue(value: Awaited<T>): this;
    mockResolvedValueOnce(value: Awaited<T>): this;
    mockRejectedValue(value: any): this;
    mockRejectedValueOnce(value: any): this;
    getMockName(): string;
    mockName(name: string): this;
  };

  interface FunctionLike {
    readonly name: string;
  }

  // Helper utility
  function fn<T = any, Y extends any[] = any>(): Mock<T, Y>;
  function fn<T = any, Y extends any[] = any>(implementation: (...args: Y) => T): Mock<T, Y>;
  function spyOn<T extends {}, M extends keyof T>(object: T, method: M): SpyInstance<T[M]>;
  function mock<T extends string>(moduleName: T, factory?: any, options?: any): jest.Mocked<any>;
  function clearAllMocks(): void;
  function resetAllMocks(): void;
  function restoreAllMocks(): void;
}

declare const expect: {
  <T = any>(actual: T): jest.Matchers<T>;
  extend(matchers: Record<string, any>): void;
  any(constructor: any): any;
  anything(): any;
  arrayContaining(arr: Array<any>): any;
  objectContaining(obj: Record<string, any>): any;
  stringContaining(str: string): any;
  stringMatching(str: string | RegExp): any;
  not: {
    arrayContaining(arr: Array<any>): any;
    objectContaining(obj: Record<string, any>): any;
    stringContaining(str: string): any;
    stringMatching(str: string | RegExp): any;
  };
}; 