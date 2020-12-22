use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocalErr {
    #[error("the key '{0:?}' had error '{1:?}'")]
    EnvErr(String, std::env::VarError),
    #[error("No enviroment keys set")]
    EnviromentKeysNone,
    #[error("glob parsing error")]
    GlobErr(#[from] glob::GlobError),
    #[error("glob pattern error")]
    GlobPatternErr(#[from] glob::PatternError),
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("xml parsing error")]
    XunitError(#[from] xunit_repo_interface::XunitError),
    #[error("unknown LocalErr error")]
    Unknown,
}
