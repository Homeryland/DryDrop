use std::fs;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::Mailbox,
    transport::smtp::authentication::{Credentials, Mechanism},
};

pub async fn send_email(
    cf_api_key: &str,
    from: &str,
    to: Vec<&str>,
    subject: &str,
    template_path: &str,
    place_holders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::new("api_token".to_string(), cf_api_key.to_string());
    let transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.mx.cloudflare.net")?
        .port(465)
        .credentials(creds)
        .authentication(vec![Mechanism::Plain, Mechanism::Login])
        .build();
    let mut html_template = fs::read_to_string(template_path)?;
    for (k, v) in place_holders {
        html_template = html_template.replace(k, v)
    }
    let mut builder = Message::builder()
        .from(from.parse::<Mailbox>()?)
        .subject(subject);
    for addr in &to {
        builder = builder.to(addr.parse::<Mailbox>()?);
    }
    let email = builder
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(html_template)?;
    transport.send(email).await?;
    Ok(())
}
