use std::error::Error;
use crate::apis::config::Config;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(email: String, body: String) -> Result<(), Box<dyn Error>> {
    let config = Config::init();

    // Create the email message
    let email = Message::builder()
        .from(config.smtp_from.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Shopping verification code")
        .body(body.to_string())
        .unwrap();

    // Create the SMTP transport with authentication
    let smtp_transport = SmtpTransport::relay(&config.smtp_host)
        .unwrap()
        .credentials(Credentials::new(config.smtp_user, config.smtp_pass))
        .build();

    // Send the email
    smtp_transport
        .send(&email)
        .map_err(|err| format!("Failed to send email: {}", err))?;

    Ok(()) // Return Ok(()) if everything is successful

}