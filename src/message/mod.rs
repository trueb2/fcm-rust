#[cfg(test)]
mod tests;

use notification::Notification;
use erased_serde::Serialize;
use serde_json::{self, Value};

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Normal, High
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content_available: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    delay_while_idle: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Notification>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    registration_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_live: Option<i32>,

    to: String,
}

/// Represents a FCM message. Construct the FCM message
/// using various utility methods and finally send it.
/// # Examples:
/// ```rust
/// use fcm::MessageBuilder;
///
/// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
/// builder.dry_run(true);
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct Message {
    pub api_key: String,
    pub body: MessageBody,
}

///
/// A builder to get a `Message` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::MessageBuilder;
///
/// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
/// builder.dry_run(true);
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct MessageBuilder {
    api_key: String,
    collapse_key: Option<String>,
    content_available: Option<bool>,
    data: Option<Value>,
    delay_while_idle: Option<bool>,
    dry_run: Option<bool>,
    notification: Option<Notification>,
    priority: Option<Priority>,
    registration_ids: Option<Vec<String>>,
    restricted_package_name: Option<String>,
    time_to_live: Option<i32>,
    to: String,
}

impl MessageBuilder {
    /// Get a new instance of Message. You need to supply either
    /// a registration id, or a topic (/topics/...).
    pub fn new<S: Into<String>>(api_key: S, to: S) -> MessageBuilder {
        MessageBuilder {
            api_key: api_key.into(),
            to: to.into(),
            registration_ids: None,
            collapse_key: None,
            priority: None,
            content_available: None,
            delay_while_idle: None,
            time_to_live: None,
            restricted_package_name: None,
            dry_run: None,
            data: None,
            notification: None,
        }
    }

    /// Set various registration ids to which the message ought to be sent.
    pub fn registration_ids<S: Into<String>>(&mut self, ids: Vec<S>) -> &mut MessageBuilder {
        self.registration_ids = Some(ids.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Set this parameter to identify groups of messages that can be collapsed.
    pub fn collapse_key<S: Into<String>>(&mut self, collapse_key: S) -> &mut MessageBuilder {
        self.collapse_key = Some(collapse_key.into());
        self
    }

    /// Set the priority of the message. You can set Normal or High priorities.
    /// # Examples:
    /// ```rust
    /// use fcm::{MessageBuilder, Priority};
    /// 
    /// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
    /// builder.priority(Priority::High);
    /// let message = builder.finalize();
    /// ```
    pub fn priority(&mut self, priority: Priority) -> &mut MessageBuilder {
        self.priority = Some(priority);
        self
    }

    /// To set the `content-available` field on iOS
    pub fn content_available(&mut self, content_available: bool) -> &mut MessageBuilder {
        self.content_available = Some(content_available);
        self
    }

    /// When set to `true`, sends the message only when the device is active.
    pub fn delay_while_idle(&mut self, delay_while_idle: bool) -> &mut MessageBuilder {
        self.delay_while_idle = Some(delay_while_idle);
        self
    }

    /// How long (in seconds) to keep the message on FCM servers in case the device 
    /// is offline. The maximum and default is 4 weeks.
    pub fn time_to_live(&mut self, time_to_live: i32) -> &mut MessageBuilder {
        self.time_to_live = Some(time_to_live);
        self
    }

    /// Package name of the application where the registration tokens must match.
    pub fn restricted_package_name<S: Into<String>>(&mut self, restricted_package_name: S) -> &mut MessageBuilder {
        self.restricted_package_name = Some(restricted_package_name.into());
        self
    }

    /// When set to `true`, allows you to test FCM without actually sending the message.
    pub fn dry_run(&mut self, dry_run: bool) -> &mut MessageBuilder {
        self.dry_run = Some(dry_run);
        self
    }

    /// Use this to add custom key-value pairs to the message. This data
    /// must be handled appropriately on the client end. The data can be
    /// anything that Serde can serialize to JSON.
    ///
    /// # Examples:
    /// ```rust
    /// use fcm::MessageBuilder;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("message", "Howdy!");
    ///
    /// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
    /// builder.data(&map);
    /// let message = builder.finalize();
    /// ```
    pub fn data(&mut self, data: &Serialize) -> Result<&mut MessageBuilder, serde_json::Error> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    /// Use this to set a `Notification` for the message.
    /// # Examples:
    /// ```rust
    /// use fcm::{MessageBuilder, NotificationBuilder};
    ///
    /// let mut builder = NotificationBuilder::new();
    /// builder.title("Hey!");
    /// builder.body("Do you want to catch up later?");
    /// let notification = builder.finalize();
    /// 
    /// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
    /// builder.notification(notification);
    /// let message = builder.finalize();
    /// ```
    pub fn notification(&mut self, notification: Notification) -> &mut MessageBuilder {
        self.notification = Some(notification);
        self
    }

    /// Complete the build and get a `Message` instance
    pub fn finalize(self) -> Message {
        Message {
            api_key: self.api_key,
            body: MessageBody {
                to: self.to,
                registration_ids: self.registration_ids.clone(),
                collapse_key: self.collapse_key,
                priority: self.priority,
                content_available: self.content_available,
                delay_while_idle: self.delay_while_idle,
                time_to_live: self.time_to_live,
                restricted_package_name: self.restricted_package_name,
                dry_run: self.dry_run,
                data: self.data.clone(),
                notification: self.notification,
            }
        }
    }
}
