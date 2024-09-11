use clap::{Arg, Command};

#[allow(dead_code)]
pub fn args() -> Command {
    Command::new("gateway-proxy-service")
        .version("0.0.1-2024-03-12") // tmp solution to mark every distinct version
        .author("dagon")
        .about("gateway proxy service")
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .help("set config file path")
                .default_value("src/config/config.yaml"),
        )
}
