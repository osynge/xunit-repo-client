#[derive(Debug, Clone)]
pub struct Config {
    pub configfile: Option<String>,
    pub loglevel: Option<i8>,
    pub xunit_local_globs: Option<Vec<String>>,
    pub enviroment_sk: Option<String>,
    pub enviroment_keys: Option<Vec<String>>,
    pub project_sk: Option<String>,
    pub project_identiifier: Option<String>,
    pub project_human_name: Option<String>,
    pub server_host: Option<String>,
    pub server_port: Option<u32>,
}


impl Config {
    //set the method's context
    pub fn new() -> Config {
        Config {
            configfile: None,
            loglevel: None,
            xunit_local_globs: None,
            enviroment_sk: None,
            enviroment_keys: None,
            project_sk: None,
            project_identiifier: None,
            project_human_name: None,
            server_host: None,
            server_port: None,
        }
    }
    pub fn default(&self, src: &Config) -> Config {
        let mut out = self.clone();
        if out.configfile.is_none() {
            out.configfile = src.configfile.clone();
        };
        if out.loglevel.is_none() {
            out.loglevel = src.loglevel;
        };
        if out.xunit_local_globs.is_none() {
            out.xunit_local_globs = src.xunit_local_globs.clone();
        };
        if out.enviroment_sk.is_none() {
            out.enviroment_sk = src.enviroment_sk.clone();
        };
        if out.enviroment_keys.is_none() {
            out.enviroment_keys = src.enviroment_keys.clone();
        };
        if out.project_sk.is_none() {
            out.project_sk = src.project_sk.clone();
        };
        if out.project_identiifier.is_none() {
            out.project_identiifier = src.project_identiifier.clone();
        };
        if out.project_human_name.is_none() {
            out.project_human_name = src.project_human_name.clone();
        };
        if out.server_host.is_none() {
            out.server_host = src.server_host.clone();
        };
        if out.server_port.is_none() {
            out.server_port = src.server_port;
        };
        out

    }
}
