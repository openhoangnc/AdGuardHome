use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Safe search rewriter — redirects search engine queries to their safe counterparts.
pub struct SafeSearchRewriter {
    enabled: Arc<AtomicBool>,
    mappings: HashMap<String, IpAddr>,
}

impl SafeSearchRewriter {
    /// Create a new safe search rewriter with built-in mappings.
    pub fn new(enabled: bool) -> Self {
        let mut mappings: HashMap<String, IpAddr> = HashMap::new();

        // Google — forcesafesearch.google.com
        let google_ip: IpAddr = "216.239.38.120".parse().expect("valid IP");
        for domain in &[
            "google.com",
            "www.google.com",
            "google.co.uk",
            "www.google.co.uk",
            "google.de",
            "www.google.de",
            "google.fr",
            "www.google.fr",
        ] {
            mappings.insert(domain.to_string(), google_ip);
        }

        // Bing — strict.bing.com
        let bing_ip: IpAddr = "204.79.197.220".parse().expect("valid IP");
        for domain in &["bing.com", "www.bing.com"] {
            mappings.insert(domain.to_string(), bing_ip);
        }

        // DuckDuckGo — safe.duckduckgo.com
        let ddg_ip: IpAddr = "46.51.197.89".parse().expect("valid IP");
        for domain in &["duckduckgo.com", "www.duckduckgo.com"] {
            mappings.insert(domain.to_string(), ddg_ip);
        }

        // YouTube — restrict.youtube.com
        let yt_ip: IpAddr = "216.239.38.120".parse().expect("valid IP");
        for domain in &["youtube.com", "www.youtube.com", "m.youtube.com"] {
            mappings.insert(domain.to_string(), yt_ip);
        }

        // Yandex — safe Yandex
        let yandex_ip: IpAddr = "213.180.193.56".parse().expect("valid IP");
        for domain in &["yandex.com", "www.yandex.com", "yandex.ru", "www.yandex.ru"] {
            mappings.insert(domain.to_string(), yandex_ip);
        }

        Self {
            enabled: Arc::new(AtomicBool::new(enabled)),
            mappings,
        }
    }

    /// Returns `Some(ip)` if the domain should be redirected for safe search.
    pub fn check(&self, domain: &str) -> Option<IpAddr> {
        if !self.enabled.load(Ordering::Relaxed) {
            return None;
        }
        self.mappings.get(&domain.to_lowercase()).copied()
    }

    /// Enable safe search rewrites.
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    /// Disable safe search rewrites.
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_rewritten() {
        let rewriter = SafeSearchRewriter::new(true);
        assert!(rewriter.check("www.google.com").is_some());
    }

    #[test]
    fn test_disabled_no_rewrite() {
        let rewriter = SafeSearchRewriter::new(false);
        assert!(rewriter.check("www.google.com").is_none());
    }

    #[test]
    fn test_non_search_no_rewrite() {
        let rewriter = SafeSearchRewriter::new(true);
        assert!(rewriter.check("example.com").is_none());
    }

    #[test]
    fn test_bing_rewritten() {
        let rewriter = SafeSearchRewriter::new(true);
        assert!(rewriter.check("bing.com").is_some());
    }

    #[test]
    fn test_youtube_rewritten() {
        let rewriter = SafeSearchRewriter::new(true);
        assert!(rewriter.check("www.youtube.com").is_some());
    }

    #[test]
    fn test_yandex_rewritten() {
        let rewriter = SafeSearchRewriter::new(true);
        assert!(rewriter.check("yandex.ru").is_some());
    }

    #[test]
    fn test_enable_disable() {
        let rewriter = SafeSearchRewriter::new(true);
        rewriter.disable();
        assert!(rewriter.check("www.google.com").is_none());
        rewriter.enable();
        assert!(rewriter.check("www.google.com").is_some());
    }
}
