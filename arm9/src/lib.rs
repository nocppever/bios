#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
// CORRECTION: Retirer naked_functions (devenu stable)
// #![feature(naked_functions)]

//! # OpenNitro ARM9 BIOS
//! 
//! Open-source implementation of the Nintendo DS ARM9 BIOS.

use core::arch::asm;
use core::panic::PanicInfo;

// Modules
mod vectors;
mod swi;
mod math;
mod memory;
mod decompress;
mod crc;

// Entry point pour le linker
#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    // Le BIOS démarre aux vecteurs d'exception
    loop {
        unsafe {
            // CORRECTION: wfi nécessite ARMv6k, on utilise un halt CP15 à la place
            asm!("mcr p15, 0, r0, c7, c0, 4", options(nostack, nomem));
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            // CP15 halt (ARMv5te compatible)
            asm!("mcr p15, 0, r0, c7, c0, 4", options(nostack, nomem));
        }
    }
}