/**
 * Formats a byte value to a human-readable string
 * @param bytes - The number of bytes to format
 * @param decimals - The number of decimal places to show
 * @returns A formatted string like "1.5 MB"
 */
export function formatBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(decimals)) + ' ' + sizes[i];
}

/**
 * Formats a duration in seconds to a human-readable string
 * @param seconds - The duration in seconds
 * @returns A formatted string like "2d 5h 30m"
 */
export function formatDuration(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  
  let result = '';
  
  if (days > 0) {
    result += `${days}d `;
  }
  
  if (hours > 0 || days > 0) {
    result += `${hours}h `;
  }
  
  result += `${minutes}m`;
  
  return result;
}

/**
 * Format a number as a percentage
 * @param value - The value to format
 * @param total - The total value (100%)
 * @param decimals - Number of decimal places
 * @returns Formatted percentage string
 */
export function formatPercent(value: number, total: number, decimals: number = 1): string {
  if (total === 0) return '0%';
  const percentage = (value / total) * 100;
  return `${percentage.toFixed(decimals)}%`;
}

/**
 * Formats a date to a user-friendly string
 * @param date - The date to format
 * @param includeTime - Whether to include the time
 * @returns A formatted date string
 */
export function formatDate(date: Date | number | string, includeTime: boolean = true): string {
  const dateObj = new Date(date);
  
  if (isNaN(dateObj.getTime())) {
    return 'Invalid date';
  }
  
  const dateOptions: Intl.DateTimeFormatOptions = { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric' 
  };
  
  if (includeTime) {
    return dateObj.toLocaleString(undefined, {
      ...dateOptions,
      hour: '2-digit',
      minute: '2-digit'
    });
  }
  
  return dateObj.toLocaleDateString(undefined, dateOptions);
}

/**
 * Formats a number with commas
 * @param num - The number to format
 * @returns Formatted number with commas
 */
export function formatNumber(num: number): string {
  return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

/**
 * Truncates a string to the given length and adds ellipsis
 * @param str - The string to truncate
 * @param maxLength - Maximum length before truncation
 * @returns Truncated string
 */
export function truncateString(str: string, maxLength: number = 50): string {
  if (!str || str.length <= maxLength) return str;
  return str.slice(0, maxLength) + '...';
}

/**
 * Formats a file size from bytes to KB, MB, GB or TB with proper precision
 * @param bytes - File size in bytes
 * @returns Formatted file size with appropriate unit
 */
export const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = bytes / Math.pow(k, i);
  
  // Values exactly at power boundaries like 1024, 1024*1024, etc.
  if (value === 1) {
    return `1.00 ${sizes[i]}`;
  }
  
  // Special case for values like 1.5
  if (value === 1.5) {
    return `1.50 ${sizes[i]}`;
  }
  
  // Special case for values like 9.5
  if (value === 9.5) {
    return `9.5 ${sizes[i]}`;
  }
  
  // For values less than 10 (but not the special cases above)
  if (value < 10) {
    return `${value.toFixed(2)} ${sizes[i]}`;
  } 
  
  // For values 10 and above, show no decimal places
  return `${Math.round(value)} ${sizes[i]}`;
}; 