use xunit_repo_interface::{File,Xunit, XunitError};
use std::path::PathBuf;
use std::result::Result;
use std::path::Path;
use thiserror::Error;
use std::io::BufReader;
use std::io::Read;
//use xunit_repo_interface::Xunit;

#[derive(Error, Debug)]
pub enum LocalErr {
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("xml parsing error")]
    XunitError(#[from] xunit_repo_interface::XunitError),

    #[error("unknown LocalErr error")]
    Unknown,
}


pub fn try_into(path : PathBuf) -> Result<xunit_repo_interface::File,LocalErr> {
    if path.exists() == false {
        return Err(LocalErr::Unknown)
    }
    if path.is_relative() == false {
        return Err(LocalErr::Unknown);
    };
    let dir  = path.parent().ok_or(LocalErr::Unknown)?
        .to_str().ok_or(LocalErr::Unknown)?.to_string();
    let filename = path.file_name().ok_or(LocalErr::Unknown)?
        .to_str().ok_or(LocalErr::Unknown )?.to_string();


    let fpath = path.to_str().ok_or(LocalErr::Unknown )?;
    println!("{:?}", fpath);
    let file = std::fs::File::open(fpath)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let content = Xunit::try_from_xml(&contents)?;
    let out = xunit_repo_interface::File{
        content : content,
        filename : filename,
        directory: dir,
    };
    Ok(out)
}