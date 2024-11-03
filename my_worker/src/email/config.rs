use std::sync::OnceLock;

pub fn email_config() -> &'static EmailConfig {
    static INSTANCE: OnceLock<EmailConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        EmailConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING EMAIL CONF cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct EmailConfig {
    pub SMTP_HOST: String,
    pub SMTP_PORT: String,
    pub SMTP_USER: String,
    pub SMTP_PASS: String,
    pub EMAIL_FROM: String,
    pub EMAIL_FROM_NAME: String,
}

impl EmailConfig {
    fn load_from_env() -> crate::error::Result<EmailConfig> {
        Ok(EmailConfig {
            SMTP_HOST: get_env("SMTP_HOST")?,
            SMTP_PORT: get_env("SMTP_PORT")?,
            SMTP_USER: get_env("SMTP_USER")?,
            SMTP_PASS: get_env("SMTP_PASS")?,
            EMAIL_FROM: get_env("EMAIL_FROM")?,
            EMAIL_FROM_NAME: get_env("EMAIL_FROM_NAME")?,
        })
    }
}

fn get_env(name: &'static str) -> crate::error::Result<String> {
    std::env::var(name).map_err(|_| crate::error::Error::MissingEnv(name))
}
