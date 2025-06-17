import { formatCapacity, formatDate, formatPercentage } from './format';

describe('Format Utilities', () => {
  describe('formatCapacity', () => {
    test('handles zero bytes', () => {
      expect(formatCapacity(0)).toBe('0 Bytes');
    });

    test('formats bytes correctly', () => {
      expect(formatCapacity(500)).toBe('500 Bytes');
    });

    test('formats kilobytes correctly', () => {
      expect(formatCapacity(1024)).toBe('1 KB');
      expect(formatCapacity(2048)).toBe('2 KB');
    });

    test('formats megabytes correctly', () => {
      expect(formatCapacity(1048576)).toBe('1 MB');
      expect(formatCapacity(2097152)).toBe('2 MB');
    });

    test('formats gigabytes correctly', () => {
      expect(formatCapacity(1073741824)).toBe('1 GB');
      expect(formatCapacity(10737418240)).toBe('10 GB');
    });

    test('formats terabytes correctly', () => {
      expect(formatCapacity(1099511627776)).toBe('1 TB');
      expect(formatCapacity(2199023255552)).toBe('2 TB');
    });
  });

  describe('formatDate', () => {
    test('formats date string correctly', () => {
      const dateString = '2023-01-01T12:00:00Z';
      const result = formatDate(dateString);
      
      // The result will vary based on the locale, but should contain the date and time
      expect(result).toContain('2023');
      expect(result.length).toBeGreaterThan(10); // Should contain more than just the date
    });

    test('formats date object correctly', () => {
      const date = new Date(2023, 0, 1, 12, 0, 0);
      const result = formatDate(date);
      
      // The result will vary based on the locale, but should contain the date and time
      expect(result).toContain('2023');
      expect(result.length).toBeGreaterThan(10);
    });
  });

  describe('formatPercentage', () => {
    test('formats whole numbers', () => {
      expect(formatPercentage(50)).toBe('50.0%');
    });

    test('formats decimal numbers with default precision', () => {
      expect(formatPercentage(50.5)).toBe('50.5%');
    });

    test('formats decimal numbers with custom precision', () => {
      expect(formatPercentage(50.555, 2)).toBe('50.55%');
    });

    test('handles zero', () => {
      expect(formatPercentage(0)).toBe('0.0%');
    });
  });
}); 