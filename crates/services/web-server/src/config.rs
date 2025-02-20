use lib_utils::envs::get_env;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: std::sync::OnceLock<WebConfig> = std::sync::OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING WebConf - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
    pub SERVICE_URL: String,
    pub DOMAIN_MAIN: String,
    pub DOMAIN_DEV: String,
}

impl WebConfig {
    fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
        Ok(WebConfig {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            SERVICE_URL: get_env("SERVICE_URL")?,
            DOMAIN_MAIN: get_env("DOMAIN_MAIN")?,
            DOMAIN_DEV: get_env("DOMAIN_DEV")?,
        })
    }
}
