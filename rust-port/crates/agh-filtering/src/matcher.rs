use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::Arc;

use aho_corasick::AhoCorasick;
use regex::Regex;

use crate::parser::FilterRule;

/// The action associated with a rule.
#[derive(Clone, Debug)]
pub enum RuleAction {
    Block,
    Allow,
}

/// Result of checking a domain against the filtering engine.
#[derive(Debug)]
pub enum FilterResult {
    Blocked { matched_rule: String },
    Allowed { matched_rule: String },
    Rewrite { ip: IpAddr },
    NoMatch,
}

/// High-performance DNS filtering engine.
///
/// Thread-safe (`Send + Sync`) — safe to share via `Arc` across tokio tasks.
pub struct FilteringEngine {
    exact_block: HashSet<String>,
    exact_allow: HashSet<String>,
    /// Wildcard patterns stored as the suffix to match (e.g. `.example.com`).
    wildcard_block: Vec<String>,
    wildcard_allow: Vec<String>,
    regexes: Vec<(Regex, RuleAction)>,
    rewrites: HashMap<String, IpAddr>,
    /// AhoCorasick for fast substring pre-filtering.
    ac: Option<Arc<AhoCorasick>>,
    /// The patterns corresponding to each AC pattern index.
    ac_domains: Vec<String>,
    rule_count: usize,
}

// SAFETY: AhoCorasick is Send + Sync.
unsafe impl Send for FilteringEngine {}
unsafe impl Sync for FilteringEngine {}

impl FilteringEngine {
    /// Build a `FilteringEngine` from a list of parsed rules.
    pub fn build(rules: Vec<FilterRule>) -> Self {
        let mut exact_block = HashSet::new();
        let mut exact_allow = HashSet::new();
        let mut wildcard_block = Vec::new();
        let wildcard_allow = Vec::new();
        let mut regexes = Vec::new();
        let mut rewrites = HashMap::new();
        let mut ac_domains: Vec<String> = Vec::new();

        for rule in &rules {
            match rule {
                FilterRule::DomainBlock { domain } => {
                    exact_block.insert(domain.clone());
                    ac_domains.push(domain.clone());
                }
                FilterRule::DomainAllow { domain } => {
                    exact_allow.insert(domain.clone());
                }
                FilterRule::WildcardBlock { pattern } => {
                    // `*.example.com` → store `.example.com` for suffix check
                    let suffix = pattern.trim_start_matches('*').to_owned();
                    wildcard_block.push(suffix);
                }
                FilterRule::Regex { pattern } => {
                    if let Ok(re) = Regex::new(pattern) {
                        regexes.push((re, RuleAction::Block));
                    }
                }
                FilterRule::HostsEntry { domain, ip } => {
                    exact_block.insert(domain.clone());
                    rewrites.insert(domain.clone(), *ip);
                }
                FilterRule::Rewrite { domain, ip } => {
                    rewrites.insert(domain.clone(), *ip);
                }
            }
        }

        let ac = if ac_domains.is_empty() {
            None
        } else {
            AhoCorasick::new(&ac_domains).ok().map(Arc::new)
        };

        let rule_count = rules.len();

        Self {
            exact_block,
            exact_allow,
            wildcard_block,
            wildcard_allow,
            regexes,
            rewrites,
            ac,
            ac_domains,
            rule_count,
        }
    }

