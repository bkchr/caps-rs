# caps

[![Build Status](https://travis-ci.org/lucab/caps-rs.svg?branch=master)](https://travis-ci.org/lucab/caps-rs)
[![crates.io](https://img.shields.io/crates/v/caps.svg)](https://crates.io/crates/caps)
[![LoC](https://tokei.rs/b1/github/lucab/caps-rs?category=code)](https://github.com/lucab/caps-rs)
[![Documentation](https://docs.rs/caps/badge.svg)](https://docs.rs/caps)

A pure-Rust library to work with Linux capabilities.

`caps` provides support for manipulating capabilities available in modern Linux
kernels. It supports traditional POSIX sets (Effective, Inheritable, Permitted)
as well as Linux-specific Ambient and Bounding capabilities sets.

`caps` provides a simple and idiomatic interface to handle capabilities on Linux.
See `capabilities(7)` for more details.

## Motivations

This library tries to achieve the following goals:
 * fully support modern kernels, including recent capabilities and sets
 * provide an idiomatic interface
 * be usable in static targets, without requiring an external C library

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
caps = "0.0"
```

and this to your crate root:

```rust
extern crate caps;
```

## Example

```rust
extern crate caps;
use caps::{Capability, CapSet};

fn manipulate_caps() {
    // Retrieve permitted set.
    let cur = caps::read(None, CapSet::Permitted).unwrap();
    println!("Current permitted caps: {:?}.", cur);
    
    // Retrieve effective set.
    let cur = caps::read(None, CapSet::Effective).unwrap();
    println!("Current effective caps: {:?}.", cur);
    
    // Check if CAP_CHOWN is in permitted set.
    let perm_chown = caps::has_cap(None, CapSet::Permitted, Capability::CAP_CHOWN).unwrap();
    if !perm_chown.unwrap() {
        println!("Try running this as root!");
        return;
    }

    // Clear all effective caps.
    caps::clear(None, CapSet::Effective).unwrap();
    println!("Cleared effective caps.");
    let cur = caps::read(None, CapSet::Effective).unwrap();
    println!("Current effective caps: {:?}.", cur);

    // Since `CAP_CHOWN` is still in permitted, it can be raised again.
    caps::raise(None, CapSet::Effective, Capability::CAP_CHOWN).unwrap();
    println!("Raised CAP_CHOWN in effective set.");
    let cur = caps::read(None, CapSet::Effective).unwrap();
    println!("Current effective caps: {:?}.", cur);
}
```

Some more examples are available under [examples](examples).

## License

`caps` is available under the terms of both the MIT license and the
Apache License (Version 2.0).
