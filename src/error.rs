use glob::{GlobError, PatternError};
use thiserror::Error;
use xunit_repo_interface::{File, Xunit, XunitError};

#[derive(Error, Debug)]
pub enum LocalErr {
    #[error("glob parsing error")]
    GlobErr(#[from] GlobError),
    #[error("glob pattern error")]
    GlobPatternErr(#[from] PatternError),
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("xml parsing error")]
    XunitError(#[from] xunit_repo_interface::XunitError),
    #[error("unknown LocalErr error")]
    Unknown,
}
