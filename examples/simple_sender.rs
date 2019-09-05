extern crate argparse;
extern crate fcm;
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate serde_derive;

use argparse::{ArgumentParser, Store};
use fcm::{Client, MessageBuilder};
use tokio::runtime::current_thread;

#[derive(Serialize)]
struct CustomData {
    message: &'static str,
}

fn main() {
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

    let client = Client::new().unwrap();

    let data = CustomData { message: "howdy" };

    let mut builder = MessageBuilder::new(&api_key, &device_token);
    builder.data(&data).unwrap();
    let payload = builder.finalize();

    match current_thread::block_on_all(client.send(payload)) {
        Ok(response) => println!("Sent: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
