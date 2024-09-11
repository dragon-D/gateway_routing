use crate::proxy::rule::RuleCondition;
use async_trait::async_trait;
use log::{debug, info};
use pingora_core::prelude::HttpPeer;
use pingora_http::ResponseHeader;
use pingora_load_balancing::prelude::RoundRobin;
use pingora_load_balancing::{Backend, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};
use prometheus::core::Metric;
use std::sync::Arc;

pub struct LB {
    pub round_robin: Arc<LoadBalancer<RoundRobin>>,
    pub req_metric: prometheus::IntCounter,
}

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        info!("request filter");
        // let _ = _session.respond_error(403).await;
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut (),
    ) -> pingora::Result<Box<HttpPeer>> {
        let mut upstream = self
            .round_robin
            .select(b"", 256) // hash doesn't matter
            .unwrap();

        let locations = RuleCondition::get_read_self().get_rule();

        for location in locations {
            if _session
                .req_header()
                .uri
                .path()
                .starts_with(location.match_path.as_str())
            {
                if location.proxy_url.is_some() {
                    let req_hander = _session.req_header_mut();
                    let uri = location.proxy_url.unwrap().parse::<http::Uri>().unwrap();
                    req_hander.set_uri(uri);
                }
                if location.proxy_host.is_some() {
                    upstream = Backend::new(location.proxy_host.unwrap().as_str()).unwrap();
                }

                continue;
            }
        }

        //Backend::new("127.0.0.1:8080").unwrap()
        info!(
            "uri: {:?}; upstream peer is: {:?}",
            _session.req_header().uri,
            upstream
        );

        let peer = Box::new(HttpPeer::new(
            upstream,
            false,
            "one.one.one.one".to_string(),
        ));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();
        Ok(())
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        _upstream_response.insert_header("test", "test").unwrap();
        Ok(())
    }

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        // access log
        info!(
            "{} response code: {response_code}",
            self.request_summary(session, ctx)
        );

        self.req_metric.inc();
    }
}
