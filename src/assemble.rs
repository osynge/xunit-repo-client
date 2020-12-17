use std::collections::HashMap;

use crate::parse_glob;
use crate::error::LocalErr;
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

fn gen_enviroment(cfg: &crate::config::Config) -> xunit_repo_interface::Enviroment {
    let sk = match cfg.project_identiifier.as_ref() {
        Some(p) => Some(p.clone()),
        None => None,
    };
    let key_value = HashMap::new();
    xunit_repo_interface::Enviroment { sk, key_value }
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
    let enviroment = gen_enviroment(cfg);
    let run = gen_run(cfg);
    let files = match cfg.xunit_local_globs.as_ref() {
        Some(f) => parse_glob::load_globs(f)?,
        None => vec![]

    };
    Ok(xunit_repo_interface::Upload{
        project,
        enviroment,
        run,
        files,
    })
}
