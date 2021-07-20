use std::error::Error;
use std::thread;
use std::time::Duration;

use rand::{self, Rng};

use blinkt::Blinkt;
use rppal::spi;

fn main() -> Result<(), Box<dyn Error>> {
    let mut left = Blinkt::with_spi_bus(8_000_000, spi::Bus::Spi0, 432)?;
    let mut right = Blinkt::with_spi_bus(8_000_000, spi::Bus::Spi0, 432)?;
    let mut rng = rand::thread_rng();

    blinkt.set_all_pixels_brightness(0.1);

    loop {
        // Iterate over all pixels, setting red, green and blue to random values.
        for pixel in &mut left {
            pixel.set_rgb(rng.gen(), rng.gen(), rng.gen());
        }

        for pixel in &mut right {
            pixel.set_rgb(rng.gen(), rng.gen(), rng.gen());
        }

        // Send the new color values to the Blinkt! board.
        left.show()?;
        right.show()?;

        thread::sleep(Duration::from_millis(30));
    }
}
