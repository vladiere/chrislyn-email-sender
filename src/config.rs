use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    pub EMAIL_HOST: String,
    pub EMAIL_PORT: String,
    pub EMAIL_USER: String,
    pub EMAIL_PASS: String,
    pub WEB_FOLDER: String,
    pub SERVER_HOST: String,
    pub SERVER_PORT: String,
}

impl CoreConfig {
    fn load_from_env() -> super::error::Result<CoreConfig> {
        Ok(CoreConfig {
            EMAIL_HOST: get_env("WEB_FOLDER")?,
            EMAIL_PORT: get_env("EMAIL_PORT")?,
            EMAIL_USER: get_env("EMAIL_USER")?,
            EMAIL_PASS: get_env("EMAIL_PASS")?,
            WEB_FOLDER: get_env("WEB_FOLDER")?,
            SERVER_HOST: get_env("SERVER_HOST")?,
            SERVER_PORT: get_env("SERVER_PORT")?,
        })
    }
}

fn get_env(name: &'static str) -> super::error::Result<String> {
    std::env::var(name).map_err(|_| super::error::Error::MissingEnv(name))
}
