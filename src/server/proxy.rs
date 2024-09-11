use crate::proxy::http::http_proxy::LB;
use log::info;
use pingora_core::prelude::background_service;
use pingora_core::server::configuration::ServerConf;
use pingora_core::services::listening::Service;
use pingora_load_balancing::prelude::RoundRobin;
use pingora_load_balancing::{health_check, LoadBalancer};
use pingora_proxy::HttpProxy;
use std::sync::Arc;
use std::time::Duration;

pub fn proxy_http_service(
    my_server: &Arc<ServerConf>,
    addr: &str,
    upstream: Vec<String>,
) -> Vec<Box<dyn pingora_core::services::Service>> {
    let mut push_service: Vec<Box<dyn pingora_core::services::Service>> = vec![];
    let mut upstreams: LoadBalancer<RoundRobin> = LoadBalancer::try_from_iter(upstream).unwrap();
    // We add health check in the background so that the bad server is never selected.
    let hc = health_check::TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(Duration::from_secs(1));

    // 心跳检查
    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    push_service.push(Box::new(background));

    let mut lb = pingora_proxy::http_proxy_service(
        &my_server,
        LB {
            round_robin: upstreams,
            req_metric: prometheus::IntCounter::new("req_metric", "req_metric").unwrap(),
        },
    );
    lb.add_tcp(addr);

    info!("server start: {:?}", addr);

    push_service.push(Box::new(lb));
    push_service
}
