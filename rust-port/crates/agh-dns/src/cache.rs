use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use hickory_proto::op::Message;

/// DNS response cache keyed on (name, qtype, qclass).
pub struct DnsCache {
    entries: Mutex<HashMap<CacheKey, CacheEntry>>,
    max_entries: usize,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct CacheKey {
    name: String,
    qtype: u16,
    qclass: u16,
}

struct CacheEntry {
    response: Message,
    expires_at: Instant,
}

impl DnsCache {
    const DEFAULT_TTL: Duration = Duration::from_secs(60);

    /// Create a new cache with the given capacity.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            max_entries,
        }
    }

    /// Look up a cached response for the first query in the request.
    pub fn get(&self, request: &Message) -> Option<Message> {
        let key = cache_key(request)?;
        let entries = self.entries.lock().expect("lock poisoned");
        let entry = entries.get(&key)?;
        if entry.expires_at > Instant::now() {
            Some(entry.response.clone())
        } else {
            None
        }
    }

    /// Insert a response into the cache.
    pub fn insert(&self, request: &Message, response: Message) {
        let key = match cache_key(request) {
            Some(k) => k,
            None => return,
        };

        // Don't cache SERVFAIL.
        use hickory_proto::op::ResponseCode;
        if response.response_code() == ResponseCode::ServFail {
            return;
        }

        let ttl = min_ttl(&response).unwrap_or(Self::DEFAULT_TTL);
        if ttl == Duration::ZERO {
            return;
        }

        let mut entries = self.entries.lock().expect("lock poisoned");
        if entries.len() >= self.max_entries {
            self.evict_expired_locked(&mut entries);
        }

        entries.insert(
            key,
            CacheEntry {
                response,
                expires_at: Instant::now() + ttl,
            },
        );
    }

    /// Return the number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.lock().expect("lock poisoned").len()
    }

    /// Return true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Remove all expired entries.
    pub fn clear_expired(&self) {
        let mut entries = self.entries.lock().expect("lock poisoned");
        self.evict_expired_locked(&mut entries);
    }

    fn evict_expired_locked(&self, entries: &mut HashMap<CacheKey, CacheEntry>) {
        let now = Instant::now();
        entries.retain(|_, v| v.expires_at > now);
    }
}

fn cache_key(request: &Message) -> Option<CacheKey> {
    let q = request.queries().first()?;
    Some(CacheKey {
        name: q.name().to_lowercase().to_string(),
        qtype: u16::from(q.query_type()),
        qclass: u16::from(q.query_class()),
    })
}

fn min_ttl(response: &Message) -> Option<Duration> {
    let ttl = response
        .answers()
        .iter()
        .chain(response.name_servers())
        .map(|r| r.ttl())
        .min()?;
    Some(Duration::from_secs(u64::from(ttl)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hickory_proto::op::{Message, MessageType, ResponseCode};

    fn make_query(name: &str) -> Message {
        use hickory_proto::op::Query;
        use hickory_proto::rr::{DNSClass, Name, RecordType};
        let mut msg = Message::new();
        msg.set_message_type(MessageType::Query);
        let mut q = Query::new();
        q.set_name(Name::from_ascii(name).unwrap());
        q.set_query_type(RecordType::A);
        q.set_query_class(DNSClass::IN);
        msg.add_query(q);
        msg
    }

    #[test]
    fn test_cache_miss_returns_none() {
        let cache = DnsCache::new(100);
        let req = make_query("example.com");
        assert!(cache.get(&req).is_none());
    }

    #[test]
    fn test_servfail_not_cached() {
        let cache = DnsCache::new(100);
        let req = make_query("example.com");
        let mut resp = req.clone();
        resp.set_message_type(MessageType::Response);
        resp.set_response_code(ResponseCode::ServFail);
        cache.insert(&req, resp);
        assert!(cache.get(&req).is_none());
    }

    #[test]
    fn test_len() {
        let cache = DnsCache::new(100);
        assert_eq!(cache.len(), 0);
    }
}
