use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocalErr {
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
