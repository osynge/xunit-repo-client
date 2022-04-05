mod clap;
mod environment;
mod toml;
use clone_or::CloneOr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, CloneOr)]
pub struct Config {
    pub config_file: Option<String>,
    pub loglevel: Option<i8>,
    pub xunit_local_globs: Option<Vec<String>>,
    pub environment_sk: Option<String>,
    pub environment_keys: Option<Vec<String>>,
    pub project_sk: Option<String>,
    pub project_identifier: Option<String>,
    pub project_human_name: Option<String>,
    pub run_identifier: Option<String>,
    pub run_sk: Option<String>,
    pub service_url: Option<String>,
}

impl Config {
    //set the method's context
    pub fn new() -> Config {
        Config {
            config_file: None,
            loglevel: None,
            xunit_local_globs: None,
            environment_sk: None,
            environment_keys: None,
            project_sk: None,
            project_identifier: None,
            project_human_name: None,
            run_identifier: None,
            run_sk: None,
            service_url: None,
        }
    }
}

impl From<toml::ConfigFile> for Config {
    fn from(src: toml::ConfigFile) -> Self {
        Config {
            config_file: None,
            loglevel: src.loglevel,
            xunit_local_globs: src.xunit,
            environment_sk: src.environment_sk,
            environment_keys: src.environment_keys,
            project_sk: src.project_sk,
            project_identifier: src.project_identifier,
            project_human_name: src.project_human_name,
            run_identifier: None,
            run_sk: None,
            service_url: src.service_url,
        }
    }
}

#[derive(Error, Debug)]
pub(crate) enum ConfigureErr {
    #[error("File not found '{0}'.")]
    TomlErr(#[from] toml::toml::de::Error),
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("File not found '{0}'.")]
    FilePathIsNotFile(String),
}

pub(crate) fn configure() -> Result<Config, ConfigureErr> {
    let cfg_clap = clap::cli_clap();
    let cfg_env = environment::cli_env();
    let cfg_clap_env = cfg_clap.clone_or(&cfg_env);
    let cfg_file = match &cfg_clap_env.config_file {
        Some(p) => toml::load_config_from_path_string(p)?,
        None => match toml::load_config_from_default_path() {
            Ok(f) => f,
            Err(f) => Config::new(),
        },
    };
    let cfg = cfg_clap_env.clone_or(&cfg_file);
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_config_with_data_1() -> Config {
        Config {
            config_file: Some(String::from("configfile")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("xunit_local_globs")]),
            environment_sk: Some(String::from("environment_sk")),
            environment_keys: Some(vec![String::from("environment_keys")]),
            project_sk: Some(String::from("project_sk")),
            project_identifier: Some(String::from("project_identifier")),
            project_human_name: Some(String::from("project_human_name")),
            run_identifier: Some(String::from("run_identifier")),
            run_sk: Some(String::from("run_sk")),
            service_url: Some(String::from("service_url")),
        }
    }
    fn gen_config_with_data_2() -> Config {
        Config {
            config_file: Some(String::from("2")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("2")]),
            environment_sk: Some(String::from("2")),
            environment_keys: Some(vec![String::from("2")]),
            project_sk: Some(String::from("2")),
            project_identifier: Some(String::from("2")),
            project_human_name: Some(String::from("2")),
            run_identifier: Some(String::from("2")),
            run_sk: Some(String::from("2")),
            service_url: Some(String::from("2")),
        }
    }

    #[test]
    fn gets_default_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = b.clone_or(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_none_with_none() {
        let a = Config::new();
        let b = Config::new();
        let c = b.clone_or(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = a.clone_or(&b);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_some() {
        let a = gen_config_with_data_1();
        let b = gen_config_with_data_2();
        let c = a.clone_or(&b);
        assert_eq!(c, a);
    }
}
