use std::str::FromStr;

use lettre::message::{Mailbox, SinglePart};
use lettre::transport::smtp::extension::ClientId;
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use thiserror::Error;

use crate::file::EmailConfig;

pub mod template;

#[derive(Debug, Error)]
pub enum SendError {
    #[error("Failed to parse email address")]
    EmailParse,
    #[error("Failed to parse email address: {0}")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("General error: {0}")]
    General(#[from] lettre::error::Error),
    #[error("Failed to deliver email due to SMTM failure: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),
}

pub async fn send_report(
    mail_config: &EmailConfig,
    to: Vec<String>,
    body: String,
) -> Result<(), SendError> {
    let mb_to = to
        .into_iter()
        .map(|addr| Mailbox::from_str(&addr))
        .collect::<Result<Vec<_>, _>>()?;
    let mb_from = Mailbox::new(
        Some(mail_config.from_name.clone()),
        Address::from_str(&mail_config.from_email)?,
    );

    let mut msg = Message::builder().from(mb_from);

    for mb in mb_to {
        msg = msg.bcc(mb);
    }

    let msg = msg.singlepart(SinglePart::html(body))?;

    let transport: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&mail_config.smtp_relay)?
            .hello_name(ClientId::Domain(
                get_ehlo_domain(&mail_config.from_email).ok_or(SendError::EmailParse)?,
            ))
            .build();

    transport.send(msg).await?;

    Ok(())
}

fn get_ehlo_domain(email: &str) -> Option<String> {
    email.split("@").nth(1).map(|domain| domain.to_string())
}
