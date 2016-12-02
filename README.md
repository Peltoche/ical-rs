
# ical-rs 0.2.0
===============

[![Build Status](https://travis-ci.org/Peltoche/rust-vcard-ical.svg?branch=master)](https://travis-ci.org/Peltoche/rust-vcard-ical)


This is a library to parse the ICalendar format defined in [RFC5545](http://tools.ietf.org/html/rfc5545), as well as
similar formats like VCard.

There are probably some issues to be taken care of, but the library should work for most cases. If you like to help out and
would like to discuss any API changes, please [contact me](dev@halium.fr) or create an issue.

The initial goal was to make a port from the [ical.js](https://github.com/mozilla-comm/ical.js) library in JavaScript and
many code/algorithms was taken from it but in order to but more 'Rusty' a complete rewrite have been made.


## Installing

Put this in your `Cargo.toml`:

```toml
[dependencies]
ical = "0.2"
```


## Overview

There is several ways to use Ical depending on the level of parsing you want.

