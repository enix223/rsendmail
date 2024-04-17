use clap::Parser;
use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism},
    Message, SmtpTransport, Transport,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'u', long, help = "authentication username")]
    username: String,

    #[arg(short = 'p', long, help = "authentication password")]
    password: String,

    #[arg(short = 'r', long, help = "receiver email address")]
    receiver: String,

    #[arg(short = 's', long, help = "smtp server")]
    server: String,

    #[arg(short = 'P', long, default_value_t = 25, help = "smtp server port")]
    port: u16,

    #[arg(short = 't', long, help = "use tls or not")]
    tls: bool,

    #[arg(short = 'S', long, help = "email subject")]
    subject: String,

    #[arg(short = 'b', long, help = "email body")]
    body: String,
}

fn main() {
    let args = Args::parse();

    let email = Message::builder()
        .from(args.username.parse().expect("invalid sender mailbox"))
        .to(args.receiver.parse().expect("invalid receiver mailbox"))
        .subject(args.subject)
        .body(args.body)
        .expect("failed to create email message");

    let transport = match args.tls {
        false => SmtpTransport::builder_dangerous(&args.server),
        true => SmtpTransport::relay(&args.server).expect("could not create smtp transport"),
    };
    let sender = transport
        .port(args.port)
        .credentials(Credentials::new(args.username, args.password))
        .authentication(vec![Mechanism::Plain])
        .build();

    let result = sender.send(&email);
    match result {
        Ok(_) => println!("send mail success"),
        Err(e) => println!("failed to send mail: {:?}", e),
    };
}
