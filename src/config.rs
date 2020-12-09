pub struct config {
    pub configfile: Option<String>,
    pub loglevel: Option<i8>,
    pub xunit_local_globs: Vec<String>,
    pub enviroment_sk: Option<String>,
    pub enviroment_keys: Vec<String>,
    pub project_sk: Option<String>,
    pub project_identiifier: Option<String>,
    pub project_human_name: Option<String>,
    pub server_host: Option<String>,
    pub server_port: Option<u32>,
}
