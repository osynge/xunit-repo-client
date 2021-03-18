pub fn upload(
    url: &String,
    payload: &xunit_repo_interface::Upload,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{url}/upload", url = url);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(&url)
        .json::<xunit_repo_interface::Upload>(&payload)
        .send();
    println!("{:#?}", resp);
    println!("{:#?}", resp.unwrap().text());
    Ok(())
}
