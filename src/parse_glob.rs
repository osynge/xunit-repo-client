use glob::glob;

fn load_glob(glob_str: &str) -> Result<Vec<xunit_repo_interface::File>, crate::error::LocalErr> {
    let mut output = vec![];
    for path in glob(glob_str)? {
        let ding = path?;
        let file = crate::into::file::try_into(&ding)?;
        output.push(file);
    }
    Ok(output)
}

pub(crate) fn load_globs(
    globs: &Vec<String>,
) -> Result<Vec<xunit_repo_interface::File>, crate::error::LocalErr> {
    let mut output = vec![];
    for glob_str in globs {
        output.append(&mut load_glob(glob_str)?);
    }
    Ok(output)
}
