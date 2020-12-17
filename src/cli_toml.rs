use crate::config;
use dirs::home_dir;
use serde_derive::Deserialize;
use std::io::{BufRead, BufReader};
use std::path::Path;
use thiserror::Error;
use toml;

#[derive(Deserialize)]
pub struct ConfigFile {
    pub loglevel: Option<i8>,
    pub xunit: Option<Vec<String>>,
    pub enviroment_sk: Option<String>,
    pub enviroment_keys: Option<Vec<String>>,
    pub project_sk: Option<String>,
    pub project_identiifier: Option<String>,
    pub project_human_name: Option<String>,
    pub server_host: Option<String>,
    pub server_port: Option<u32>,
}

impl Into<config::Config> for ConfigFile {
    fn into(self) -> config::Config {
        config::Config {
            configfile: None,
            loglevel: self.loglevel,
            xunit_local_globs: self.xunit,
            enviroment_sk: self.enviroment_sk,
            enviroment_keys: self.enviroment_keys,
            project_sk: self.project_sk,
            project_identiifier: self.project_identiifier,
            project_human_name: self.project_human_name,
            server_host: self.server_host,
            server_port: self.server_port,
        }
    }
}
#[derive(Error, Debug)]
pub enum CliTomlErr {
    #[error("File not found '{0}'.")]
    TomlErr(#[from] toml::de::Error),
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("File not found '{0}'.")]
    FilePathIsNotFile(String),
}

pub fn load_config_from_path_string(input_path: &String) -> Result<config::Config, CliTomlErr> {
    let path = Path::new(input_path);
    if !path.is_file() {
        return Err(CliTomlErr::FilePathIsNotFile(String::from(input_path)));
    }
    let toml_str = std::fs::read_to_string(&path)?;
    let cf: ConfigFile = toml::from_str(&toml_str)?;
    Ok(cf.into())
}

pub fn load_config_from_default_path() -> Result<config::Config, ()> {
    let path = String::from("/etc/xunit-repo-client.toml");
    if let Ok(cfg) = load_config_from_path_string(&path) {
        return Ok(cfg);
    };
    if let Some(mut dirhome) = dirs::home_dir() {
        dirhome.push(".xunit-repo-client.toml");
        if let Some(cfg_path_str ) = dirhome.to_str() {

            let cfg_path = String::from(cfg_path_str);
            if let Ok(cfg) = load_config_from_path_string(&cfg_path) {
                print!("cfg");
                return Ok(cfg);
            };
        }
    }
    Err(())
}
