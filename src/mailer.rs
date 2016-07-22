extern crate dotenv;
extern crate lettre;

use queries;
use self::dotenv::dotenv;
use self::lettre::email::{Email, EmailBuilder};
use self::lettre::transport::EmailTransport;
use self::lettre::transport::smtp::SmtpTransportBuilder;
use std::env;

fn env_var(var: &str) -> String {
    match env::var(var) {
        Ok(x) => x,
        _ => "".to_string(),
    }
}

fn email_builder(to: &str, from: &str) -> Email {
    EmailBuilder::new()
        .to(to)
        .from(from)
        .subject("Hey guess what?")
        .body("Just a body") // Use a template here.
        .build()
        .unwrap()
}

pub fn mailer() {
    dotenv().ok();
    let email_host: &str = &env_var("EMAIL_HOST");
    let email_port: u16 = env_var("EMAIL_PORT").parse::<u16>().unwrap();
    let email_address: &str = &env_var("EMAIL_ADDRESS");
    let email_password: &str = &env_var("EMAIL_PASSWORD");

    let mut mailer = SmtpTransportBuilder::new((email_host, email_port))
        .unwrap()
        .credentials(email_address, email_password)
        .connection_reuse(true)
        .build();

    for invitee in queries::pending() {
        let invitee_email: &str = &invitee.email;
        let email = email_builder(invitee_email, email_address);
        mailer.send(email.clone()).unwrap();    
        queries::invited(invitee_email.to_string());
    }
    mailer.close();
}
