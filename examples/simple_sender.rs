use argparse::{ArgumentParser, Store};
use fcm::{Client, MessageBuilder, NotificationBuilder};
use serde::Serialize;

#[derive(Serialize)]
struct CustomData {
    message: &'static str,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let mut device_token = String::new();
    let mut api_key = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A simple FCM notification sender");
        ap.refer(&mut device_token)
            .add_option(&["-t", "--device_token"], Store, "Device token");
        ap.refer(&mut api_key)
            .add_option(&["-k", "--api_key"], Store, "API key");
        ap.parse_args_or_exit();
    }

    let client = Client::new();

    let mut notif_builder = NotificationBuilder::new();
    notif_builder.title("Hey!");
    notif_builder.body("Hello from rust");
    let mut msg_builder = MessageBuilder::new(&api_key, &device_token);
    msg_builder.notification(notif_builder.finalize());

    let response = client.send(msg_builder.finalize()).await?;
    println!("Sent: {:?}", response);

    Ok(())
}
