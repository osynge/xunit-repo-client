mod clap;
mod environment;
mod toml;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
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
    pub server_host: Option<String>,
    pub server_port: Option<u32>,
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
            server_host: None,
            server_port: None,
        }
    }
    pub fn copy_with_default(&self, src: &Config) -> Config {
        let config_file = match self.config_file.as_ref().or_else(|| src.config_file.as_ref()) {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let loglevel = self.loglevel.or_else(|| src.loglevel);
        let xunit_local_globs = match self
            .xunit_local_globs
            .as_ref()
            .or_else(|| src.xunit_local_globs.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let environment_sk = match self
            .environment_sk
            .as_ref()
            .or_else(|| src.environment_sk.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let environment_keys = match self
            .environment_keys
            .as_ref()
            .or_else(|| src.environment_keys.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let project_sk = match self.project_sk.as_ref().or_else(|| src.project_sk.as_ref()) {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let project_identifier = match self
            .project_identifier
            .as_ref()
            .or_else(|| src.project_identifier.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };

        let project_human_name = match self
            .project_human_name
            .as_ref()
            .or_else(|| src.project_human_name.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let run_identifier = match self
            .run_identifier
            .as_ref()
            .or_else(|| src.run_identifier.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let run_sk = match self.run_sk.as_ref().or_else(|| src.run_sk.as_ref()) {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let server_host = match self
            .server_host
            .as_ref()
            .or_else(|| src.server_host.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let server_port = self.server_port.or_else(|| src.server_port);
        Config {
            config_file,
            loglevel,
            xunit_local_globs,
            environment_sk,
            environment_keys,
            project_sk,
            project_identifier,
            project_human_name,
            run_identifier,
            run_sk,
            server_host,
            server_port,
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
    let cfg_clap_env = cfg_clap.copy_with_default(&cfg_env);
    let cfg_file = match &cfg_clap_env.config_file {
        Some(p) => toml::load_config_from_path_string(p)?,
        None => match toml::load_config_from_default_path() {
            Ok(f) => f,
            Err(f) => Config::new(),
        },
    };
    let cfg = cfg_clap_env.copy_with_default(&cfg_file);
    /*clap_fern::log_setup(&cfg);
    info!("config={:#?}", cfg);*/
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_config_with_data_1() -> Config {
        Config {
            configfile: Some(String::from("configfile")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("xunit_local_globs")]),
            environment_sk: Some(String::from("environment_sk")),
            environment_keys: Some(vec![String::from("environment_keys")]),
            project_sk: Some(String::from("project_sk")),
            project_identifier: Some(String::from("project_identifier")),
            project_human_name: Some(String::from("project_human_name")),
            run_identifier: Some(String::from("run_identifier")),
            run_sk: Some(String::from("run_sk")),
            server_host: Some(String::from("server_host")),
            server_port: Some(8080),
        }
    }
    fn gen_config_with_data_2() -> Config {
        Config {
            configfile: Some(String::from("2")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("2")]),
            environment_sk: Some(String::from("2")),
            environment_keys: Some(vec![String::from("2")]),
            project_sk: Some(String::from("2")),
            project_identifier: Some(String::from("2")),
            project_human_name: Some(String::from("2")),
            run_identifier: Some(String::from("2")),
            run_sk: Some(String::from("2")),
            server_host: Some(String::from("2")),
            server_port: Some(2),
        }
    }

    #[test]
    fn gets_default_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = b.copy_with_default(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_none_with_none() {
        let a = Config::new();
        let b = Config::new();
        let c = b.copy_with_default(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = a.copy_with_default(&b);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_some() {
        let a = gen_config_with_data_1();
        let b = gen_config_with_data_2();
        let c = a.copy_with_default(&b);
        assert_eq!(c, a);
    }
}
