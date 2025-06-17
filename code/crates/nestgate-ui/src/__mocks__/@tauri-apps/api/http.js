const mockFetch = jest.fn(() => Promise.resolve({
  status: 200,
  data: {},
  ok: true
}));

export { mockFetch as fetch };
export default { fetch: mockFetch }; 