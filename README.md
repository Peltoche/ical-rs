# ical-rs

## Overview

This library is capable of both parsing and generating calendars in the iCalendar format defined in [RFC5545](http://tools.ietf.org/html/rfc5545) as well as similar formats like vCard.

## Foreward

There are probably some issues to be taken care of, but the library should work for most cases. 
If you would like to help out or would like to discuss any API changes please create an issue.

In the beginning the goal was to port the [ical.js](https://github.com/mozilla-comm/ical.js) JavaScript library, a lot of code and algorithms were taken from it at first; but in order to but more "Rusty" a complete rewrite was made.

## Installation

To include this library in your project, simply add it to your list of dependencies in `Cargo.toml`:
```toml
[dependencies]
ical = "0.8.*"
```

### Feature Flags

By default all features are enabled, you can selectively include only the features you need in your project by disabling default features and then manually adding only what you need like this:
```toml
[dependencies]
version = "0.8.*"
default-features = false
features = ["vcard", "ical"]