use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;

use crate::config::config::Configs;
use crate::proxy::ProxyLb;

use crate::server::proxy::proxy_http_service;

pub struct AppData {
    config: Configs,
    opt: Opt,
}

impl AppData {
    pub fn new(config: Configs, opt: Opt) -> Self {
        AppData { config, opt }
    }
}

pub fn server_run(app: AppData) {
    // read command line arguments

    let mut my_server = Server::new(Some(app.opt)).unwrap();
    my_server.bootstrap();

    // load balancer hosts
    let servers = app.config.get_server();

    let mut pro_servers: Vec<Box<dyn pingora_core::services::Service>> = vec![];
    let proxy_server = ProxyLb::new(&my_server.configuration);
    for server in servers {
        let proxy_rule = server.get_upstream();
        let service_protocol =
            proxy_server.match_protocol(server.get_address(), server.get_protocol(), proxy_rule);
        pro_servers.extend(service_protocol);
    }

    let mut prometheus_service_http =
        pingora::services::listening::Service::prometheus_http_service();
    prometheus_service_http.add_tcp("127.0.0.1:6192");
    my_server.add_service(prometheus_service_http);

    my_server.add_services(pro_servers);
    my_server.run_forever();
}
