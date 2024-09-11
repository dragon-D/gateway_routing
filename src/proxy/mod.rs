use pingora_core::server::configuration::ServerConf;
use std::sync::Arc;

pub mod rule;

pub mod http;

use crate::server::proxy::proxy_http_service;

pub enum ServiceType {
    Http,
    Tcp,
    None,
}

pub struct ProxyLb<'a> {
    server_conf: &'a Arc<ServerConf>,
}

impl<'a> ProxyLb<'a> {
    pub fn new(server_conf: &'a Arc<ServerConf>) -> Self {
        ProxyLb { server_conf }
    }

    pub fn match_protocol(
        &self,
        address: &'a str,
        protocol: ServiceType,
        upstream: Vec<String>,
    ) -> Vec<Box<dyn pingora_core::services::Service>> {
        match protocol {
            ServiceType::Http => {
                let services = proxy_http_service(self.server_conf, address, upstream);
                services
            }
            ServiceType::Tcp => {
                // todo
                vec![]
            }
            _ => {
                vec![]
            }
        }
    }
}
