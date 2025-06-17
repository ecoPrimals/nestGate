import { SecurityService } from '../security/security.service';

describe('SecurityService', () => {
  let securityService: SecurityService;

  beforeEach(() => {
    securityService = new SecurityService();
  });

  describe('Input Validation and Sanitization', () => {
    it('should sanitize HTML input', () => {
      const maliciousInput = '<script>alert("XSS")</script><p>Hello</p>';
      const sanitizedInput = securityService.sanitizeInput(maliciousInput);
      
      expect(sanitizedInput).not.toContain('<script>');
      expect(sanitizedInput).toContain('<p>Hello</p>');
    });

    it('should validate emails correctly', () => {
      expect(securityService.validateEmail('valid@example.com')).toBe(true);
      expect(securityService.validateEmail('invalid')).toBe(false);
      expect(securityService.validateEmail('')).toBe(false);
      expect(securityService.validateEmail('<script>alert("XSS")</script>@example.com')).toBe(false);
    });

    it('should validate password strength', () => {
      expect(securityService.validatePasswordStrength('weak')).toBe(false); // Too short
      expect(securityService.validatePasswordStrength('password123')).toBe(false); // Doesn't meet criteria
      expect(securityService.validatePasswordStrength('P@ssw0rd!')).toBe(true); // Strong password
    });
  });

  describe('SQL Injection Protection', () => {
    it('should detect SQL injection attempts', () => {
      expect(securityService.detectSqlInjection("'; DROP TABLE users; --")).toBe(true);
      expect(securityService.detectSqlInjection("Robert'); DROP TABLE users; --")).toBe(true);
      expect(securityService.detectSqlInjection("1 OR 1=1")).toBe(false); // Missing quotes
      expect(securityService.detectSqlInjection("' OR '1'='1")).toBe(true);
      expect(securityService.detectSqlInjection("Normal input")).toBe(false);
    });

    it('should build safe parameterized queries', () => {
      const query = securityService.buildSafeQuery('getUserById', { id: "1' OR '1'='1" });
      expect(query).toContain('?');
      expect(query).not.toContain("1' OR '1'='1");
    });
  });

  describe('XSS Protection', () => {
    it('should prevent XSS in objects', () => {
      const obj = {
        name: '<script>alert("XSS")</script>John',
        description: '<img src="x" onerror="alert(\'XSS\')">',
        nested: {
          prop: '<script>fetch("evil.com")</script>'
        }
      };
      
      const sanitized = securityService.preventXSS(obj);
      
      expect(sanitized.name).not.toContain('<script>');
      expect(sanitized.description).not.toContain('onerror');
      expect(sanitized.nested.prop).not.toContain('<script>');
    });

    it('should encode HTML entities', () => {
      const input = '<div onclick="evil()">Text</div>';
      const encoded = securityService.encodeHtmlEntities(input);
      
      expect(encoded).toContain('&lt;div');
      expect(encoded).toContain('&gt;');
      expect(encoded).not.toContain('<div');
    });
  });

  describe('CSRF Protection', () => {
    it('should generate and validate CSRF tokens', () => {
      const token = securityService.generateCsrfToken();
      
      expect(typeof token).toBe('string');
      expect(token.length).toBeGreaterThan(16);
      expect(securityService.validateCsrfToken(token)).toBe(true);
      // Token should be one-time use
      expect(securityService.validateCsrfToken(token)).toBe(false);
    });

    it('should reject invalid CSRF tokens', () => {
      expect(securityService.validateCsrfToken('invalid-token')).toBe(false);
      expect(securityService.validateCsrfToken('')).toBe(false);
    });
  });

  describe('Authentication Security', () => {
    it('should securely hash and verify passwords', () => {
      const password = 'SecureP@ssword123';
      const hash = securityService.hashPassword(password);
      
      expect(hash).not.toBe(password);
      expect(hash.length).toBeGreaterThan(20);
      expect(securityService.verifyPassword(password, hash)).toBe(true);
      expect(securityService.verifyPassword('WrongPassword', hash)).toBe(false);
    });

    it('should detect common passwords', () => {
      expect(securityService.isCommonPassword('password')).toBe(true);
      expect(securityService.isCommonPassword('admin')).toBe(true);
      expect(securityService.isCommonPassword('vF8%k2P!9zQx')).toBe(false);
    });
  });

  describe('Rate Limiting', () => {
    it('should track login attempts and lock accounts', () => {
      // Initial state
      expect(securityService.isAccountLocked('testuser')).toBe(false);
      
      // Track 4 failed attempts
      for (let i = 0; i < 4; i++) {
        securityService.trackLoginAttempt('testuser', false);
      }
      
      // Account should not be locked yet
      expect(securityService.isAccountLocked('testuser')).toBe(false);
      
      // Track the 5th failed attempt
      securityService.trackLoginAttempt('testuser', false);
      
      // Account should be locked now
      expect(securityService.isAccountLocked('testuser')).toBe(true);
      
      // Successful login should unlock the account
      securityService.trackLoginAttempt('testuser', true);
      expect(securityService.isAccountLocked('testuser')).toBe(false);
    });

    it('should implement progressive delays', () => {
      const delay1 = securityService.getLoginDelayMs('user', 1);
      const delay2 = securityService.getLoginDelayMs('user', 2);
      const delay3 = securityService.getLoginDelayMs('user', 3);
      
      expect(delay2).toBeGreaterThan(delay1);
      expect(delay3).toBeGreaterThan(delay2);
      expect(delay3).toBe(delay1 * 4); // 2^2 = 4 times the base delay
    });
  });

  describe('Session Security', () => {
    it('should generate and validate session tokens', () => {
      const token = securityService.generateSessionToken('user123');
      const result = securityService.validateSessionToken(token);
      
      expect(result.valid).toBe(true);
      expect(result.userId).toBe('user123');
    });

    it('should detect session hijacking', () => {
      const sessionId = securityService.createSession('user1', '192.168.1.1', 'Chrome/90.0');
      
      // Same IP and user agent (no hijacking)
      expect(securityService.detectSessionHijacking(sessionId, '192.168.1.1', 'Chrome/90.0')).toBe(false);
      
      // Different IP (potential hijacking)
      expect(securityService.detectSessionHijacking(sessionId, '10.0.0.1', 'Chrome/90.0')).toBe(true);
      
      // Different user agent (potential hijacking)
      expect(securityService.detectSessionHijacking(sessionId, '192.168.1.1', 'Firefox/88.0')).toBe(true);
      
      // Unknown session ID
      expect(securityService.detectSessionHijacking('unknown-session', '192.168.1.1', 'Chrome/90.0')).toBe(true);
    });
  });
}); 