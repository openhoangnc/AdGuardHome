use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

/// Result of a safe browsing check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafeBrowsingResult {
    Safe,
    Malware,
    Phishing,
}

struct CacheEntry {
    result: SafeBrowsingResult,
    expires_at: Instant,
}

/// Checks domains against AdGuard's safe-browsing hash-prefix DNS service.
pub struct SafeBrowsingChecker {
    enabled: Arc<AtomicBool>,
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl SafeBrowsingChecker {
    const CACHE_TTL: Duration = Duration::from_secs(5 * 60);
    const MAX_CACHE: usize = 10_000;

    /// Create a new checker, optionally enabled.
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(enabled)),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if a domain is flagged by safe browsing.
    ///
    /// Network lookup is stubbed — always returns `Safe`.
    /// Real DNS lookup would query `<prefix>.sb.dns.adguard.com`.
    pub async fn check(&self, domain: &str) -> SafeBrowsingResult {
        if !self.enabled.load(Ordering::Relaxed) {
            return SafeBrowsingResult::Safe;
        }

        let prefix = Self::hash_prefix(domain);

        // Check cache.
        {
            let cache = self.cache.lock().expect("lock poisoned");
            if let Some(entry) = cache.get(&prefix) {
                if entry.expires_at > Instant::now() {
                    return entry.result;
                }
            }
        }

        // Stub: real implementation would do a DNS TXT lookup to
        // `<prefix>.sb.dns.adguard.com` and compare the full SHA256.
        let result = SafeBrowsingResult::Safe;

        // Cache result.
        {
            let mut cache = self.cache.lock().expect("lock poisoned");
            if cache.len() >= Self::MAX_CACHE {
                // Evict expired entries.
                let now = Instant::now();
                cache.retain(|_, v| v.expires_at > now);
            }
            cache.insert(
                prefix,
                CacheEntry { result, expires_at: Instant::now() + Self::CACHE_TTL },
            );
        }

        result
    }

    /// Enable safe browsing checks.
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    /// Disable safe browsing checks.
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    /// Compute the hash prefix used for the DNS lookup.
    /// Returns the first 4 bytes of SHA256(domain) as 8 lowercase hex chars.
    pub fn hash_prefix(domain: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(domain.to_lowercase().as_bytes());
        let hash = hasher.finalize();
        hex::encode(&hash[..4])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_disabled_always_safe() {
        let checker = SafeBrowsingChecker::new(false);
        let result = checker.check("malware.example.com").await;
        assert_eq!(result, SafeBrowsingResult::Safe);
    }

    #[tokio::test]
    async fn test_enabled_returns_safe_stub() {
        let checker = SafeBrowsingChecker::new(true);
        let result = checker.check("example.com").await;
        assert_eq!(result, SafeBrowsingResult::Safe);
    }

    #[test]
    fn test_hash_prefix_length() {
        let prefix = SafeBrowsingChecker::hash_prefix("example.com");
        assert_eq!(prefix.len(), 8);
    }

    #[test]
    fn test_hash_prefix_deterministic() {
        let a = SafeBrowsingChecker::hash_prefix("example.com");
        let b = SafeBrowsingChecker::hash_prefix("example.com");
        assert_eq!(a, b);
    }

    #[test]
    fn test_enable_disable() {
        let checker = SafeBrowsingChecker::new(false);
        checker.enable();
        assert!(checker.enabled.load(Ordering::Relaxed));
        checker.disable();
        assert!(!checker.enabled.load(Ordering::Relaxed));
    }
}
