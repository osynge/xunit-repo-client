use crate::config;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::{crate_name, crate_version};

use std::convert::TryFrom;

fn log_setup(matches: &ArgMatches) -> Option<i8> {
    /* 0 is silent higher number more verbose */
    match (&matches.value_of("verbose"), &matches.value_of("quiet")) {
        (None, None) => return None,
        (Some(_), None) => match i8::try_from(20) {
            Ok(p) => Some(p),
            Err(_) => Some(i8::MAX),
        },
        (None, Some(_)) => match i8::try_from(matches.occurrences_of("quiet")) {
            Ok(p) => Some(p.saturating_neg()),
            Err(_) => Some(i8::MIN),
        },
        (Some(_), Some(_)) => {
            let verbose = match i8::try_from(matches.occurrences_of("verbose")) {
                Ok(p) => p,
                Err(_) => i8::MAX,
            };
            let quiet = match i8::try_from(matches.occurrences_of("quiet")) {
                Ok(p) => p.saturating_neg(),
                Err(_) => i8::MIN,
            };
            Some(verbose.saturating_sub(quiet))
        }
    }
}

pub fn cli_clap() -> crate::config::Config {
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
            Arg::with_name("enviroment_variable")
                .short("e")
                .long("env")
                .value_name("ENVIROMENT_VARIABLE")
                .help("Which enviroment variables to process")
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
            Arg::with_name("enviroment-key")
                .long("enviroment-id")
                .value_name("ENV_ID")
                .help("directory storing jobs shell scripts.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project-identifier")
                .long("project-identiifier")
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
            Arg::with_name("host")
                .long("host")
                .value_name("SERVER")
                .help("Sets the host to upload the xunit to.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .help("Sets the host's port.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("globs")
                .help("Sets the input files to use, wild cards are allowed.")
                .value_name("GLOB")
                .multiple(true),
        );

    let matches = application.get_matches();
    let loglevel = log_setup(&matches);
    let xunit_local_globs: Vec<String> = match matches.values_of("xunit") {
        Some(itr) => itr.into_iter().map(|x| String::from(x)).collect(),
        None => vec![],
    };
    let enviroment_keys: Vec<String> = match matches.values_of("enviroment_variable") {
        Some(itr) => itr.into_iter().map(|x| String::from(x)).collect(),
        None => vec![],
    };

    let configfile = match matches.value_of("config") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    let enviroment_sk = match matches.value_of("enviroment-key") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    let project_identiifier = match matches.value_of("project-identifier") {
        Some(p) => Some(String::from(p)),
        None => None,
    };

    let project_human_name = match matches.value_of("project-name") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    let server_host = match matches.value_of("host") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    let server_port = match matches.value_of("port") {
        Some(p) => match p.parse() {
            Ok(f) => Some(f),
            Err(_) => None,
        },
        None => None,
    };
    crate::config::Config {
        configfile,
        loglevel,
        xunit_local_globs,
        enviroment_sk,
        enviroment_keys,
        project_sk: None,
        project_identiifier,
        project_human_name,
        server_host,
        server_port,
    }
}
