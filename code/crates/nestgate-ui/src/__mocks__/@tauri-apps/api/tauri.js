const mockInvoke = jest.fn(() => Promise.resolve({}));

export { mockInvoke as invoke };
export default { invoke: mockInvoke }; 