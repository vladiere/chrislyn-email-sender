use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use tracing::{error, info};

use crate::email::email_config;

use super::error::{Error, Result};

use super::{template, EmailTemplateParams};

pub async fn send_email(email_params: EmailTemplateParams) -> Result<()> {
    info!("{:<20} - {:?}", "ROUTE WORKER", "send_email");
    let smtp_user = &email_config().SMTP_USER;
    let smtp_pass = &email_config().SMTP_PASS;
    let email_from = &email_config().EMAIL_FROM;
    let email_from_name = &email_config().EMAIL_FROM_NAME;
    let smtp_host = &email_config().SMTP_HOST;
    let smtp_port = &email_config().SMTP_PORT;

    let message = MessageBuilder::new()
        .from((email_from_name.as_str(), email_from.as_str()))
        .to(email_params.email_to.clone())
        .subject(format!("{} Grade", email_params.exam_phase.clone()))
        .html_body(template(email_params))
        .text_body("Hello world!");

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    let client_res = SmtpClientBuilder::new(smtp_host.as_str(), smtp_port.parse().unwrap())
        .implicit_tls(false)
        .credentials((smtp_user.as_str(), smtp_pass.as_str()))
        .connect()
        .await;

    match client_res {
       Ok(mut client) => {
            if let Err(e) = client.send(message).await {
                error!("{:<20} - {:?}\n", "ROUTE WORKER ERROR", e);
                return Err(Error::SendingEmailFailed(e.to_string()));
            }
        },
        Err(e) => {
            error!("{:<20} - {:?}\n", "ROUTE WORKER ERROR", e);
            return Err(Error::SMTPServerConnectFailed(e.to_string()));
        }
    }

    Ok(())
}
