// **SESSION MANAGEMENT**
//! Session functionality and utilities.
// Session lifecycle management for authentication service.

use super::types::{Session, DeviceInfo};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

/// Session manager
pub struct SessionManager {
    sessions: HashMap<String, Session>,
}
impl SessionManager {
    /// Create new session manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Create new session
    pub fn create_session(&mut self, user_id: String, device_info: DeviceInfo, ip_address: String, user_agent: String) -> String {
        let session_id = Uuid::new_v4().to_string();
        let now = SystemTime::now();
        
        let session = Session {
            session_id: session_id.clone(),
            user_id,
            created_at: now,
            expires_at: now + std::time::Duration::from_secs(86400), // 24 hours
            last_activity: now,
            ip_address,
            user_agent,
            is_active: true,
            device_info,
        };

        self.sessions.insert(session_id.clone(), session);
        session_id
    }

    /// Get session
    pub const fn get_session(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    }

    /// Update session activity
    pub fn update_activity(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.last_activity = SystemTime::now();
            true
        } else {
            false
        }
    }

    /// Invalidate session
    pub fn invalidate_session(&mut self, session_id: &str) -> bool {
        self.sessions.remove(session_id).is_some()
    }

    /// Cleanup expired sessions
    pub fn cleanup_expired(&mut self) {
        let now = SystemTime::now();
        self.sessions.retain(|_, session| session.expires_at > now);
    }
} 