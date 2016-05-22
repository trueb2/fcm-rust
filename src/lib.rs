//! fcm
//! ===
//!
//! # Usage:
//!
//! Add this to `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! fcm = "0.1.4"
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
//! use fcm::Message;
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! map.insert("message", "Howdy!");
//! 
//! let result = Message::new("<registration id>")
//!     .data(map)
//!     .send("<FCM API Key>");
//! ```
//!
//! To send a message using FCM Notifications, we first build the notification:
//! 
//! ```rust
//! use fcm::{Message, NotificationBuilder};
//!
//! let notification = NotificationBuilder::new("Hey!")
//!     .body("Do you want to catch up later?")
//!     .finalize();
//! ```
//! And then set it in the message, before sending it:
//! 
//! ```no_run
//! # use fcm::{Message, NotificationBuilder};
//! # let notification = NotificationBuilder::new("Hey!")
//! #     .body("Do you want to catch up later?")
//! #     .finalize();
//! let result = Message::new("<registration id>")
//!     .notification(notification)
//!     .send("<FCM API Key>");
//! ```
//! You can now handle the result accordingly:
//!
//! ```no_run
//! # use fcm::{Message, NotificationBuilder};
//! # let notification = NotificationBuilder::new("Hey!")
//! #     .body("Do you want to catch up later?")
//! #     .finalize();
//! # let result = Message::new("<registration id>")
//! #     .notification(notification)
//! #     .send("<FCM API Key>");
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

pub use message::response::FcmError as Error;

