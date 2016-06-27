use {MessageBuilder, Priority};
use notification::NotificationBuilder;
use std::collections::HashMap;

#[test]
fn should_create_new_message() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.to, "token");
}

#[test]
fn should_set_registration_ids() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.registration_ids, None);

    let msg = MessageBuilder::new("token")
        .registration_ids(vec!["id1"]).finalize();

    assert_eq!(msg.registration_ids, Some(vec!["id1".to_string()]));
}

#[test]
fn should_set_collapse_key() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.collapse_key, None);

    let msg = MessageBuilder::new("token")
        .collapse_key("key").finalize();

    assert_eq!(msg.collapse_key, Some("key"));
}

#[test]
fn should_set_priority() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.priority, None);

    let msg = MessageBuilder::new("token")
            .priority(Priority::Normal).finalize();

    assert_eq!(msg.priority, Some(Priority::Normal));
}

#[test]
fn should_set_content_available() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.content_available, None);

    let msg = MessageBuilder::new("token")
        .content_available(true).finalize();

    assert_eq!(msg.content_available, Some(true));
}

#[test]
fn should_set_delay_while_idle() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.delay_while_idle, None);

    let msg = MessageBuilder::new("token")
        .delay_while_idle(true).finalize();

    assert_eq!(msg.delay_while_idle, Some(true));
}

#[test]
fn should_set_time_to_live() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.time_to_live, None);

    let msg = MessageBuilder::new("token")
        .time_to_live(10).finalize();

    assert_eq!(msg.time_to_live, Some(10));
}

#[test]
fn should_set_restricted_package_name() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.restricted_package_name, None);

    let msg = MessageBuilder::new("token")
        .restricted_package_name("name").finalize();

    assert_eq!(msg.restricted_package_name, Some("name"));
}

#[test]
fn should_set_dry_run() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.dry_run, None);

    let msg = MessageBuilder::new("token")
        .dry_run(true).finalize();

    assert_eq!(msg.dry_run, Some(true));
}

#[test]
fn should_set_data() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.data, None);

    let mut data = HashMap::new();
    data.insert("my", "data");

    let msg = MessageBuilder::new("token")
        .data(data).finalize();

    assert_eq!(msg.data.unwrap().get("my"), Some(&"data".to_string()));
}

#[test]
fn should_set_notifications() {
    let msg = MessageBuilder::new("token").finalize();

    assert_eq!(msg.notification, None);

    let nm = NotificationBuilder::new("title").finalize();

    let msg = MessageBuilder::new("token")
        .notification(nm).finalize();

    assert!(msg.notification != None);
}
