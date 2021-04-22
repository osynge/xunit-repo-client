use std::time::Duration;

pub fn upload(
    url: &String,
    payload: &xunit_repo_interface::Upload,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{url}/upload", url = url);
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(600))
        .build()?;
    let resp = client
        .post(&url)
        .json::<xunit_repo_interface::Upload>(&payload)
        .send();
    println!("{:#?}", resp);
    let text = match resp {
        Ok(p) => p.text(),
        Err(_) => Ok(String::from("")),
    };
    println!("resp.text={:#?}", text);

    Ok(())
}
