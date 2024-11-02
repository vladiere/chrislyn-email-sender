use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use tracing::info;

use super::{template, EmailTemplateParams};

pub async fn send_email(email_params: EmailTemplateParams) {
    info!("{:<12} - {:?}\n", "HANDLER", "send_email");

    let message = MessageBuilder::new()
        .from(("Ms. C. Dano", "chrislynmarie2000@gmail.com"))
        .to(email_params.email_to.clone())
        .subject(format!("{} Grade", email_params.term.clone()))
        .html_body(template(email_params))
        .text_body("Hello world!");

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials(("kazhu.korvi@gmail.com", "sqkirhhcfvlvmmvz"))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
