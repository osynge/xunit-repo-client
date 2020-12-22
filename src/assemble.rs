use crate::error::LocalErr;
use crate::parse_glob;
use std::collections::HashMap;
use std::env;
use xunit_repo_interface;

fn gen_project(cfg: &crate::config::Config) -> xunit_repo_interface::Project {
    let sk = match cfg.project_sk.as_ref() {
        Some(p) => Some(p.clone()),
        None => None,
    };
    let human_name = match cfg.project_human_name.as_ref() {
        Some(p) => Some(p.clone()),
        None => None,
    };
    let identiifier = match cfg.project_identiifier.as_ref() {
        Some(p) => Some(p.clone()),
        None => None,
    };
    xunit_repo_interface::Project {
        sk,
        human_name,
        identiifier,
    }
}

fn gen_enviroment(
    cfg: &crate::config::Config,
) -> Result<xunit_repo_interface::Enviroment, LocalErr> {
    let sk = match cfg.enviroment_sk.as_ref() {
        Some(p) => Some(p.clone()),
        None => None,
    };
    let mut key_value = HashMap::new();
    for key in cfg
        .enviroment_keys
        .as_ref()
        .ok_or(LocalErr::EnviromentKeysNone)?
    {
        let value = match env::var(key) {
            Ok(value) => value,
            Err(e) => {
                return Err(LocalErr::EnvErr(key.clone(), e));
            }
        };
        key_value.insert(key.clone(), value);
    }
    Ok(xunit_repo_interface::Enviroment { sk, key_value })
}

fn gen_run(cfg: &crate::config::Config) -> xunit_repo_interface::Run {
    let sk = match cfg.project_identiifier.as_ref() {
        Some(p) => Some(p.clone()),
        None => None,
    };
    let client_identifier = match cfg.project_identiifier.as_ref() {
        Some(p) => Some(p.clone()),
        None => None,
    };
    xunit_repo_interface::Run {
        sk,
        client_identifier,
    }
}

pub fn gen_payload(cfg: &crate::config::Config) -> Result<xunit_repo_interface::Upload, LocalErr> {
    let project = gen_project(cfg);
    let enviroment = gen_enviroment(cfg)?;
    let run = gen_run(cfg);
    let files = match cfg.xunit_local_globs.as_ref() {
        Some(f) => parse_glob::load_globs(f)?,
        None => vec![],
    };
    Ok(xunit_repo_interface::Upload {
        project,
        enviroment,
        run,
        files,
    })
}
