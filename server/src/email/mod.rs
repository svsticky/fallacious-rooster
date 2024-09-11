use crate::file::EmailConfig;
use lettre::message::{Mailbox, SinglePart};
use lettre::transport::smtp::client::{AsyncSmtpConnection, TlsParameters};
use lettre::transport::smtp::extension::ClientId;
use lettre::Address;
use lettre::Message;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, trace};

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
    #[error("Could not connect to the server")]
    Connect,
}

pub async fn send_report(
    mail_config: &EmailConfig,
    local_addr4: Ipv4Addr,
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

    let client_id =
        ClientId::Domain(get_ehlo_domain(&mail_config.from_email).ok_or(SendError::EmailParse)?);

    trace!("Opening SMTP connection");
    let mut conn = AsyncSmtpConnection::connect_tokio1(
        (mail_config.smtp_relay.as_str(), 587),
        Some(Duration::from_secs(3)),
        &client_id,
        // We cannot do STARTTLS (which uses port 465, which is blocked by Hetzner), so use port 587
        // Port 587 starts out with regular SMTP commands, after the EHLO we upgrade to STARTTLS
        None,
        Some(IpAddr::V4(local_addr4)),
    )
    .await?;

    if conn.can_starttls() {
        conn.starttls(
            TlsParameters::new_rustls(mail_config.smtp_relay.as_str().into())?,
            &client_id,
        )
        .await?;
    }

    trace!("Checking SMTP connection");
    if conn.test_connected().await {
        debug!("SMTP connection OK");
    } else {
        error!("Could not connect to server (SMTP)");
        return Err(SendError::Connect);
    }

    trace!("Sending email");
    conn.send(msg.envelope(), &msg.formatted()).await?;

    Ok(())
}

fn get_ehlo_domain(email: &str) -> Option<String> {
    email.split('@').nth(1).map(|domain| domain.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ehlo_domain() {
        assert_eq!(
            get_ehlo_domain("foo@example.com"),
            Some("example.com".to_owned())
        );
        assert_eq!(get_ehlo_domain("example.org"), None);
        assert_eq!(get_ehlo_domain("example@"), Some(String::new()))
    }
}
