use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use tracing::{error, info};

use crate::{config::core_config, template, EmailSend};

use super::error::{Error, Result};

pub async fn send_email(data: EmailSend) -> Result<()> {
    info!("{:<20} - {:?}", "ROUTE WORKER", "send_email");

    let EmailSend {
        email_add,
        email_subj,
        email_msg,
    } = data;

    let (smtp_user, smtp_pass, smtp_host, smtp_port, email_to) = (
        &core_config().GMAIL_SMTP_USER,
        &core_config().GMAIL_SMTP_PASS,
        &core_config().GMAIL_SMTP_HOST,
        &core_config().GMAIL_SMTP_PORT,
        &core_config().HOSTINGER_USER,
    );

    let template = template::template_01(template::EmailTemplate01 {
        email_add,
        email_subj: email_subj.clone(),
        email_msg,
    });

    let message = MessageBuilder::new()
        .from((smtp_user.as_str(), "Cebu Tours & Adventure")) // For actual sending email
        .to(email_to.as_str())
        .subject(email_subj)
        .html_body(template);

    let client_res = SmtpClientBuilder::new(
        smtp_host.as_str(),
        smtp_port
            .parse()
            .map_err(|_| Error::EmailPortParsingError)?,
    )
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
            info!("{:<20} - {:?}", "ROUTE WORKER", "Email sent successfully");
            Ok(())
        }

        Err(e) => {
            error!("{:<20} - {:?}\n", "ROUTE WORKER ERROR", e);
            Err(Error::SMTPServerConnectFailed(e.to_string()))
        }
    }
}
