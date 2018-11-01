#[macro_use] extern crate serde_derive;

use tokio;
use argparse::{ArgumentParser, Store};
use fcm::{MessageBuilder, Client};
use futures::{
    future::lazy,
    Future,
};

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
        ap.refer(&mut device_token).add_option(&["-t", "--device_token"], Store, "Device token");
        ap.refer(&mut api_key).add_option(&["-k", "--api_key"], Store, "API key");
        ap.parse_args_or_exit();
    }

    let client = Client::new().unwrap();

    let data = CustomData {
        message: "howdy",
    };

    let mut builder = MessageBuilder::new(&api_key, &device_token);
    builder.data(&data).unwrap();
    let payload = builder.finalize();
    let sending = client.send(payload);

    tokio::run(lazy(move || {
        sending
            .map(|response| {
                println!("Sent: {:?}", response);
            }).map_err(|error| {
                println!("Error: {:?}", error)
            })
    }));
}
