// swi.rs - FINAL avec allow(dead_code)

use core::arch::{asm, naked_asm};

// --- DÉCLARATION EXTERNE DE sub_0800 (défini dans vectors.rs) ---
#[allow(dead_code)]  // CORRECTION: Éviter le warning
extern "C" {
    fn sub_0800() -> u32;
}

// --- DÉCLARATION DES AUTRES FONCTIONS EXTERNES ---
#[allow(unused)]
extern "C" {
    // Dans decompress.rs
    fn swi_lz77_wram(source: *const u8, dest: *mut u8);
    
    // Dans crc.rs
    fn swi_crc16_impl(crc: u16, data: *const u8, len: u32) -> u16;
    
    // Dans math.rs (Fonctions Naked)
    fn div_impl();
    fn sqrt_impl();
    
    // Dans memory.rs (Fonctions Naked)
    fn cpu_set_impl();
    fn cpu_fast_set_impl();
}

// --- DISPATCHER PRINCIPAL ---
#[no_mangle]
pub unsafe extern "C" fn swi_dispatch(swi_num: u32) {
    match swi_num {
        0x00 => swi_soft_reset(),
        0x01 => swi_register_ram_reset(),
        0x03 => swi_wait_by_loop(),
        0x04 => swi_intr_wait(),
        0x05 => swi_vblank_intr_wait(),
        0x06 => swi_halt(),
        0x07 => swi_stop(),
        
        // MATHS
        0x09 => swi_div(),
        0x0A => swi_div_arm(),
        
        // MÉMOIRE
        0x0B => swi_cpu_set(),
        0x0C => swi_cpu_fast_set(),
        
        // MATHS SUITE
        0x0D => swi_sqrt(),
        
        // UTILITAIRES
        0x0E => swi_crc16(),
        0x0F => swi_is_debugger(),
        
        // DÉCOMPRESSION
        0x11 => swi_lz77(),
        
        _ => {}
    }
}

// --- WRAPPERS ---

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_soft_reset() {
    naked_asm!(
        "ldr r0, =0x027FFE34",
        "ldr pc, [r0]",
    )
}

#[no_mangle]
unsafe extern "C" fn swi_register_ram_reset() {}

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_halt() {
    naked_asm!(
        "mcr p15, 0, r0, c7, c0, 4",
        "bx lr",
    )
}

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_stop() {
    naked_asm!(
        "mcr p15, 0, r0, c7, c0, 4",
        "bx lr",
    )
}

#[no_mangle]
unsafe extern "C" fn swi_is_debugger() {
    asm!("mov r0, #0", out("r0") _);
}

// --- MATH WRAPPERS ---

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_div() {
    naked_asm!("b div_impl")
}

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_div_arm() {
    naked_asm!("b div_impl")
}

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_sqrt() {
    naked_asm!("b sqrt_impl")
}

// --- MEMORY WRAPPERS ---

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_cpu_set() {
    naked_asm!("b cpu_set_impl")
}

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_cpu_fast_set() {
    naked_asm!("b cpu_fast_set_impl")
}

// --- UTILS WRAPPERS ---

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_lz77() {
    naked_asm!(
        "stmfd sp!, {{r4, lr}}",
        "bl swi_lz77_wram",
        "ldmfd sp!, {{r4, pc}}",
    )
}

#[no_mangle]
#[unsafe(naked)]
unsafe extern "C" fn swi_crc16() {
    naked_asm!(
        "stmfd sp!, {{r4, lr}}",
        "bl swi_crc16_impl",
        "ldmfd sp!, {{r4, pc}}",
    )
}

// --- WAIT / INT WRAPPERS ---

#[no_mangle]
unsafe extern "C" fn swi_wait_by_loop() {}

// --- SVC_VBLANKINTRWAIT ---
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn swi_vblank_intr_wait() {
    naked_asm!(
        "mov r0, #1",
        "mov r1, #1",
        // Fall through vers IntrWait
    )
}

// --- SVC_INTRWAIT (BASÉ SUR BIOS OFFICIEL) ---
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn swi_intr_wait() {
    naked_asm!(
        // r0 = discardOldFlags
        // r1 = waitFlags
        
        "push {{r4, lr}}",
        
        // Si r0 != 0, clear old flags
        "cmp r0, #0",
        "blne sub_0800",
        
        // Boucle d'attente
        "1:",
        
        // HALT
        "mov lr, #0",
        "mcr p15, #0, lr, c7, c0, 4",
        
        // Check flags
        "bl sub_0800",
        "cmp r0, #0",
        "beq 1b",
        
        "pop {{r4, lr}}",
        "bx lr",
    )
}