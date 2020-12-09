use crate::error::LocalErr;
use crate::into::file;
use glob::glob;
use xunit_repo_interface;

fn load_glob(glob_str: &String) -> Result<Vec<xunit_repo_interface::File>, crate::error::LocalErr> {
    let mut output = vec![];
    for path in glob(glob_str)? {
        let ding = path?;
        let file = crate::into::file::try_into(ding)?;
        output.push(file);
    }
    Ok(output)
}

pub fn load_globs(
    globs: &Vec<String>,
) -> Result<Vec<xunit_repo_interface::File>, crate::error::LocalErr> {
    let mut output = vec![];
    for glob_str in globs {
        output.append(&mut load_glob(glob_str)?);
    }
    Ok(output)
}
