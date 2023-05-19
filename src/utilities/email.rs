
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport}; 

pub fn send_signup_confirmation(receiver: String, confirmation_link: String, credential_username: String, credentials_password: String) {
  let email = Message::builder() 
    .from(("Ndex <noreply@ndex.gg>").parse().unwrap()) 
    .to(receiver.parse().unwrap()) 
    .header(ContentType::TEXT_HTML)
    .subject("Email confirmation - Ndex") 
    .body(format!("
            <div>
                <p> Thank you for joining Ndex! Let's verify your email so we can finish creating your account. Your confirmation link is: </p>
                <p> {link} </p>

                <div> This email was sent by Ndex</div>
            </div>
    ", link =confirmation_link)) 
    .unwrap(); 

  // Create confirmation
  let creds = Credentials::new(credential_username, credentials_password); 
  
  // Open a remote connection to gmail 
  let mailer = SmtpTransport::relay("smtp.gmail.com") 
    .unwrap() 
    .credentials(creds) 
    .build(); 
  
  // Send the email 
  match mailer.send(&email) { 
    Ok(_) => println!("Email sent successfully!"), 
    Err(e) => panic!("Could not send email: {:?}", e), 
  }
}