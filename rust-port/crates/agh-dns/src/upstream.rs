use std::time::Duration;

use async_trait::async_trait;
use hickory_proto::op::Message;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::TokioResolver;

use crate::{DnsError, server::QueryHandler};

/// Configuration for upstream DNS resolvers.
pub struct UpstreamConfig {
    pub servers: Vec<String>,
    pub timeout: Duration,
}

impl Default for UpstreamConfig {
    fn default() -> Self {
        Self {
            servers: vec!["8.8.8.8:53".to_owned(), "8.8.4.4:53".to_owned()],
            timeout: Duration::from_secs(5),
        }
    }
}

/// Upstream DNS resolver backed by `hickory-resolver`.
pub struct UpstreamResolver {
    resolver: TokioResolver,
}

impl UpstreamResolver {
    /// Create a new resolver using the system or default DNS configuration.
    pub async fn new(_config: UpstreamConfig) -> Result<Self, DnsError> {
        use hickory_resolver::name_server::TokioConnectionProvider;

        let resolver = hickory_resolver::TokioResolver::builder_with_config(
            ResolverConfig::default(),
            TokioConnectionProvider::default(),
        )
        .build();

        Ok(Self { resolver })
    }
}

#[async_trait]
impl QueryHandler for UpstreamResolver {
    async fn handle(&self, request: &Message) -> Message {
        use hickory_proto::op::{MessageType, ResponseCode};

        let query = match request.queries().first() {
            Some(q) => q.clone(),
            None => return crate::server::servfail_response(request),
        };

        let name = query.name().clone();
        let record_type = query.query_type();

        match self.resolver.lookup(name.clone(), record_type).await {
            Ok(lookup) => {
                let mut response = request.clone();
                response.set_message_type(MessageType::Response);
                response.set_response_code(ResponseCode::NoError);
                for record in lookup.records() {
                    response.add_answer(record.clone());
                }
                response
            }
            Err(_) => crate::server::servfail_response(request),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = UpstreamConfig::default();
        assert!(!cfg.servers.is_empty());
        assert!(cfg.timeout > Duration::ZERO);
    }
}
