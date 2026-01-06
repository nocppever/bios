//! Memory Manipulation Functions (SWI 0x0B & 0x0C)

#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn cpu_set_impl() {
    // r0 = source, r1 = dest, r2 = control
    // Control bit 24: 1=Fixed Source (Fill), 0=Copy
    // Control bit 26: 1=32bit, 0=16bit
    // Control 0-20: Count
    core::arch::naked_asm!(
        "stmfd sp!, {{r4-r6, lr}}", // Sauvegarde registres
        
        "mov r3, r2, lsr #24",      // r3 = Flags (bits 24+)
        "bic r2, r2, #0xFF000000",  // r2 = Count (bits 0-20) only
        
        "tst r3, #4",               // Test bit 26 (32-bit mode?)
        "bne 2f",                   // Si 1, aller vers 32-bit (label 2)

        // --- 16-bit Mode ---
        "tst r3, #1",               // Test bit 24 (Fixed source?)
        "bne 1f",                   // Si 1, aller vers Fill (label 1)

        // 16-bit Copy
        "0:",
        "ldrh r4, [r0], #2",        // Load halfword & inc src
        "strh r4, [r1], #2",        // Store halfword & inc dst
        "subs r2, r2, #1",
        "bne 0b",
        "ldmfd sp!, {{r4-r6, pc}}",

        // 16-bit Fill
        "1:",
        "ldrh r4, [r0]",            // Load source une seule fois
        "3:",
        "strh r4, [r1], #2",
        "subs r2, r2, #1",
        "bne 3b",
        "ldmfd sp!, {{r4-r6, pc}}",

        // --- 32-bit Mode ---
        "2:", 
        "tst r3, #1",               // Test bit 24 (Fixed source?)
        "bne 4f",                   // Si 1, aller vers Fill (label 4)

        // 32-bit Copy
        "5:",
        "ldr r4, [r0], #4",
        "str r4, [r1], #4",
        "subs r2, r2, #1",
        "bne 5b",
        "ldmfd sp!, {{r4-r6, pc}}",

        // 32-bit Fill
        "4:",
        "ldr r4, [r0]",             // Load source une seule fois
        "6:",
        "str r4, [r1], #4",
        "subs r2, r2, #1",
        "bne 6b",
        "ldmfd sp!, {{r4-r6, pc}}",
    )
}

#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn cpu_fast_set_impl() {
    // r0 = source, r1 = dest, r2 = control
    // FastSet copie toujours par blocs de 32 octets (8 mots de 32 bits)
    // C'est souvent utilisé pour copier la VRAM.
    core::arch::naked_asm!(
        "stmfd sp!, {{r4-r10, lr}}",
        
        "mov r3, r2, lsr #24",      // Flags
        "bic r2, r2, #0xFF000000",  // Count (nombre de WORDS)
        
        // FastSet attend un compte multiple de 8. Divisons par 8 pour compter les blocs.
        "mov r2, r2, lsr #3",       
        "cmp r2, #0",
        "beq 9f",                   // Si count=0, on sort

        "tst r3, #1",               // Fixed source (Fill) ?
        "bne 2f",

        // --- Copy Mode (8 words par boucle) ---
        "1:",
        "ldmia r0!, {{r3-r10}}",    // Charge 8 mots (32 octets) depuis src
        "stmia r1!, {{r3-r10}}",    // Écrit 8 mots vers dest
        "subs r2, r2, #1",
        "bne 1b",
        "ldmfd sp!, {{r4-r10, pc}}",

        // --- Fill Mode (8 words par boucle) ---
        "2:",
        "ldr r3, [r0]",             // Charge la valeur source UNE FOIS
        "mov r4, r3",               // Duplique dans tous les registres
        "mov r5, r3",
        "mov r6, r3",
        "mov r7, r3",
        "mov r8, r3",
        "mov r9, r3",
        "mov r10, r3",
        
        "3:",
        "stmia r1!, {{r3-r10}}",    // Écrit 8 fois la même valeur
        "subs r2, r2, #1",
        "bne 3b",

        "9:",
        "ldmfd sp!, {{r4-r10, pc}}",
    )
}