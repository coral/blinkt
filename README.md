# Blinkt

[![Build Status](https://travis-ci.org/golemparts/blinkt.svg?branch=master)](https://travis-ci.org/golemparts/blinkt)
[![crates.io](https://meritbadge.herokuapp.com/blinkt)](https://crates.io/crates/blinkt)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-v1.31.0-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

Blinkt is a Rust library that provides an interface for the Pimoroni Blinkt!, and any similar APA102 or SK9822 LED strips or boards, on a Raspberry Pi. The library supports bitbanging mode on any GPIO pins, and hardware SPI mode on GPIO 10 (physical pin 19) for data, and GPIO 11 (physical pin 23) for clock.

For bitbanging mode, Blinkt gains access to the BCM283x GPIO peripheral either through `/dev/gpiomem` or `/dev/mem`. Hardware SPI mode is controlled through `/dev/spidev0.0`.

Both the original APA102 and the SK9822 clone are supported. The RGB LED/driver ICs are referred to as pixels throughout the code and documentation.

## Documentation

Documentation for the latest release can be found at [docs.golemparts.com/blinkt](https://docs.golemparts.com/blinkt). Documentation for earlier releases is stored at [docs.rs/blinkt](https://docs.rs/blinkt).

## Usage

Add a dependency for `blinkt` to your `Cargo.toml`.

```toml
[dependencies]
blinkt = "0.5"
```

Call `Blinkt::new()` to create a new Blinkt with the default settings. Alternative configuration options are available through `Blinkt::with_settings()` and `Blinkt::with_spi()`.

```rust
use blinkt::Blinkt;

let mut blinkt = Blinkt::new()?;
```

## Examples

The example below demonstrates swapping all pixels on a Blinkt! board between red, green and blue.

```rust

use std::error::Error;
use std::{thread, mem};
use std::time::Duration;

use blinkt::Blinkt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut blinkt = Blinkt::new()?;
    let (red, green, blue) = (&mut 255, &mut 0, &mut 0);

    loop {
        blinkt.set_all_pixels(*red, *green, *blue);
        blinkt.show()?;

        thread::sleep(Duration::from_millis(250));

        mem::swap(red, green);
        mem::swap(red, blue);
    }
}
```

To control an LED strip, consisting of 144 pixels, connected to the Raspberry Pi's hardware SPI pins (data on GPIO 10 (physical pin 19), and clock on GPIO 11 (physical pin 23)), at 16MHz clock speed, replace the Blinkt::new() line in the above example with the following. You may have to tweak the maximum clock speed based on the number of pixels and wire quality.

```rust
let mut blinkt = Blinkt::with_spi(16_000_000, 144)?;
```

Additional examples can be found in the `examples` directory.

## Cross compilation

If you're not working directly on a Raspberry Pi, you'll likely need to cross compile your code for the appropriate ARM architecture. Check out [this guide](https://github.com/japaric/rust-cross) for more information, or try the [cross](https://github.com/japaric/cross) project for "zero setup" cross compilation.

## Copyright and license

Copyright (c) 2016-2019 Rene van der Meer. Released under the [MIT license](LICENSE).
