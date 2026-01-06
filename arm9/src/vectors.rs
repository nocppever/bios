// vectors.rs - VERSION FINALE (syntaxe ARM valide pour -512)

use core::arch::naked_asm;

#[allow(unused)]
extern "C" {
    fn swi_dispatch(swi_num: u32);
}

// --- EXCEPTION VECTORS ---
#[link_section = ".vectors"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn exception_vectors() {
    naked_asm!(
        "b reset_handler",
        "b undefined_handler",
        "b swi_handler",
        "b prefetch_handler",
        "b data_handler",
        "b reserved_handler",
        "b irq_handler",
        "b fiq_handler",
    );
}

#[unsafe(naked)] #[no_mangle] unsafe extern "C" fn undefined_handler() { naked_asm!("b .") }
#[unsafe(naked)] #[no_mangle] unsafe extern "C" fn prefetch_handler() { naked_asm!("b .") }
#[unsafe(naked)] #[no_mangle] unsafe extern "C" fn data_handler() { naked_asm!("b .") }
#[unsafe(naked)] #[no_mangle] unsafe extern "C" fn reserved_handler() { naked_asm!("b .") }
#[unsafe(naked)] #[no_mangle] unsafe extern "C" fn fiq_handler() { naked_asm!("b .") }

// --- RESET HANDLER ---
#[link_section = ".text.reset_handler"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn reset_handler() -> ! {
    naked_asm!(
        // Init stacks
        "bl init_stacks",
        
        // Jump vers le jeu
        "ldr r12, =0x027FFE24",
        "ldr pc, [r12]",
        
        "1: b 1b",
    )
}

// --- INIT STACKS ---
#[link_section = ".text.init_stacks"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn init_stacks() {
    naked_asm!(
        // SVC mode
        "mov r0, #0xd3",
        "msr cpsr_fsxc, r0",
        "ldr sp, =0x00803FC0",
        
        // IRQ mode
        "mov r0, #0xd2",
        "msr cpsr_fsxc, r0",
        "ldr sp, =0x00803FA0",
        
        // System mode (avec IRQ activées)
        "mov r0, #0x5f",
        "msr cpsr_fsxc, r0",
        "ldr sp, =0x00803EC0",
        
        // Clear DTCM interrupt area
        "mrc p15, #0, r4, c9, c1, #0",
        "lsr r4, r4, #12",
        "lsl r4, r4, #12",
        "add r4, r4, #0x1000",
        
        // CORRECTION: Pour -512, on utilise sub depuis 0
        // -512 = 0 - 512 = sub r1, r1, #512
        "mov r0, #0",
        "mov r1, #0",
        "sub r1, r1, #512",          // r1 = 0 - 512 = -512
        "1:",
        "str r0, [r4, r1]",
        "adds r1, r1, #4",
        "blt 1b",
        
        "bx lr",
    )
}

// --- SWI HANDLER ---
#[link_section = ".text.swi_handler"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn swi_handler() {
    naked_asm!(
        "push {{r11, r12, lr}}",
        
        // CRITIQUE: HALFWORD
        "ldrh r12, [lr, #-2]",
        "and r12, r12, #0xff",
        
        // Table lookup
        "adr r11, svc_table",
        "ldr r12, [r11, r12, lsl #2]",
        
        // Switch en System mode
        "mrs r11, spsr",
        "stmdb sp!, {{r11}}",
        "and r11, r11, #0x80",
        "orr r11, r11, #0x1f",
        "msr cpsr_fsxc, r11",
        
        // Appeler le SWI
        "push {{r2, lr}}",
        "blx r12",
        "pop {{r2, lr}}",
        
        // Revenir en SVC mode
        "mov r12, #0xd3",
        "msr cpsr_fsxc, r12",
        "ldm sp!, {{r11}}",
        "msr spsr_fsxc, r11",
        
        "pop {{r11, r12, lr}}",
        "movs pc, lr",
        
        "svc_table:",
        ".word 0",
    )
}

// --- IRQ HANDLER ---
#[link_section = ".text.irq_handler"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn irq_handler() {
    naked_asm!(
        "push {{r0, r1, r2, r3, r12, lr}}",
        
        // Lire DTCM dynamiquement
        "mrc p15, #0, r0, c9, c1, #0",
        "lsr r0, r0, #12",
        "lsl r0, r0, #12",
        "add r0, r0, #0x1000",
        
        // Jump vers user handler
        "adr lr, 1f",
        "ldr pc, [r0, #-4]",
        
        "1:",
        "pop {{r0, r1, r2, r3, r12, lr}}",
        "subs pc, lr, #4",
    )
}

// --- SUB_0800: Check et Clear Interrupt Flags ---
#[link_section = ".text.sub_0800"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn sub_0800() {
    naked_asm!(
        // r1 = requested interrupt flags
        // Returns: r0 = matched flags (0 if no match)
        
        // Disable IME
        "mov r12, #0x04000000",
        "str r12, [r12, #0x208]",
        
        // Lire DTCM_TOP dynamiquement
        "mrc p15, #0, r3, c9, c1, #0",
        "lsr r3, r3, #12",
        "lsl r3, r3, #12",
        "add r3, r3, #0x1000",
        
        // Lire les flags depuis DTCM+0x3FF8
        "ldr r2, [r3, #-8]",
        
        // Check si les flags demandés sont actifs
        "ands r0, r1, r2",
        
        // Si match, clear les flags
        "eorne r2, r2, r0",
        "strne r2, [r3, #-8]",
        
        // Enable IME
        "mov r12, #1",
        "mov r3, #0x04000000",
        "str r12, [r3, #0x208]",
        
        "bx lr",
    )
}