//! Session management for MCP service
//!
//! This module provides session handling for the MCP service, including
//! client authentication, session tracking, and session cleanup.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::error::{Error, Result};

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Session is active
    Active,
    /// Session is authenticating
    Authenticating,
    /// Session is being established
    Establishing,
    /// Session is being closed
    Closing,
    /// Session is closed
    Closed,
}

/// Client authentication level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthLevel {
    /// No authentication
    None,
    /// Basic authentication
    Basic,
    /// Token authentication
    Token,
    /// Certificate authentication
    Certificate,
}

/// Session information
#[derive(Debug, Clone)]
pub struct Session {
    /// Session ID
    pub id: String,
    /// Client ID
    pub client_id: String,
    /// Authentication level
    pub auth_level: AuthLevel,
    /// Session state
    pub state: SessionState,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Client address
    pub client_address: String,
}

impl Session {
    /// Create a new session
    pub fn new(client_id: String, client_address: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4().to_string(),
            client_id,
            auth_level: AuthLevel::None,
            state: SessionState::Establishing,
            created_at: now,
            last_activity: now,
            client_address,
        }
    }

    /// Update last activity time
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }

    /// Check if session is expired
    pub fn is_expired(&self, timeout: Duration) -> bool {
        let now = Utc::now();
        let last_activity = self.last_activity;

        now.signed_duration_since(last_activity)
            .to_std()
            .unwrap_or_default()
            > timeout
    }

    /// Set session state
    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
        self.update_activity();
    }

    /// Set authentication level
    pub fn set_auth_level(&mut self, auth_level: AuthLevel) {
        self.auth_level = auth_level;
        self.update_activity();
    }
}

/// Session manager
#[derive(Debug)]
pub struct SessionManager {
    /// Active sessions
    sessions: HashMap<String, Session>,
    /// Session timeout
    timeout: Duration,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            timeout: Duration::from_secs(3600), // Default: 1 hour
        }
    }

    /// Create a new session for a client
    pub fn create_session(&mut self, client_id: String, client_address: String) -> Session {
        let session = Session::new(client_id, client_address.clone());
        let id = session.id.clone();

        self.sessions.insert(id.clone(), session.clone());

        debug!("Created session {} for client at {}", id, client_address);
        session
    }

    /// Get a session by ID
    pub fn get_session(&self, id: &str) -> Option<Session> {
        self.sessions.get(id).cloned()
    }

    /// Update a session
    pub fn update_session(&mut self, session: Session) -> Result<()> {
        if !self.sessions.contains_key(&session.id) {
            return Err(Error::session(format!("Session not found: {}", session.id)));
        }

        self.sessions.insert(session.id.clone(), session);
        Ok(())
    }

    /// Close a session
    pub fn close_session(&mut self, id: &str) -> Result<()> {
        if let Some(mut session) = self.sessions.get(id).cloned() {
            session.set_state(SessionState::Closing);
            self.sessions.insert(id.to_string(), session);
            self.sessions.remove(id);

            debug!("Closed session {}", id);
            Ok(())
        } else {
            Err(Error::session(format!("Session not found: {}", id)))
        }
    }

    /// Close all sessions
    pub async fn close_all(&mut self) -> Result<()> {
        let ids: Vec<String> = self.sessions.keys().cloned().collect();

        for id in ids {
            self.close_session(&id)?;
        }

        Ok(())
    }

    /// Clean up expired sessions
    pub fn cleanup_expired(&mut self) -> usize {
        let expired: Vec<String> = self
            .sessions
            .iter()
            .filter(|(_, session)| session.is_expired(self.timeout))
            .map(|(id, _)| id.clone())
            .collect();

        let count = expired.len();

        for id in expired {
            let _ = self.close_session(&id);
        }

        if count > 0 {
            debug!("Cleaned up {} expired sessions", count);
        }

        count
    }

    /// Get the number of active sessions
    pub fn active_count(&self) -> usize {
        self.sessions
            .values()
            .filter(|s| s.state == SessionState::Active)
            .count()
    }

    /// Get all sessions
    pub fn all_sessions(&self) -> Vec<Session> {
        self.sessions.values().cloned().collect()
    }

    /// Set session timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Get session timeout
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let mut manager = SessionManager::new();
        let session = manager.create_session("client-1".to_string(), "192.168.1.1".to_string());

        assert_eq!(session.client_id, "client-1");
        assert_eq!(session.state, SessionState::Establishing);
        assert_eq!(session.auth_level, AuthLevel::None);

        let retrieved = manager
            .get_session(&session.id)
            .expect("Session should exist after creation");
        assert_eq!(retrieved.id, session.id);
    }

    #[test]
    fn test_session_expiration() {
        let mut session = Session::new("client-1".to_string(), "192.168.1.1".to_string());

        // Set last activity to 2 hours ago
        let two_hours_ago = Utc::now() - chrono::Duration::hours(2);
        session.last_activity = two_hours_ago;

        // Check if session is expired with 1 hour timeout
        assert!(session.is_expired(Duration::from_secs(3600)));

        // Update activity
        session.update_activity();

        // Should no longer be expired
        assert!(!session.is_expired(Duration::from_secs(3600)));
    }

    #[test]
    fn test_session_cleanup() {
        let mut manager = SessionManager::new();

        // Create three sessions
        let session1 = manager.create_session("client-1".to_string(), "192.168.1.1".to_string());
        let session2 = manager.create_session("client-2".to_string(), "192.168.1.2".to_string());
        let session3 = manager.create_session("client-3".to_string(), "192.168.1.3".to_string());

        // Set a short timeout
        manager.set_timeout(Duration::from_millis(10));

        // Sleep to make sessions expire
        std::thread::sleep(Duration::from_millis(20));

        // Clean up expired sessions
        let cleaned = manager.cleanup_expired();

        assert_eq!(cleaned, 3);
        assert_eq!(manager.all_sessions().len(), 0);
    }
}
