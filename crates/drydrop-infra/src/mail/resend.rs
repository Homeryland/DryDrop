use std::fs;

use resend_rs::{Resend, types::CreateEmailBaseOptions};

pub async fn send_email(
    resend: &Resend,
    from: &str,
    to: Vec<&str>,
    subject: &str,
    template_path: &str,
    place_holders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut html_template = fs::read_to_string(template_path)?;
    for (k, v) in place_holders {
        html_template = html_template.replace(k, v)
    }
    let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&html_template);
    match resend.emails.send(email).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Failed to send email: {:?}", e),
    }
    Ok(())
}
