use crate::error::LocalErr;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use thiserror::Error;
use xunit_repo_interface::{File, Xunit};
//use xunit_repo_interface::Xunit;

pub fn try_into(path: PathBuf) -> Result<xunit_repo_interface::File, LocalErr> {
    if path.exists() == false {
        return Err(LocalErr::Unknown);
    }
    if path.is_relative() == false {
        return Err(LocalErr::Unknown);
    };
    let dir = path
        .parent()
        .ok_or(LocalErr::Unknown)?
        .to_str()
        .ok_or(LocalErr::Unknown)?
        .to_string();
    let filename = path
        .file_name()
        .ok_or(LocalErr::Unknown)?
        .to_str()
        .ok_or(LocalErr::Unknown)?
        .to_string();

    let fpath = path.to_str().ok_or(LocalErr::Unknown)?;
    println!("{:?}", fpath);
    let file = std::fs::File::open(fpath)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let content = Xunit::try_from_xml(&contents)?;
    let out = xunit_repo_interface::File {
        content: content,
        filename: filename,
        directory: dir,
    };
    Ok(out)
}
