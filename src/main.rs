// Copyright 2024 Cloudflare, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod b;
mod config;
mod proxy;
mod server;

use log::info;
use pingora_core::server::configuration::Opt;
use structopt::StructOpt;

use crate::config::config::Configs;
use crate::proxy::rule::RuleCondition;
use crate::server::service::server_run;

// RUST_LOG=INFO cargo run --example load_balancer
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    //配置加载

    let opt = Opt::from_args();
    let config = Configs::load_yaml_with_opt_override(opt.conf.clone()).unwrap();

    // 规则加载
    let locations = config.get_server()[0].get_location().unwrap();
    RuleCondition::load_rule(locations);

    info!("rule {:?}", RuleCondition::get_read_self().get_rule());


    let app = server::service::AppData::new(config, opt);
    server_run(app);
}
