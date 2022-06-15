use std::convert::From;
use std::process::{ExitCode, Termination};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum LocalErr {
    #[error("All is good.'")]
    Good,
    #[error("configuration")]
    Config(#[from] crate::configuration::errors::ConfigureErr),
    #[error("the key '{0:?}' had error '{1:?}'")]
    EnvErr(String, std::env::VarError),
    #[error("Environment keys missing '{0:?}'")]
    EnvironmentKeysMissing(Vec<String>),
    #[error("No environment keys set")]
    EnvironmentKeysNone,
    #[error("No run identifier")]
    NoRunIdentifier,
    #[error("glob parsing error:{0:?}")]
    GlobErr(#[from] glob::GlobError),
    #[error("glob pattern error:{0:?}")]
    GlobPatternErr(#[from] glob::PatternError),
    #[error("io parsing error:{0:?}")]
    IoErr(#[from] std::io::Error),
    #[error("xml parsing error:{0:?}")]
    XunitError(#[from] xunit_repo_interface::XunitError),
    #[error("Upload failed:{0:?}")]
    Upload(String),
    #[error("Unknown error")]
    Unknown,
}

impl From<LocalErr> for u8 {
    fn from(item: LocalErr) -> u8 {
        match item {
            LocalErr::Good => 0,
            LocalErr::Config(_) => 1,
            LocalErr::EnvErr(_, _) => 5,
            LocalErr::EnvironmentKeysMissing(_) => 10,
            LocalErr::EnvironmentKeysNone => 15,
            LocalErr::NoRunIdentifier => 20,
            LocalErr::GlobErr(_) => 25,
            LocalErr::GlobPatternErr(_) => 30,
            LocalErr::IoErr(_) => 35,
            LocalErr::XunitError(_) => 40,
            LocalErr::Upload(_) => 45,
            LocalErr::Unknown => 255,
        }
    }
}

impl Termination for LocalErr {
    fn report(self) -> ExitCode {
        match self {
            LocalErr::Good => {}
            _ => log::error!("{}", self),
        }
        ExitCode::from(u8::from(self))
    }
}
