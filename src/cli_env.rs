pub fn cli_env() -> crate::config::Config {
    let mut out = crate::config::Config::new();
    for (key, value) in std::env::vars() {
        if "XRC_CONFIG".eq(&key) {
            out.configfile = Some(value.clone());
        }
        if "XRC_XUNIT".eq(&key) {
            out.xunit_local_globs = Some(vec![value.clone()]);
        }
        if "XRC_ENVIROMENT_KEY".eq(&key) {
            out.enviroment_sk = Some(value.clone());
        }
        if "XRC_ENVIROMENT".eq(&key) {
            out.enviroment_keys = Some(vec![value.clone().split(":").collect()]);
        }
        if "XRC_PROJECT_KEY".eq(&key) {
            out.project_sk = Some(value.clone());
        }
        if "XRC_PROJECT_IDENTIFIER".eq(&key) {
            out.project_identiifier = Some(value.clone());
        }
        if "XRC_PROJECT_NAME".eq(&key) {
            out.project_human_name = Some(value.clone());
        }
        if "XRC_RUN_IDENTIFIER".eq(&key) {
            out.run_identifier = Some(value.clone());
        }
        if "XRC_RUN_KEY".eq(&key) {
            out.run_sk = Some(value.clone());
        }
        if "XRC_HOST".eq(&key) {
            out.server_host = Some(value.clone());
        }
        if "XRC_PORT".eq(&key) {
            out.server_port = Some(
                value
                    .parse()
                    .expect("Enviroment variable XRC_PORT is not an integer"),
            );
        }
    }
    out
}
