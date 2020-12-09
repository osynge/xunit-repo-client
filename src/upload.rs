use std::{collections::HashMap, intrinsics::copy_nonoverlapping};
use xunit_repo_interface;
pub fn upload(host: &String, port: &u32) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{host}:{port}/upload", host = host, port = port);
    let jam = xunit_repo_interface::Run {
        sk: None,
        client_identifier: None,
    };
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(&url)
        .json::<xunit_repo_interface::Run>(&jam)
        .send();
    println!("{:#?}", resp);
    Ok(())
}
