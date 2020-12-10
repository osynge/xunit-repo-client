mod clap_fern;
mod cli_clap;
mod config;
mod error;
mod into;
mod parse_glob;
mod upload;

fn main() {
    let cfg = cli_clap::cli_clap();
    clap_fern::log_setup(&cfg);

    let _output = parse_glob::load_globs(&cfg.xunit_local_globs);
    println!("{:#?}", _output);
    let host = cfg.server_host.expect("Hostname not set");
    let port = cfg.server_port.expect("Port not set");
    upload::upload(&host, &port);
}
