use clap::App;
use clap::Arg;
use clap::{crate_name, crate_version};

use std::convert::TryFrom;

fn log_setup(verbose_occurrences: u64, quiet_occurrences: u64) -> Option<i8> {
    if (0, 0) == (verbose_occurrences, quiet_occurrences) {
        return None;
    };
    let verbose = match i8::try_from(verbose_occurrences) {
        Ok(p) => p,
        Err(_) => i8::MAX,
    };
    let quiet = match i8::try_from(quiet_occurrences) {
        Ok(p) => p,
        Err(_) => i8::MAX,
    };
    Some(verbose.saturating_sub(quiet))
}

pub fn cli_clap() -> crate::configuration::Config {
    let application = App::new(crate_name!())
        .about("Parses an input file to do awesome things")
        .version(crate_version!())
        .author("Owen Synge <osynge@googlemail.com>")
        .arg(
            Arg::with_name("verbose")
                .help("Increase log output.")
                .short("v")
                .multiple(true)
                .long("verbose"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Decrease log output.")
                .short("q")
                .multiple(true)
                .long("quiet"),
        )
        .arg(
            Arg::with_name("environment_variable")
                .short("e")
                .long("env")
                .value_name("ENVIROMENT_VARIABLE")
                .help("Which environment variables to process")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("xunit")
                .long("xunit")
                .value_name("XUNIT_FILES")
                .help("glob of xunit files")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("environment-key")
                .long("environment-id")
                .value_name("ENV_ID")
                .help("directory storing jobs shell scripts.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project-identifier")
                .long("project-identifier")
                .value_name("PROJECT_IDENTIFIER")
                .help("An identifier for the project such as VCS checkout url")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project-name")
                .long("project-name")
                .value_name("PROJECT_NAME")
                .help("A name for the project.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("run-identifier")
                .long("run-identifier")
                .value_name("RUN_IDENTIFIER")
                .help("An identifier for the run such as jenkins build number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("run-key")
                .long("run-key")
                .value_name("RUN_KEY")
                .help("The key for the run")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("service-url")
                .long("url")
                .value_name("SERVICE_URL")
                .help("Sets the service url to upload the xunit to.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("globs")
                .help("Sets the input files to use, wild cards are allowed.")
                .value_name("GLOB")
                .multiple(true),
        );

    let matches = application.get_matches();
    let loglevel = log_setup(
        matches.occurrences_of("verbose"),
        matches.occurrences_of("quiet"),
    );
    let xunit_local_globs = matches
        .values_of("xunit")
        .map(|itr| itr.into_iter().map(String::from).collect());
    let environment_keys = matches
        .values_of("environment_variable")
        .map(|itr| itr.into_iter().map(String::from).collect());
    let config_file = matches.value_of("config").map(String::from);
    let environment_sk = matches.value_of("environment-key").map(String::from);
    let project_identifier = matches.value_of("project-identifier").map(String::from);
    let project_human_name = matches.value_of("project-name").map(String::from);
    let run_identifier = matches.value_of("run-identifier").map(String::from);
    let service_url = matches.value_of("service-url").map(String::from);
    crate::configuration::Config {
        config_file,
        loglevel,
        xunit_local_globs,
        environment_sk,
        environment_keys,
        project_sk: None,
        project_identifier,
        project_human_name,
        run_identifier,
        run_sk: None,
        service_url,
    }
}
