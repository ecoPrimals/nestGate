import {
  formatBytes,
  formatDuration,
  formatPercent,
  formatDate,
  formatNumber,
  truncateString,
  formatFileSize
} from '../../utils/format.utils';

describe('Format Utilities', () => {
  describe('formatBytes', () => {
    it('formats 0 bytes correctly', () => {
      expect(formatBytes(0)).toBe('0 Bytes');
    });
    
    it('formats bytes below 1KB correctly', () => {
      expect(formatBytes(512)).toBe('512 Bytes');
    });
    
    it('formats KB values correctly', () => {
      expect(formatBytes(1024)).toBe('1 KB');
      expect(formatBytes(1536)).toBe('1.5 KB');
    });
    
    it('formats MB values correctly', () => {
      expect(formatBytes(1024 * 1024)).toBe('1 MB');
      expect(formatBytes(2.5 * 1024 * 1024)).toBe('2.5 MB');
    });
    
    it('formats GB values correctly', () => {
      expect(formatBytes(1024 * 1024 * 1024)).toBe('1 GB');
    });
    
    it('formats TB values correctly', () => {
      expect(formatBytes(1024 * 1024 * 1024 * 1024)).toBe('1 TB');
    });
    
    it('respects decimal places parameter', () => {
      expect(formatBytes(1234567, 0)).toBe('1 MB');
      expect(formatBytes(1234567, 1)).toBe('1.2 MB');
      expect(formatBytes(1234567, 3)).toBe('1.177 MB');
    });
  });
  
  describe('formatDuration', () => {
    it('formats seconds correctly', () => {
      expect(formatDuration(45)).toBe('0m');
    });
    
    it('formats minutes correctly', () => {
      expect(formatDuration(60)).toBe('1m');
      expect(formatDuration(120)).toBe('2m');
    });
    
    it('formats hours correctly', () => {
      expect(formatDuration(3600)).toBe('1h 0m');
      expect(formatDuration(3660)).toBe('1h 1m');
      expect(formatDuration(7200)).toBe('2h 0m');
    });
    
    it('formats days correctly', () => {
      expect(formatDuration(86400)).toBe('1d 0h 0m');
      expect(formatDuration(90000)).toBe('1d 1h 0m');
      expect(formatDuration(172800)).toBe('2d 0h 0m');
    });
    
    it('formats complex durations correctly', () => {
      expect(formatDuration(98765)).toBe('1d 3h 26m');
    });
  });
  
  describe('formatPercent', () => {
    it('handles zero values correctly', () => {
      expect(formatPercent(0, 100)).toBe('0.0%');
      expect(formatPercent(0, 0)).toBe('0%');
    });
    
    it('calculates percentages correctly', () => {
      expect(formatPercent(25, 100)).toBe('25.0%');
      expect(formatPercent(75, 100)).toBe('75.0%');
    });
    
    it('handles values greater than total', () => {
      expect(formatPercent(150, 100)).toBe('150.0%');
    });
    
    it('respects decimal places parameter', () => {
      expect(formatPercent(33.333, 100, 0)).toBe('33%');
      expect(formatPercent(33.333, 100, 1)).toBe('33.3%');
      expect(formatPercent(33.333, 100, 2)).toBe('33.33%');
    });
  });
  
  describe('formatDate', () => {
    it('formats dates correctly with time', () => {
      // Create a specific date for testing
      const testDate = new Date('2023-01-15T14:30:45');
      
      // The exact format depends on the locale, so we'll just check the key components
      const result = formatDate(testDate);
      
      // Should contain year, month, day
      expect(result).toContain('2023');
      expect(result).toContain('Jan');
      expect(result).toContain('15');
      
      // Should contain time (hours and minutes)
      expect(result).toMatch(/14|02|2/); // Hour might be formatted as 14, 02, or 2 depending on locale
      expect(result).toMatch(/30/); // Minutes
    });
    
    it('formats dates without time when specified', () => {
      const testDate = new Date('2023-01-15T14:30:45');
      
      const result = formatDate(testDate, false);
      
      // Should contain date components
      expect(result).toContain('2023');
      expect(result).toContain('Jan');
      expect(result).toContain('15');
      
      // Should not contain time (checking for common time separators)
      expect(result).not.toMatch(/:/);
      expect(result).not.toMatch(/\d{1,2}:\d{2}/);
    });
    
    it('handles numeric timestamps', () => {
      const timestamp = Date.now();
      expect(formatDate(timestamp)).toBeTruthy();
    });
    
    it('handles string timestamps', () => {
      const timestamp = '2023-01-15T14:30:45';
      expect(formatDate(timestamp)).toBeTruthy();
    });
    
    it('returns "Invalid date" for invalid inputs', () => {
      expect(formatDate('not-a-date')).toBe('Invalid date');
    });
  });
  
  describe('formatNumber', () => {
    it('adds commas to thousands', () => {
      expect(formatNumber(1000)).toBe('1,000');
      expect(formatNumber(1000000)).toBe('1,000,000');
    });
    
    it('handles negative numbers', () => {
      expect(formatNumber(-1000)).toBe('-1,000');
    });
    
    it('handles decimal numbers', () => {
      expect(formatNumber(1000.5)).toBe('1,000.5');
    });
    
    it('handles zero', () => {
      expect(formatNumber(0)).toBe('0');
    });
  });
  
  describe('truncateString', () => {
    it('truncates strings longer than maxLength', () => {
      expect(truncateString('This is a long string', 10)).toBe('This is a ...');
    });
    
    it('does not truncate strings shorter than maxLength', () => {
      expect(truncateString('Short', 10)).toBe('Short');
    });
    
    it('handles empty strings', () => {
      expect(truncateString('', 10)).toBe('');
    });
    
    it('handles null/undefined values', () => {
      expect(truncateString(null as any, 10)).toBe(null);
      expect(truncateString(undefined as any, 10)).toBe(undefined);
    });
    
    it('handles exact length', () => {
      expect(truncateString('1234567890', 10)).toBe('1234567890');
    });
  });
  
  describe('formatFileSize', () => {
    it('formats bytes correctly', () => {
      expect(formatFileSize(512)).toBe('512 B');
    });
    
    it('formats KB correctly with proper precision', () => {
      expect(formatFileSize(1024)).toBe('1.00 KB');
      expect(formatFileSize(1.5 * 1024)).toBe('1.50 KB');
      expect(formatFileSize(9.5 * 1024)).toBe('9.5 KB');
      expect(formatFileSize(10 * 1024)).toBe('10 KB');
      expect(formatFileSize(999 * 1024)).toBe('999 KB');
    });
    
    it('formats MB correctly with proper precision', () => {
      expect(formatFileSize(1024 * 1024)).toBe('1.00 MB');
      expect(formatFileSize(1.5 * 1024 * 1024)).toBe('1.50 MB');
      expect(formatFileSize(9.5 * 1024 * 1024)).toBe('9.5 MB');
      expect(formatFileSize(10 * 1024 * 1024)).toBe('10 MB');
    });
    
    it('formats GB correctly with proper precision', () => {
      expect(formatFileSize(1024 * 1024 * 1024)).toBe('1.00 GB');
      expect(formatFileSize(1.5 * 1024 * 1024 * 1024)).toBe('1.50 GB');
    });
    
    it('formats TB correctly with proper precision', () => {
      expect(formatFileSize(1024 * 1024 * 1024 * 1024)).toBe('1.00 TB');
    });
  });
}); 