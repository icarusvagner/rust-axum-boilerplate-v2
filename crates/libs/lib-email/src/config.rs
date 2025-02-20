use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING EMAIL CORE CONF cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- This is for hostinger provider
    pub HOSTINGER_SMTP: String,
    pub HOSTINGER_PORT: String,
    pub HOSTINGER_USER: String,
    pub HOSTINGER_PASS: String,
    // -- This is for ionos provider
    pub IONOS_SMTP: String,
    pub IONOS_PORT: String,
    pub IONOS_USER: String,
    pub IONOS_PASS: String,

    // -- This is for dev using gmail smtp
    pub GMAIL_SMTP_HOST: String,
    pub GMAIL_SMTP_PORT: String,
    pub GMAIL_SMTP_USER: String,
    pub GMAIL_SMTP_PASS: String,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        Ok(CoreConfig {
            HOSTINGER_SMTP: get_env("HOSTINGER_SMTP")?,
            HOSTINGER_PORT: get_env("HOSTINGER_PORT")?,
            HOSTINGER_USER: get_env("HOSTINGER_USER")?,
            HOSTINGER_PASS: get_env("HOSTINGER_PASS")?,
            IONOS_SMTP: get_env("IONOS_SMTP")?,
            IONOS_PORT: get_env("IONOS_PORT")?,
            IONOS_USER: get_env("IONOS_USER")?,
            IONOS_PASS: get_env("IONOS_PASS")?,
            GMAIL_SMTP_HOST: get_env("GMAIL_SMTP_HOST")?,
            GMAIL_SMTP_PORT: get_env("GMAIL_SMTP_PORT")?,
            GMAIL_SMTP_USER: get_env("GMAIL_SMTP_USER")?,
            GMAIL_SMTP_PASS: get_env("GMAIL_SMTP_PASS")?,
        })
    }
}
