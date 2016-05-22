fcm
===

## Usage

Add this to `Cargo.toml`:

```rust
[dependencies]
fcm = "0.1.4"
```

then add this to your crate root:

```rust
extern crate fcm;
```

## Examples:
 
Here is an example to send out a FCM Message with some custom data:
 
```rust
use fcm::Message;
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("message", "Howdy!");

let result = Message::new("<registration id>")
    .data(map)
    .send("<FCM API Key>");
```

To send a message using FCM Notifications, we first build the notification:

```rust
use fcm::{Message, NotificationBuilder};

let notification = NotificationBuilder::new("Hey!")
    .body("Do you want to catch up later?")
    .finalize();
```

And then set it in the message, before sending it:

```rust
let result = Message::new("<registration id>")
    .notification(notification)
    .send("<FCM API Key>");
```

You can now handle the result accordingly:

```rust
match result {
  Ok(response) => println!("message_id: {:?}", response.message_id),
  Err(error) => println!("Error: {:?}", error),
}
```
