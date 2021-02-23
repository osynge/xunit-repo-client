#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub configfile: Option<String>,
    pub loglevel: Option<i8>,
    pub xunit_local_globs: Option<Vec<String>>,
    pub environment_sk: Option<String>,
    pub environment_keys: Option<Vec<String>>,
    pub project_sk: Option<String>,
    pub project_identifier: Option<String>,
    pub project_human_name: Option<String>,
    pub run_identifier: Option<String>,
    pub run_sk: Option<String>,
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
            environment_sk: None,
            environment_keys: None,
            project_sk: None,
            project_identifier: None,
            project_human_name: None,
            run_identifier: None,
            run_sk: None,
            server_host: None,
            server_port: None,
        }
    }
    pub fn default(&self, src: &Config) -> Config {
        let configfile = match self.configfile.as_ref().or_else(|| src.configfile.as_ref()) {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let loglevel = self.loglevel.or_else(|| src.loglevel);
        let xunit_local_globs = match self
            .xunit_local_globs
            .as_ref()
            .or_else(|| src.xunit_local_globs.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let environment_sk = match self
            .environment_sk
            .as_ref()
            .or_else(|| src.environment_sk.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let environment_keys = match self
            .environment_keys
            .as_ref()
            .or_else(|| src.environment_keys.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let project_sk = match self.project_sk.as_ref().or_else(|| src.project_sk.as_ref()) {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let project_identifier = match self
            .project_identifier
            .as_ref()
            .or_else(|| src.project_identifier.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };

        let project_human_name = match self
            .project_human_name
            .as_ref()
            .or_else(|| src.project_human_name.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let run_identifier = match self
            .run_identifier
            .as_ref()
            .or_else(|| src.run_identifier.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let run_sk = match self.run_sk.as_ref().or_else(|| src.run_sk.as_ref()) {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let server_host = match self
            .server_host
            .as_ref()
            .or_else(|| src.server_host.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let server_port = self.server_port.or_else(|| src.server_port);
        Config {
            configfile,
            loglevel,
            xunit_local_globs,
            environment_sk,
            environment_keys,
            project_sk,
            project_identifier,
            project_human_name,
            run_identifier,
            run_sk,
            server_host,
            server_port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_config_with_data_1() -> Config {
        Config {
            configfile: Some(String::from("configfile")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("xunit_local_globs")]),
            environment_sk: Some(String::from("environment_sk")),
            environment_keys: Some(vec![String::from("environment_keys")]),
            project_sk: Some(String::from("project_sk")),
            project_identifier: Some(String::from("project_identifier")),
            project_human_name: Some(String::from("project_human_name")),
            run_identifier: Some(String::from("run_identifier")),
            run_sk: Some(String::from("run_sk")),
            server_host: Some(String::from("server_host")),
            server_port: Some(8080),
        }
    }
    fn gen_config_with_data_2() -> Config {
        Config {
            configfile: Some(String::from("2")),
            loglevel: Some(1),
            xunit_local_globs: Some(vec![String::from("2")]),
            environment_sk: Some(String::from("2")),
            environment_keys: Some(vec![String::from("2")]),
            project_sk: Some(String::from("2")),
            project_identifier: Some(String::from("2")),
            project_human_name: Some(String::from("2")),
            run_identifier: Some(String::from("2")),
            run_sk: Some(String::from("2")),
            server_host: Some(String::from("2")),
            server_port: Some(2),
        }
    }

    #[test]
    fn gets_default_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = b.default(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_none_with_none() {
        let a = Config::new();
        let b = Config::new();
        let c = b.default(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = a.default(&b);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_some() {
        let a = gen_config_with_data_1();
        let b = gen_config_with_data_2();
        let c = a.default(&b);
        assert_eq!(c, a);
    }
}
