use core::arch::naked_asm;

// On déclare les fonctions assembleur existantes pour pouvoir les mettre dans notre table
extern "C" {
    fn SVC_SoftReset();
    fn SVC_WaitByLoop();
    fn SVC_IntrWait();
    fn SVC_VBlankIntrWait();
    fn SVC_Halt();
    fn SVC_CpuSet();
    fn SVC_CpuFastSet();
    fn SVC_BitUnPack();
    fn SVC_LZ77UnCompWRAM();
    fn SVC_LZ77UnCompVRAM();
    fn SVC_HuffUnComp();
    fn SVC_RLUnCompWRAM();
    fn SVC_RLUnCompVRAM();
    fn SVC_Diff8UnFilter();
    fn SVC_Diff16UnFilter();
    fn SVC_CustomPost();
    
    // Si vous avez porté CRC et Math en Rust, utilisez leurs noms Rust, 
    // sinon déclarez les noms ASM (SVC_Div, SVC_Sqrt, SVC_GetCRC16)
    fn SVC_Div();
    fn SVC_Sqrt();
    fn SVC_GetCRC16();
    fn SVC_IsDebugger();
}

/// Nouveau Handler SVC écrit en Rust (Naked ASM wrapper)
/// Remplace __svc_handler de bios_asm.rs
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn __svc_handler() {
    naked_asm!(
        "push {{r11, r12, lr}}",
        
        // 1. Lire le numéro du SWI (l'instruction avant le retour)
        "ldrh r12, [lr, #-2]",
        "and r12, r12, #0xff",
        
        // 2. Charger l'adresse de la table (définie plus bas via LABEL local)
        "adr r11, 2f", 
        "ldr r12, [r11, r12, lsl #2]",
        
        // 3. Sauvegarder SPSR et passer en mode System
        "mrs r11, spsr",
        "stmdb sp!, {{r11}}",
        
        // Note: Le BIOS original force les bits de mode manuellement
        "and r11, r11, #0x80",
        "orr r11, r11, #0x1f",
        "msr cpsr_fsxc, r11",
        
        // 4. Appeler la fonction cible
        "push {{r2, lr}}",
        "blx r12",
        "pop {{r2, lr}}",
        
        // 5. Restaurer le mode Supervisor et SPSR
        "mov r12, #0xd3",
        "msr cpsr_fsxc, r12",
        "ldm sp!, {{r11}}",
        "msr spsr_fsxc, r11",
        
        "pop {{r11, r12, lr}}",
        "movs pc, lr",

        // --- TABLE DE SAUT RECONSTRUITE ---
        ".align 2",
        "2:", // Label local pour la table
        ".word {soft_reset}",       // 0x00
        ".word 0",                  // 0x01
        ".word 0",                  // 0x02
        ".word {wait_loop} + 1",    // 0x03 (+1 pour Thumb)
        ".word {intr_wait}",        // 0x04
        ".word {vblank_wait}",      // 0x05
        ".word {halt}",             // 0x06
        ".word 0",                  // 0x07
        ".word 0",                  // 0x08
        ".word {div}",              // 0x09
        ".word 0",                  // 0x0A
        ".word {cpu_set} + 1",      // 0x0B
        ".word {cpu_fast_set}",     // 0x0C
        ".word {sqrt}",             // 0x0D
        ".word {crc16} + 1",        // 0x0E
        ".word {is_debugger} + 1",  // 0x0F
        ".word {unpack}",           // 0x10
        ".word {lz77_wram}",        // 0x11
        ".word {lz77_vram} + 1",    // 0x12
        ".word {huff} + 1",         // 0x13
        ".word {rl_wram} + 1",      // 0x14
        ".word {rl_vram} + 1",      // 0x15
        ".word {diff8} + 1",        // 0x16
        ".word 0",                  // 0x17
        ".word {diff16} + 1",       // 0x18
        ".word 0",                  // 0x19
        ".word 0",                  // 0x1A
        ".word 0",                  // 0x1B
        ".word 0",                  // 0x1C
        ".word 0",                  // 0x1D
        ".word 0",                  // 0x1E
        ".word {custom} + 1",       // 0x1F

        // Mapping des symboles externes
        soft_reset = sym SVC_SoftReset,
        wait_loop = sym SVC_WaitByLoop,
        intr_wait = sym SVC_IntrWait,
        vblank_wait = sym SVC_VBlankIntrWait,
        halt = sym SVC_Halt,
        div = sym SVC_Div,
        cpu_set = sym SVC_CpuSet,
        cpu_fast_set = sym SVC_CpuFastSet,
        sqrt = sym SVC_Sqrt,
        crc16 = sym SVC_GetCRC16,
        is_debugger = sym SVC_IsDebugger,
        unpack = sym SVC_BitUnPack,
        lz77_wram = sym SVC_LZ77UnCompWRAM,
        lz77_vram = sym SVC_LZ77UnCompVRAM,
        huff = sym SVC_HuffUnComp,
        rl_wram = sym SVC_RLUnCompWRAM,
        rl_vram = sym SVC_RLUnCompVRAM,
        diff8 = sym SVC_Diff8UnFilter,
        diff16 = sym SVC_Diff16UnFilter,
        custom = sym SVC_CustomPost,
    )
}