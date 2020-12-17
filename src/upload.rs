pub fn upload(host: &String, port: &u32, payload : &xunit_repo_interface::Upload) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{host}:{port}/upload", host = host, port = port);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(&url)
        .json::<xunit_repo_interface::Upload>(&payload)
        .send();
    println!("{:#?}", resp);
    Ok(())
}
