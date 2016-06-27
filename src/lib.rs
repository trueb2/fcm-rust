#![doc(html_root_url = "https://panicbit.github.io/fcm-rust/fcm/")]
//! fcm
//! ===
//!
//! # Usage:
//!
//! Add this to `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! fcm = "0.1.0"
//! ```
//!
//! then add this to your crate root:
//!
//! ```ignore
//! extern crate fcm;
//! ```
//!
//! # Examples:
//! 
//! Here is an example to send out a FCM Message with some custom data:
//! 
//! ```no_run
//! use fcm::{MessageBuilder, Client};
//! use std::collections::HashMap;
//!
//! let client = Client::new();
//!
//! let mut map = HashMap::new();
//! map.insert("message", "Howdy!");
//! 
//! let message = MessageBuilder::new("<registration id>").data(map);
//! let result = client.send(message.finalize(), "<FCM API Key>");
//! ```
//!
//! To send a message using FCM Notifications, we first build the notification:
//! 
//! ```rust
//! use fcm::{MessageBuilder, NotificationBuilder};
//!
//! let notification = NotificationBuilder::new("Hey!")
//!     .body("Do you want to catch up later?")
//!     .finalize();
//! ```
//! And then set it in the message, before sending it:
//! 
//! ```no_run
//! # use fcm::{MessageBuilder, NotificationBuilder, Client};
//! # let client = Client::new();
//! # let notification = NotificationBuilder::new("Hey!")
//! #     .body("Do you want to catch up later?")
//! #     .finalize();
//! let message = MessageBuilder::new("<registration id>").notification(notification);
//! let result = client.send(message.finalize(), "<FCM API Key>");
//! ```
//! You can now handle the result accordingly:
//!
//! ```no_run
//! # use fcm::{MessageBuilder, NotificationBuilder, Client};
//! # let client = Client::new();
//! # let notification = NotificationBuilder::new("Hey!")
//! #     .body("Do you want to catch up later?")
//! #     .finalize();
//! # let message = MessageBuilder::new("<registration id>")
//! #     .notification(notification).finalize();
//! # let result = client.send(message, "<FCM API Key>");
//! match result {
//!   Ok(response) => println!("message_id: {:?}", response.message_id),
//!   Err(error) => println!("Error: {:?}", error),
//! }
//! ```

extern crate rustc_serialize;
extern crate hyper;

mod message;
pub use message::*;
mod notification;
pub use notification::*;
mod client;
pub use client::*;

pub use client::response::FcmError as Error;

