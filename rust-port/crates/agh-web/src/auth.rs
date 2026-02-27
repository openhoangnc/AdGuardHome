use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::http::{HeaderMap, header};
use uuid::Uuid;

const SESSION_COOKIE: &str = "agh_session";
const SESSION_TTL: Duration = Duration::from_secs(720 * 3600); // 30 days

/// A session entry in the store.
#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,
    pub expires_at: Instant,
}

/// In-memory session store.
pub struct SessionStore {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new session for the given user. Returns the session token.
    pub fn create(&self, username: &str) -> String {
        let token = Uuid::new_v4().to_string();
        let mut sessions = self.sessions.lock().expect("lock poisoned");
        sessions.insert(
            token.clone(),
            Session {
                username: username.to_owned(),
                expires_at: Instant::now() + SESSION_TTL,
            },
        );
        token
    }

    /// Validate a session token. Returns the username if valid.
    pub fn validate(&self, token: &str) -> Option<String> {
        let sessions = self.sessions.lock().expect("lock poisoned");
        let session = sessions.get(token)?;
        if session.expires_at > Instant::now() {
            Some(session.username.clone())
        } else {
            None
        }
    }

    /// Remove a session (logout).
    pub fn remove(&self, token: &str) {
        self.sessions.lock().expect("lock poisoned").remove(token);
    }

    /// Purge all expired sessions.
    pub fn purge_expired(&self) {
        let now = Instant::now();
        self.sessions
            .lock()
            .expect("lock poisoned")
            .retain(|_, v| v.expires_at > now);
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a `Set-Cookie: agh_session=<token>; HttpOnly; Path=/` header value.
pub fn make_session_cookie(token: &str, secure: bool) -> String {
    let flags = if secure { "; Secure" } else { "" };
    format!("{SESSION_COOKIE}={token}; HttpOnly; Path=/{flags}")
}

/// Extract the session token from the Cookie header.
pub fn extract_session_token(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in cookie.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix(&format!("{SESSION_COOKIE}=")) {
            return Some(val.to_owned());
        }
    }
    None
}

/// Verify a bcrypt password hash against the given plain-text password.
///
/// Compatible with the Go `bcrypt.CompareHashAndPassword` implementation.
pub fn verify_password(hash: &str, password: &str) -> bool {
    if hash.is_empty() {
        return false;
    }
    bcrypt::verify(password, hash).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_session() {
        let store = SessionStore::new();
        let token = store.create("admin");
        assert_eq!(store.validate(&token), Some("admin".to_owned()));
    }

    #[test]
    fn test_remove_session() {
        let store = SessionStore::new();
        let token = store.create("admin");
        store.remove(&token);
        assert_eq!(store.validate(&token), None);
    }

    #[test]
    fn test_extract_session_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            format!("other=value; {SESSION_COOKIE}=mytoken123; another=x")
                .parse()
                .unwrap(),
        );
        assert_eq!(
            extract_session_token(&headers),
            Some("mytoken123".to_owned())
        );
    }

    #[test]
    fn test_invalid_session_returns_none() {
        let store = SessionStore::new();
        assert_eq!(store.validate("nonexistent-token"), None);
    }
}
