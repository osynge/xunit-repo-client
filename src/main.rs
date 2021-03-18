#[macro_use]
extern crate log;
mod assemble;
mod clap_fern;
mod configuration;
mod error;
mod into;
mod parse_glob;
mod upload;

fn run() -> i32 {
    let cfg = match configuration::configure() {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return 2;
        }
    };
    clap_fern::log_setup(&cfg);
    info!("config={:#?}", cfg);

    let payload = match assemble::gen_payload(&cfg) {
        Ok(p) => p,
        Err(fail) => {
            error!("{}", fail);
            return 7;
        }
    };
    let url = cfg.service_url.expect("Service URL not set");
    upload::upload(&url, &payload);
    0
}

fn main() {
    std::process::exit(run())
}
