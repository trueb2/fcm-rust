# fcm
[![Travis](https://img.shields.io/travis/panicbit/fcm-rust.svg?style=flat-square)][travis]
[![Coveralls](https://img.shields.io/coveralls/panicbit/fcm-rust.svg?style=flat-square)][coveralls]
[![Crates.io Version](https://img.shields.io/crates/v/fcm.svg?style=flat-square)][crates.io]
[![Crates.io Downloads](https://img.shields.io/crates/dv/fcm.svg?style=flat-square)][crates.io]
[![Crates.io License](https://img.shields.io/crates/l/fcm.svg?style=flat-square)][crates.io]

[crates.io]: https://crates.io/crates/fcm
[travis]: https://travis-ci.org/panicbit/fcm-rust
[coveralls]: https://coveralls.io/github/panicbit/fcm-rust


## Usage

Add this to `Cargo.toml`:

```rust
[dependencies]
fcm = "0.2"
tokio = "0.1"
futures = "0.1"
```

then add this to your crate root:

```rust
extern crate fcm;
extern crate tokio;
extern crate futures;
```

## Examples

Check out the examples directory for a simple sender.
