use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::{Message, Transport};

use crate::error::ServerError;

pub struct EmailService {
    smtp_host: String,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
}

impl EmailService {
    pub fn new(
        smtp_host: String,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
    ) -> Self {
        Self {
            smtp_host,
            smtp_username,
            smtp_password,
            from_email,
        }
    }

    pub async fn send_verification_code(
        &self,
        to_email: &str,
        code: &str,
    ) -> Result<(), ServerError> {
        let creds = Credentials::new(
            self.smtp_username.clone().into(),
            self.smtp_password.clone().into(),
        );

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(creds)
            .build();

        let text_body = format!(
            "Your verification code is: {}\n\nThis code will expire in 5 minutes.",
            code
        );

        let message = Message::builder()
            .from(self.from_email.parse().map_err(|_| ServerError::InternalServerError)?)
            .to(to_email.parse().map_err(|_| ServerError::BadRequest("Invalid email address".to_owned()))?)
            .subject("Email Verification Code")
            .body(text_body)
            .map_err(|_| {
                eprintln!("Failed to build email message");
                ServerError::InternalServerError
            })?;

        mailer
            .send(&message)
            .map_err(|e| {
                eprintln!("Failed to send email message {}", e);
                ServerError::InternalServerError
            })?;

        Ok(())
    }
}