    /// Check a domain name against the filtering rules.
    ///
    /// Priority:
    /// 1. Allowlist exact → `Allowed`
    /// 2. Exact block → `Blocked`
    /// 3. Wildcard allow → `Allowed`
    /// 4. Wildcard block → `Blocked`
    /// 5. Regex → `Blocked`/`Allowed`
    /// 6. AhoCorasick substring (domain-boundary verified) → `Blocked`
    /// 7. `NoMatch`
    pub fn check_domain(&self, domain: &str) -> FilterResult {
        let domain = domain.to_lowercase();
        let domain = domain.trim_end_matches('.');

        // 1. Allowlist exact.
        if self.exact_allow.contains(domain) {
            return FilterResult::Allowed { matched_rule: domain.to_owned() };
        }

        // 2. Exact block / rewrite.
        if let Some(&ip) = self.rewrites.get(domain) {
            return FilterResult::Rewrite { ip };
        }
        if self.exact_block.contains(domain) {
            return FilterResult::Blocked { matched_rule: domain.to_owned() };
        }

        // Also check parent domains for block rules (||example.com^ blocks sub.example.com).
        for block_domain in &self.exact_block {
            if domain.ends_with(&format!(".{block_domain}")) {
                return FilterResult::Blocked { matched_rule: block_domain.clone() };
            }
        }

        // 3. Wildcard allow.
        for suffix in &self.wildcard_allow {
            if domain == suffix.trim_start_matches('.') || domain.ends_with(suffix.as_str()) {
                return FilterResult::Allowed { matched_rule: format!("*{suffix}") };
            }
        }

        // 4. Wildcard block.
        for suffix in &self.wildcard_block {
            if domain == suffix.trim_start_matches('.') || domain.ends_with(suffix.as_str()) {
                return FilterResult::Blocked { matched_rule: format!("*{suffix}") };
            }
        }

        // 5. Regex.
        for (re, action) in &self.regexes {
            if re.is_match(domain) {
                let rule = re.as_str().to_owned();
                return match action {
                    RuleAction::Block => FilterResult::Blocked { matched_rule: rule },
                    RuleAction::Allow => FilterResult::Allowed { matched_rule: rule },
                };
            }
        }

        // 6. AhoCorasick substring with domain-boundary check.
        if let Some(ac) = &self.ac {
            for m in ac.find_iter(domain) {
                let pattern = &self.ac_domains[m.pattern().as_usize()];
                // Accept only if the match is a full domain or sub-domain boundary.
                if is_domain_match(domain, pattern) {
                    return FilterResult::Blocked { matched_rule: pattern.clone() };
                }
            }
        }

        FilterResult::NoMatch
    }

    /// Return the total number of rules loaded into this engine.
    pub fn rule_count(&self) -> usize {
        self.rule_count
    }
}

/// Returns true if `pattern` is a complete domain match within `domain`.
fn is_domain_match(domain: &str, pattern: &str) -> bool {
    if domain == pattern {
        return true;
    }
    // sub.pattern → domain ends with ".pattern"
    domain.ends_with(&format!(".{pattern}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_line;

    fn build_engine(rules_str: &[&str]) -> FilteringEngine {
        let rules = rules_str
            .iter()
            .filter_map(|s| parse_line(s))
            .collect();
        FilteringEngine::build(rules)
    }

    #[test]
    fn test_exact_block() {
        let engine = build_engine(&["||ads.example.com^"]);
        assert!(matches!(
            engine.check_domain("ads.example.com"),
            FilterResult::Blocked { .. }
        ));
    }

    #[test]
    fn test_subdomain_blocked_by_parent() {
        let engine = build_engine(&["||example.com^"]);
        assert!(matches!(
            engine.check_domain("sub.example.com"),
            FilterResult::Blocked { .. }
        ));
    }

    #[test]
    fn test_allowlist_overrides_block() {
        let engine = build_engine(&["||example.com^", "@@||safe.example.com^"]);
        assert!(matches!(engine.check_domain("safe.example.com"), FilterResult::Allowed { .. }));
        assert!(matches!(engine.check_domain("ads.example.com"), FilterResult::Blocked { .. }));
    }

    #[test]
    fn test_no_match() {
        let engine = build_engine(&["||ads.example.com^"]);
        assert!(matches!(engine.check_domain("google.com"), FilterResult::NoMatch));
    }

    #[test]
    fn test_wildcard_block() {
        let engine = build_engine(&["*.adverts.example.com"]);
        assert!(matches!(
            engine.check_domain("sub.adverts.example.com"),
            FilterResult::Blocked { .. }
        ));
    }

    #[test]
    fn test_hosts_entry_block() {
        let engine = build_engine(&["0.0.0.0 bad.example.com"]);
        assert!(matches!(
            engine.check_domain("bad.example.com"),
            FilterResult::Blocked { .. } | FilterResult::Rewrite { .. }
        ));
    }

    #[test]
    fn test_rule_count() {
        let engine = build_engine(&["||a.com^", "||b.com^", "@@||safe.com^"]);
        assert_eq!(engine.rule_count(), 3);
    }
}
