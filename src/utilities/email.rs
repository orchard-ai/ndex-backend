extern crate lettre;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport};

pub fn send_signup_confirmation(receiver: &str, confirmation_link: &str, credential_username: &str, credentials_password: &str, credentials_server: &str) {
  let email = Message::builder() 
    .from(credential_username.parse().unwrap()) 
    .to(receiver.parse().unwrap()) 
    .header(ContentType::TEXT_HTML)
    .subject("Email confirmation - Ndex") 
    .body(format!("
            <div>
                <p> Thank you for joining Ndex! Let's verify your email so we can finish creating your account. Your confirmation link is: </p>
                <p> {confirmation_link} </p>

                <div> This email was sent by Ndex</div>
            </div>
    ")) 
    .unwrap(); 

  // Create confirmation
  let creds = Credentials::new(credential_username.to_string(), credentials_password.to_string()); 

  // Open a remote connection to gmail 
  let mailer = SmtpTransport::relay(credentials_server)
  .unwrap()
  .credentials(creds)
  .build();
  
  // Send the email 
  match mailer.send(&email) { 
    Ok(_) => println!("Email sent successfully!"), 
    Err(e) => panic!("Could not send email: {e:?}"), 
  }
}