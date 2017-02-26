# Human_size

[![Build Status](https://travis-ci.org/Thomasdezeeuw/human-size-rs.svg?branch=master)](https://travis-ci.org/Thomasdezeeuw/human-size-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/anm4pm65j760m4sc?svg=true)](https://ci.appveyor.com/project/Thomasdezeeuw/human-size-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/crates/v/human_size.svg)](https://crates.io/crates/human-size)
[![Docs](https://docs.rs/human-size/badge.svg)](https://docs.rs/human-size)

The `human_size` crate has sizes for humans. See the example below to see how
to use it.

**Note: currently `human_size` requires a nightly compiler to be build, due to
the need of `u128` and `try_from`, which are both features only available on
nightly**.

```rust
#![feature(try_from)]
use human_size::{Size, Multiple};
use std::convert::TryInto;
let my_size: Size = "1000 B".parse().unwrap();
let same_size = Size::new(1, Multiple::Kilobyte);
assert_eq!(my_size, same_size);

println!("The size is {}.", my_size); // The size is 1000 B.
println!("Or {}.", same_size); // Or 1 kB.
let in_int: u64 = my_size.try_into().unwrap();
println!("Or even {}.", in_int); // Or even 1000.
```
