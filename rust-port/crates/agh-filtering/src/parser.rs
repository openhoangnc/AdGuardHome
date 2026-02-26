use std::net::IpAddr;

/// A single parsed filtering rule.
#[derive(Debug, Clone)]
pub enum FilterRule {
    /// `||example.com^` — AdBlock domain block
    DomainBlock { domain: String },
    /// `@@||example.com^` — AdBlock domain allow (whitelist)
    DomainAllow { domain: String },
    /// `*.example.com` — wildcard subdomain block
    WildcardBlock { pattern: String },
    /// `/regex/` — regex rule
    Regex { pattern: String },
    /// Hosts-format: `0.0.0.0 example.com`
    HostsEntry { ip: IpAddr, domain: String },
    /// `$dnsrewrite=1.2.3.4,domain=example.com`
    Rewrite { domain: String, ip: IpAddr },
}

/// Statistics about a filter parse operation.
#[derive(Debug, Default)]
pub struct ParseStats {
    pub total_lines: usize,
    pub valid_rules: usize,
    pub invalid_lines: usize,
}

/// Parse a single filter rule line.
/// Returns `None` for comments and empty lines.
pub fn parse_line(line: &str) -> Option<FilterRule> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('!') || line.starts_with('#') {
        return None;
    }

    // AdBlock allowlist: @@||domain^
    if let Some(rest) = line.strip_prefix("@@||") {
        let domain = strip_adblock_anchors(rest);
        if !domain.is_empty() {
            return Some(FilterRule::DomainAllow { domain: domain.to_lowercase() });
        }
    }

    // AdBlock block: ||domain^
    if let Some(rest) = line.strip_prefix("||") {
        let domain = strip_adblock_anchors(rest);
        if !domain.is_empty() {
            return Some(FilterRule::DomainBlock { domain: domain.to_lowercase() });
        }
    }

    // Regex rule: /pattern/
    if line.starts_with('/') && line.ends_with('/') && line.len() > 2 {
        let pattern = &line[1..line.len() - 1];
        return Some(FilterRule::Regex { pattern: pattern.to_owned() });
    }

    // Wildcard: *.domain.com
    if line.starts_with("*.") {
        return Some(FilterRule::WildcardBlock { pattern: line.to_lowercase() });
    }

    // $dnsrewrite modifier
    if line.contains("$dnsrewrite=") {
        if let Some(rewrite) = parse_dnsrewrite(line) {
            return Some(rewrite);
        }
    }

    // Hosts-format: IP domain [aliases...]
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        if let Ok(ip) = parts[0].parse::<IpAddr>() {
            let domain = parts[1].to_lowercase();
            if !domain.is_empty() {
                return Some(FilterRule::HostsEntry { ip, domain });
            }
        }
    }

    None
}

fn strip_adblock_anchors(s: &str) -> &str {
    // Remove trailing `^` and optional `$...` options
    let s = s.split('$').next().unwrap_or(s);
    s.trim_end_matches('^').trim_end_matches('/')
}

fn parse_dnsrewrite(line: &str) -> Option<FilterRule> {
    // Format: ||domain^$dnsrewrite=<ip>
    let (prefix, options) = line.split_once('$')?;
    let domain = if let Some(rest) = prefix.strip_prefix("||") {
        strip_adblock_anchors(rest).to_lowercase()
    } else {
        return None;
    };
    for opt in options.split(',') {
        if let Some(val) = opt.strip_prefix("dnsrewrite=") {
            if let Ok(ip) = val.parse::<IpAddr>() {
                return Some(FilterRule::Rewrite { domain, ip });
            }
        }
    }
    None
}

/// Parse a complete filter file, returning all valid rules.
pub fn parse_filter(content: &str) -> Vec<FilterRule> {
    parse_filter_with_stats(content).0
}

/// Parse a complete filter file and return rules with statistics.
pub fn parse_filter_with_stats(content: &str) -> (Vec<FilterRule>, ParseStats) {
    let mut rules = Vec::new();
    let mut stats = ParseStats::default();

    for line in content.lines() {
        stats.total_lines += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('!') || trimmed.starts_with('#') {
            continue;
        }
        match parse_line(line) {
            Some(rule) => {
                stats.valid_rules += 1;
                rules.push(rule);
            }
            None => {
                stats.invalid_lines += 1;
            }
        }
    }

    (rules, stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adblock_domain() {
        let rule = parse_line("||example.com^").unwrap();
        assert!(matches!(rule, FilterRule::DomainBlock { ref domain } if domain == "example.com"));
    }

    #[test]
    fn test_adblock_exception() {
        let rule = parse_line("@@||safe.example.com^").unwrap();
        assert!(matches!(rule, FilterRule::DomainAllow { .. }));
    }

    #[test]
    fn test_hosts_entry() {
        let rule = parse_line("0.0.0.0 ads.example.com").unwrap();
        assert!(matches!(rule, FilterRule::HostsEntry { .. }));
    }

    #[test]
    fn test_comment_skipped() {
        assert!(parse_line("! This is a comment").is_none());
        assert!(parse_line("# Also a comment").is_none());
        assert!(parse_line("").is_none());
    }

    #[test]
    fn test_wildcard() {
        let rule = parse_line("*.example.com").unwrap();
        assert!(matches!(rule, FilterRule::WildcardBlock { .. }));
    }

    #[test]
    fn test_regex_rule() {
        let rule = parse_line("/ads\\.example\\.com/").unwrap();
        assert!(matches!(rule, FilterRule::Regex { .. }));
    }

    #[test]
    fn test_parse_filter() {
        let content = "||ads.example.com^\n! comment\n0.0.0.0 tracker.com\n";
        let rules = parse_filter(content);
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_real_easylist_sample() {
        let content = include_str!("../tests/fixtures/easylist_sample.txt");
        let rules = parse_filter(content);
        assert!(!rules.is_empty());
    }
}
