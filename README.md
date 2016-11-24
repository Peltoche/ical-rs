
# ical-rs 0.1.0
===============

[![Build Status](https://travis-ci.org/Peltoche/rust-vcard-ical.svg?branch=master)](https://travis-ci.org/Peltoche/rust-vcard-ical)

This library is under heavy development. Many features are not finished.

Vcard and Ical parser for Rust. It aims to be a feature-complete parser all vcard and ical files.
* Ical-rs strictly adheres to rfc6350.
* Ical-rs handle Vcard version 3 and 4.


The initial goal was to make a porting of the [mozilla parser](https://github.com/mozilla-comm/ical.js) from Javascript
to Rust. The main logic come from this codebase, but is adapted to more Rusty.



## Usage

Put this in your `Cargo.toml`.
```toml
[dependencies]
ical-rs = "0.1.0"
```

Or, if you want [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) support,
include the features like this:
```toml
[dependencies]
ical-rs = { version = "0.1.0", features = ["rustc-serialize"] }
```


Then put this in your crate root:

```rust
extern crate ical;
```

