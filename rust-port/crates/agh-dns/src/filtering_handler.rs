use std::net::IpAddr;
use std::sync::Arc;

use async_trait::async_trait;
use hickory_proto::op::{Message, MessageType, ResponseCode};
use hickory_proto::rr::{Name, RData, Record};

use agh_filtering::matcher::{FilterResult, FilteringEngine};
use agh_filtering::safebrowsing::{SafeBrowsingChecker, SafeBrowsingResult};
use agh_filtering::safesearch::SafeSearchRewriter;

use crate::server::QueryHandler;

/// Wires the full filtering pipeline into the DNS query handler.
///
/// Filtering order:
/// 1. Safe search rewrite → return synthetic A/AAAA record
/// 2. Blocklist match → return NXDOMAIN or 0.0.0.0
/// 3. Safe browsing → return NXDOMAIN if malware/phishing
/// 4. Forward to upstream
pub struct FilteringHandler {
    engine: Arc<FilteringEngine>,
    safe_browsing: Arc<SafeBrowsingChecker>,
    safe_search: Arc<SafeSearchRewriter>,
    upstream: Arc<dyn QueryHandler>,
}

impl FilteringHandler {
    pub fn new(
        engine: Arc<FilteringEngine>,
        safe_browsing: Arc<SafeBrowsingChecker>,
        safe_search: Arc<SafeSearchRewriter>,
        upstream: Arc<dyn QueryHandler>,
    ) -> Self {
        Self { engine, safe_browsing, safe_search, upstream }
    }
}

#[async_trait]
impl QueryHandler for FilteringHandler {
    async fn handle(&self, request: &Message) -> Message {
        let query = match request.queries().first() {
            Some(q) => q.clone(),
            None => return crate::server::servfail_response(request),
        };

        let domain = query.name().to_lowercase().to_string();
        let domain = domain.trim_end_matches('.');

        // 1. Safe search rewrite.
        if let Some(safe_ip) = self.safe_search.check(domain) {
            return synthetic_a_response(request, &query.name().clone(), safe_ip);
        }

        // 2. Blocklist check.
        match self.engine.check_domain(domain) {
            FilterResult::Blocked { .. } => {
                return crate::server::nxdomain_response(request);
            }
            FilterResult::Rewrite { ip } => {
                return synthetic_a_response(request, &query.name().clone(), ip);
            }
            FilterResult::Allowed { .. } | FilterResult::NoMatch => {}
        }

        // 3. Safe browsing check.
        match self.safe_browsing.check(domain).await {
            SafeBrowsingResult::Malware | SafeBrowsingResult::Phishing => {
                return crate::server::nxdomain_response(request);
            }
            SafeBrowsingResult::Safe => {}
        }

        // 4. Forward to upstream.
        self.upstream.handle(request).await
    }
}

fn synthetic_a_response(request: &Message, name: &Name, ip: IpAddr) -> Message {
    let mut response = request.clone();
    response.set_message_type(MessageType::Response);
    response.set_response_code(ResponseCode::NoError);

    let rdata = match ip {
        IpAddr::V4(v4) => RData::A(hickory_proto::rr::rdata::A(v4)),
        IpAddr::V6(v6) => RData::AAAA(hickory_proto::rr::rdata::AAAA(v6)),
    };

    let record = Record::from_rdata(name.clone(), 300, rdata);
    response.add_answer(record);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use hickory_proto::op::{Message, MessageType, Query};
    use hickory_proto::rr::{DNSClass, Name, RecordType};

    struct EchoUpstream;

    #[async_trait]
    impl QueryHandler for EchoUpstream {
        async fn handle(&self, request: &Message) -> Message {
            let mut r = request.clone();
            r.set_message_type(MessageType::Response);
            r.set_response_code(ResponseCode::NoError);
            r
        }
    }

    fn make_query(name: &str) -> Message {
        let mut msg = Message::new();
        msg.set_message_type(MessageType::Query);
        let mut q = Query::new();
        q.set_name(Name::from_ascii(name).expect("name"));
        q.set_query_type(RecordType::A);
        q.set_query_class(DNSClass::IN);
        msg.add_query(q);
        msg
    }

    fn make_handler(rules: &[&str]) -> FilteringHandler {
        let parsed = rules.iter().filter_map(|s| agh_filtering::parser::parse_line(s)).collect();
        let engine = Arc::new(FilteringEngine::build(parsed));
        let sb = Arc::new(SafeBrowsingChecker::new(false));
        let ss = Arc::new(SafeSearchRewriter::new(false));
        let upstream = Arc::new(EchoUpstream);
        FilteringHandler::new(engine, sb, ss, upstream)
    }

    #[tokio::test]
    async fn test_blocked_domain_returns_nxdomain() {
        let handler = make_handler(&["||blocked.example.com^"]);
        let req = make_query("blocked.example.com");
        let resp = handler.handle(&req).await;
        assert_eq!(resp.response_code(), ResponseCode::NXDomain);
    }

    #[tokio::test]
    async fn test_allowed_domain_forwarded() {
        let handler = make_handler(&[]);
        let req = make_query("allowed.example.com");
        let resp = handler.handle(&req).await;
        assert_eq!(resp.response_code(), ResponseCode::NoError);
    }
}
