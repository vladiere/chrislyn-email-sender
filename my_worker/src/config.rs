use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CORE CONF cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    pub WEB_FOLDER: String,
    pub SERVER_HOST: String,
    pub SERVER_PORT: String,
}

impl CoreConfig {
    fn load_from_env() -> super::error::Result<CoreConfig> {
        Ok(CoreConfig {
            WEB_FOLDER: get_env("WEB_FOLDER")?,
            SERVER_HOST: get_env("SERVER_HOST")?,
            SERVER_PORT: get_env("SERVER_PORT")?,
        })
    }
}

fn get_env(name: &'static str) -> super::error::Result<String> {
    std::env::var(name).map_err(|_| super::error::Error::MissingEnv(name))
}
