#[macro_use]
extern crate log;
mod assemble;
mod clap_fern;
mod cli_clap;
mod cli_env;
mod cli_toml;
mod config;
mod error;
mod into;
mod parse_glob;
mod upload;

fn run() -> i32 {
    let cfg_clap = cli_clap::cli_clap();
    let cfg_env = cli_env::cli_env();
    let cfg_clap_env = cfg_clap.copy_with_default(&cfg_env);
    let cfg_file = match &cfg_clap_env.configfile {
        Some(p) => match cli_toml::load_config_from_path_string(p) {
            Ok(f) => f,
            Err(f) => {
                clap_fern::log_setup(&cfg_clap_env);
                error!("{}", f);
                return 6;
            }
        },
        None => match cli_toml::load_config_from_default_path() {
            Ok(f) => f,
            Err(f) => config::Config::new(),
        },
    };
    let cfg = cfg_clap_env.copy_with_default(&cfg_file);
    clap_fern::log_setup(&cfg);
    info!("config={:#?}", cfg);

    let payload = match assemble::gen_payload(&cfg) {
        Ok(p) => p,
        Err(fail) => {
            error!("{}", fail);
            return 7;
        }
    };
    let host = cfg.server_host.expect("Hostname not set");
    let port = cfg.server_port.expect("Port not set");
    upload::upload(&host, &port, &payload);
    0
}

fn main() {
    std::process::exit(run())
}
