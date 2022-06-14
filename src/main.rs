#[macro_use]
extern crate log;

mod assemble;
mod clap_fern;
mod configuration;
mod error;
mod into;
mod parse_glob;
mod upload;

fn run() -> Result<(), error::LocalErr> {
    let cfg = configuration::configure()?;
    clap_fern::log_setup(&cfg);
    info!("config={:#?}", cfg);

    let payload = assemble::gen_payload(&cfg)?;
    let url = cfg.service_url.expect("Service URL not set");
    if let Err(p) = upload::upload(&url, &payload) {
        return Err(error::LocalErr::Upload(format!("{}", p)));
    };
    Ok(())
}

fn main() -> error::LocalErr {
    match run() {
        Ok(_) => error::LocalErr::Good,
        Err(p) => p,
    }
}
