import axios from 'axios';
import { API_BASE_URL } from '../../constants';
import * as crypto from 'crypto';
import DOMPurify from 'dompurify';

/**
 * Service responsible for security features including:
 * - Input validation and sanitization
 * - XSS protection
 * - SQL injection protection
 * - CSRF protection
 * - Rate limiting and brute force protection
 * - Authentication security
 * - Session security
 */
export class SecurityService {
  private static readonly API_URL = `${API_BASE_URL}/api/security`;
  private readonly loginAttempts: Map<string, number> = new Map();
  private readonly accountLockouts: Map<string, number> = new Map();
  private readonly activeSessions: Map<string, { userId: string, ip: string, userAgent: string }> = new Map();
  private readonly csrfTokens: Set<string> = new Set();
  
  // Common passwords from breached databases
  private readonly commonPasswords: Set<string> = new Set([
    'password', '123456', 'qwerty', 'admin', 'welcome', 
    'password123', 'abc123', 'letmein', '12345678', 'admin123'
  ]);
  
  // SQL injection patterns
  private readonly sqlInjectionPatterns: RegExp[] = [
    /('|")\s*OR\s*('|").*('|")\s*=\s*('|")/i, // ' OR '1'='1
    /;\s*(DROP|DELETE|UPDATE|INSERT)/i,        // ; DROP TABLE
    /UNION\s+ALL\s+SELECT/i,                  // UNION ALL SELECT
    /--\s+/,                                   // SQL comment
    /(\/\*|\*\/)/,                            // Block comments
    /DROP\s+TABLE/i,                          // DROP TABLE statement
    /DELETE\s+FROM/i,                         // DELETE FROM statement
    /UPDATE\s+.*\s+SET/i                      // UPDATE statement
  ];

  /**
   * Sanitizes user input to prevent XSS
   */
  public sanitizeInput(input: string): string {
    if (!input) return '';
    
    // Use DOMPurify to clean HTML
    return DOMPurify.sanitize(input, {
      ALLOWED_TAGS: ['p', 'b', 'i', 'em', 'strong', 'a', 'ul', 'ol', 'li'],
      ALLOWED_ATTR: ['href', 'title', 'target']
    });
  }

  /**
   * Validates email address format
   */
  public validateEmail(email: string): boolean {
    if (!email) return false;
    
    // Basic email validation
    const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
    
    // Check for valid format and no suspicious content
    return emailRegex.test(email) && !this.containsMaliciousContent(email);
  }

  /**
   * Validates password strength
   */
  public validatePasswordStrength(password: string): boolean {
    if (!password || password.length < 8) return false;
    
    // Check for SQL injection attempts
    if (this.detectSqlInjection(password)) return false;
    
    // Check for complexity (at least 3 of 4 criteria)
    let criteria = 0;
    if (/[A-Z]/.test(password)) criteria++; // Uppercase
    if (/[a-z]/.test(password)) criteria++; // Lowercase
    if (/[0-9]/.test(password)) criteria++; // Digits
    if (/[^A-Za-z0-9]/.test(password)) criteria++; // Special chars
    
    return criteria >= 3;
  }

  /**
   * Sanitizes and prevents XSS in object properties
   */
  public preventXSS(data: Record<string, any>): Record<string, any> {
    const sanitizedData: Record<string, any> = {};
    
    for (const [key, value] of Object.entries(data)) {
      if (typeof value === 'string') {
        sanitizedData[key] = this.sanitizeInput(value);
      } else if (typeof value === 'object' && value !== null) {
        sanitizedData[key] = this.preventXSS(value);
      } else {
        sanitizedData[key] = value;
      }
    }
    
    return sanitizedData;
  }

  /**
   * Encodes HTML entities in content
   */
  public encodeHtmlEntities(content: string): string {
    return content
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#039;');
  }

  /**
   * Detects SQL injection attempts in input
   */
  public detectSqlInjection(input: string): boolean {
    if (!input) return false;
    
    return this.sqlInjectionPatterns.some(pattern => pattern.test(input));
  }

  /**
   * Builds a safe parameterized query
   */
  public buildSafeQuery(queryType: string, params: Record<string, any>): string {
    // This is a simplified example, in a real app this would use proper query builders
    switch (queryType) {
      case 'getUserById':
        return `SELECT * FROM users WHERE id = ?`;
      case 'updateUser':
        return `UPDATE users SET name = ?, email = ? WHERE id = ?`;
      default:
        return `SELECT 1`;
    }
  }

  /**
   * Generates a CSRF token
   */
  public generateCsrfToken(): string {
    const token = crypto.randomBytes(32).toString('hex');
    this.csrfTokens.add(token);
    return token;
  }

  /**
   * Validates a CSRF token
   */
  public validateCsrfToken(token: string): boolean {
    const isValid = this.csrfTokens.has(token);
    if (isValid) {
      // Use once only
      this.csrfTokens.delete(token);
    }
    return isValid;
  }

  /**
   * Performs a state-changing operation with CSRF protection
   */
  public async performStateChangingOperation(
    operation: string, 
    data: Record<string, any>, 
    csrfToken: string
  ): Promise<any> {
    if (!csrfToken || !this.validateCsrfToken(csrfToken)) {
      throw new Error('CSRF token validation failed');
    }
    
    const payload = {
      ...data,
      _csrf: csrfToken
    };
    
    const response = await axios.post(`${SecurityService.API_URL}/${operation}`, payload);
    return response.data;
  }

  /**
   * Tracks login attempts for a user
   */
  public trackLoginAttempt(username: string, successful: boolean): void {
    if (successful) {
      // Reset on successful login
      this.loginAttempts.delete(username);
      this.accountLockouts.delete(username);
      return;
    }
    
    // Increment failed attempts
    const attempts = (this.loginAttempts.get(username) || 0) + 1;
    this.loginAttempts.set(username, attempts);
    
    // Lock account after too many attempts
    if (attempts >= 5) {
      const lockoutTime = Date.now() + 30 * 60 * 1000; // 30 minutes
      this.accountLockouts.set(username, lockoutTime);
    }
  }

  /**
   * Checks if an account is locked
   */
  public isAccountLocked(username: string): boolean {
    const lockoutTime = this.accountLockouts.get(username);
    if (!lockoutTime) return false;
    
    if (Date.now() > lockoutTime) {
      // Lockout expired
      this.accountLockouts.delete(username);
      return false;
    }
    
    return true;
  }

  /**
   * Calculates progressive delay for login attempts
   */
  public getLoginDelayMs(username: string, attempts: number): number {
    // Exponential backoff formula: base * (2^attempts)
    const baseDelay = 100; // 100ms base
    return baseDelay * Math.pow(2, attempts);
  }

  /**
   * Hashes passwords using strong cryptography
   */
  public hashPassword(password: string): string {
    // In a real implementation, this would use bcrypt or argon2
    // This is a simplified version for demonstration purposes
    const salt = crypto.randomBytes(16).toString('hex');
    const hash = crypto.pbkdf2Sync(password, salt, 1000, 64, 'sha512').toString('hex');
    return `${salt}:${hash}`;
  }

  /**
   * Verifies a password against a stored hash
   */
  public verifyPassword(password: string, storedHash: string): boolean {
    // In a real implementation, this would use bcrypt or argon2
    const [salt, hash] = storedHash.split(':');
    const generatedHash = crypto.pbkdf2Sync(password, salt, 1000, 64, 'sha512').toString('hex');
    return hash === generatedHash;
  }

  /**
   * Checks if a password is common/weak
   */
  public isCommonPassword(password: string): boolean {
    return this.commonPasswords.has(password.toLowerCase());
  }

  /**
   * Generates a session token
   */
  public generateSessionToken(userId: string): string {
    const sessionId = crypto.randomBytes(32).toString('hex');
    const timestamp = Date.now();
    const payload = `${userId}.${timestamp}`;
    
    // In a real implementation, this would be signed with a secret key
    const signature = crypto.createHash('sha256').update(payload).digest('hex');
    
    return `${payload}.${signature}`;
  }

  /**
   * Validates a session token
   */
  public validateSessionToken(token: string): { valid: boolean, userId?: string } {
    try {
      const [userId, timestamp, signature] = token.split('.');
      
      if (!userId || !timestamp || !signature) {
        return { valid: false };
      }
      
      const payload = `${userId}.${timestamp}`;
      const expectedSignature = crypto.createHash('sha256').update(payload).digest('hex');
      
      if (signature !== expectedSignature) {
        return { valid: false };
      }
      
      // Check if token is expired (24 hours)
      const tokenTime = parseInt(timestamp, 10);
      if (Date.now() - tokenTime > 24 * 60 * 60 * 1000) {
        return { valid: false };
      }
      
      return { valid: true, userId };
    } catch (error) {
      return { valid: false };
    }
  }

  /**
   * Creates a new session
   */
  public createSession(userId: string, ip: string, userAgent: string): string {
    const sessionId = crypto.randomBytes(16).toString('hex');
    
    this.activeSessions.set(sessionId, {
      userId,
      ip,
      userAgent
    });
    
    return sessionId;
  }

  /**
   * Detects potential session hijacking
   */
  public detectSessionHijacking(sessionId: string, ip: string, userAgent: string): boolean {
    const session = this.activeSessions.get(sessionId);
    
    if (!session) {
      return true; // Unknown session
    }
    
    // Check for IP or user agent changes
    return session.ip !== ip || session.userAgent !== userAgent;
  }

  /**
   * Checks if content contains malicious script patterns
   */
  private containsMaliciousContent(content: string): boolean {
    const scriptPattern = /<script[^>]*>.*?<\/script>/i;
    const eventPattern = /on\w+\s*=/i;
    const dataUrlPattern = /data:.*?;base64/i;
    
    return scriptPattern.test(content) || 
           eventPattern.test(content) || 
           dataUrlPattern.test(content);
  }
} 