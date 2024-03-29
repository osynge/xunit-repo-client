use crate::error::LocalErr;
use std::io::Read;
use std::path::Path;
use std::result::Result;

pub(crate) fn try_into(path: &Path) -> Result<xunit_repo_interface::File, LocalErr> {
    if !path.exists() {
        return Err(LocalErr::Unknown);
    }
    if !path.is_relative() {
        return Err(LocalErr::Unknown);
    };
    let directory = path
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
    let content = xunit_repo_interface::Xunit::try_from_xml(&contents)?;
    let out = xunit_repo_interface::File {
        content,
        filename,
        directory,
    };
    Ok(out)
}
