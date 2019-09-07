# fcm
[![Travis](https://img.shields.io/travis/panicbit/fcm-rust.svg?style=flat-square)][travis]
[![Coveralls](https://img.shields.io/coveralls/panicbit/fcm-rust.svg?style=flat-square)][coveralls]
[![Crates.io Version](https://img.shields.io/crates/v/fcm.svg?style=flat-square)][crates.io]
[![Crates.io Downloads](https://img.shields.io/crates/dv/fcm.svg?style=flat-square)][crates.io]
[![Crates.io License](https://img.shields.io/crates/l/fcm.svg?style=flat-square)][crates.io]

[crates.io]: https://crates.io/crates/fcm
[travis]: https://travis-ci.org/panicbit/fcm-rust
[coveralls]: https://coveralls.io/github/panicbit/fcm-rust

## Alpha status

The current master and alpha versions use `std::future` with async/await syntax,
and requires a nightly compiler. 0.6.0 works with stable and futures 0.1.

## Examples

Check out the examples directory for a simple sender.

To see it used in a real project, take a look to the [XORC
Notifications](https://github.com/xray-tech/xorc-notifications), which is a
full-fledged consumer for sending push notifications.
