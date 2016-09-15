extern crate rustc_serialize;

use rustc_serialize::json::{ToJson};
use {NotificationBuilder};

#[test]
fn should_set_notification_title() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.title, None);

    let nm = NotificationBuilder::new()
        .title("title")
        .finalize();

    assert_eq!(nm.title, Some("title"));
}
#[test]
fn should_set_notification_body() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.body, None);

    let nm = NotificationBuilder::new()
        .body("body")
        .finalize();

    assert_eq!(nm.body, Some("body"));
}

#[test]
fn should_set_notification_icon() {
    let nm = NotificationBuilder::new()
        .icon("newicon")
        .finalize();

    assert_eq!(nm.icon, Some("newicon"));
}

#[test]
fn should_set_notification_sound() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.sound, None);

    let nm = NotificationBuilder::new()
        .sound("sound.wav")
        .finalize();

    assert_eq!(nm.sound, Some("sound.wav"));
}

#[test]
fn should_set_notification_badge() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.badge, None);

    let nm = NotificationBuilder::new()
        .badge("1")
        .finalize();

    assert_eq!(nm.badge, Some("1"));
}

#[test]
fn should_set_notification_tag() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.tag, None);

    let nm = NotificationBuilder::new()
        .tag("tag")
        .finalize();

    assert_eq!(nm.tag, Some("tag"));
}

#[test]
fn should_set_notification_color() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.color, None);

    let nm = NotificationBuilder::new()
        .color("color")
        .finalize();

    assert_eq!(nm.color, Some("color"));
}

#[test]
fn should_set_notification_click_action() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.click_action, None);

    let nm = NotificationBuilder::new()
        .click_action("action")
        .finalize();

    assert_eq!(nm.click_action, Some("action"));
}

#[test]
fn should_set_notification_body_loc_key() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.body_loc_key, None);

    let nm = NotificationBuilder::new()
        .body_loc_key("key")
        .finalize();

    assert_eq!(nm.body_loc_key, Some("key"));
}

#[test]
fn should_set_notification_body_loc_args() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.body_loc_args, None);

    let nm = NotificationBuilder::new()
        .body_loc_args(vec!["args"])
        .finalize();

    assert_eq!(nm.body_loc_args, Some(vec!["args".to_string()]));
    assert_eq!(nm.to_json().search("body_loc_args").unwrap().as_string(), Some("[\"args\"]"));
}

#[test]
fn should_set_notification_title_loc_key() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.title_loc_key, None);

    let nm = NotificationBuilder::new()
        .title_loc_key("key")
        .finalize();

    assert_eq!(nm.title_loc_key, Some("key"));
}

#[test]
fn should_set_notification_title_loc_args() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.title_loc_args, None);

    let nm = NotificationBuilder::new()
        .title_loc_args(vec!["args"])
        .finalize();

    assert_eq!(nm.title_loc_args, Some(vec!["args".to_string()]));
    assert_eq!(nm.to_json().search("title_loc_args").unwrap().as_string(), Some("[\"args\"]"));
}
