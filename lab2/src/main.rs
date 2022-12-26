#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buf;
mod game_of_life;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write;
use crate::game_of_life::game_of_life;
use crate::vga_buf::{Alignment, COLOR_BLUE, COLOR_DARK_GRAY, COLOR_LIGHT_BLUE, COLOR_RED, Screen};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(COLOR_BLUE, COLOR_DARK_GRAY, Alignment::Center);

    for i in 0..12 {
        // write!(screen, "Hello world {0}\n", i * 10);
        write!(screen, "Hello");
    }

    loop {}
}
