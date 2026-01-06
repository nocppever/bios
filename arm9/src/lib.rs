#![no_std]
#![no_main]

// Module contenant tout l'assembleur du BIOS original
mod bios;
mod swi;
mod crc;
mod math;
mod decompress;
mod memory;
use core::panic::PanicInfo;


// Panic handler minimal (obligatoire en Rust)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}