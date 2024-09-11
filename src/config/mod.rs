mod args;
pub mod config;

use crate::config::config::Configs;

pub fn get_config() -> Configs {
    let app = args::args();
    let am = app.get_matches();

    // read config file
    // let config_file_path = am.value_parser("config-file").unwrap();
    let config_file_path = am.get_one::<String>("config-file").unwrap();
    let config = config::get_config_struct(config_file_path.to_string()).unwrap();
    println!("{:?}", config);
    config
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_get_config() {
        // println!("test_get_config");
        get_config();
    }
}
