#[cfg(test)]
mod tests;

/// This struct represents a FCM notification. Use the 
/// corresponding `NotificationBuilder` to get an instance. You can then use 
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    badge: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    click_action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_key: Option<String>,
}

/// A builder to get a `Notification` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::NotificationBuilder;
///
/// let mut builder = NotificationBuilder::new();
//  builder.title("Australia vs New Zealand");
/// builder.body("3 runs to win in 1 ball");
/// let notification = builder.finalize();
/// ```
pub struct NotificationBuilder {
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    sound: Option<String>,
    badge: Option<String>,
    tag: Option<String>,
    color: Option<String>,
    click_action: Option<String>,
    body_loc_key: Option<String>,
    body_loc_args: Option<Vec<String>>,
    title_loc_key: Option<String>,
    title_loc_args: Option<Vec<String>>,
}

impl NotificationBuilder {
    /// Get a new `NotificationBuilder` instance, with a title.
    pub fn new() -> NotificationBuilder {
        NotificationBuilder {
            title: None,
            body: None,
            icon: None,
            sound: None,
            badge: None,
            tag: None,
            color: None,
            click_action: None,
            body_loc_key: None,
            body_loc_args: None,
            title_loc_key: None,
            title_loc_args: None,
        }
    }

    // Set the title of the notification
    pub fn title<S>(&mut self, title: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.title = Some(title.into());
        self
    }

    /// Set the body of the notification
    pub fn body<S>(&mut self, body: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.body = Some(body.into());
        self
    }

    /// Set the notification icon.
    pub fn icon<S>(&mut self, icon: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.icon = Some(icon.into());
        self
    }

    /// Set the sound to be played
    pub fn sound<S>(&mut self, sound: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.sound = Some(sound.into());
        self
    }

    /// Set the badge for iOS notifications
    pub fn badge<S>(&mut self, badge: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.badge = Some(badge.into());
        self
    }

    /// Tagging a notification allows you to replace existing notifications
    /// with the same tag with this new notification
    pub fn tag<S>(&mut self, tag: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.tag = Some(tag.into());
        self
    }

    /// The color of the icon, in #rrggbb format
    pub fn color<S>(&mut self, color: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.color = Some(color.into());
        self
    }

    /// What happens when the user clicks on the notification. Refer to 
    /// https://developers.google.com/cloud-messaging/http-server-ref#table2 for
    /// details.
    pub fn click_action<S>(&mut self, click_action: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.click_action = Some(click_action.into());
        self
    }

    /// Set the body key string for localization
    pub fn body_loc_key<S>(&mut self, body_loc_key: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.body_loc_key = Some(body_loc_key.into());
        self
    }

    /// String value to replace format specifiers in the body string.
    pub fn body_loc_args<S>(&mut self, body_loc_args: Vec<S>) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.body_loc_args = Some(body_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Set the title key string for localization
    pub fn title_loc_key<S>(&mut self, title_loc_key: S) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.title_loc_key = Some(title_loc_key.into());
        self
    }

    /// String value to replace format specifiers in the title string.
    pub fn title_loc_args<S>(&mut self, title_loc_args: Vec<S>) -> &mut NotificationBuilder
        where S: Into<String>
    {
        self.title_loc_args = Some(title_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Complete the build and get a `Notification` instance
    pub fn finalize(self) -> Notification {
        Notification {
            title: self.title,
            body: self.body,
            icon: self.icon,
            sound: self.sound,
            badge: self.badge,
            tag: self.tag,
            color: self.color,
            click_action: self.click_action,
            body_loc_key: self.body_loc_key,
            body_loc_args: self.body_loc_args.clone(),
            title_loc_key: self.title_loc_key,
            title_loc_args: self.title_loc_args.clone(),
        }
    }
}
