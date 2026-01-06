// default_irq_handler.rs
// Handler IRQ par défaut si le jeu n'en fournit pas

use core::arch::naked_asm;

/// Default IRQ handler qui met à jour DTCM+0x3FF8
/// 
/// Le jeu est censé fournir son propre handler à DTCM+0x3FFC,
/// mais si il ne le fait pas, celui-ci sert de fallback.
/// 
/// Ce handler:
/// 1. Lit IF (0x04000214) pour voir quelles IRQ sont actives
/// 2. Écrit dans DTCM+0x3FF8 pour IntrWait
/// 3. Acquitte IF
#[link_section = ".text.default_irq"]
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn default_irq_handler() {
    naked_asm!(
        // Ne pas sauvegarder les registres, le irq_handler l'a déjà fait
        
        // 1. Lire IF (Interrupt Flags)
        "ldr r0, =0x04000214",
        "ldr r1, [r0]",            // r1 = quelles IRQ sont actives
        
        // 2. Lire DTCM_TOP dynamiquement
        "mrc p15, #0, r2, c9, c1, #0",
        "lsr r2, r2, #12",
        "lsl r2, r2, #12",
        "add r2, r2, #0x1000",     // r2 = DTCM_TOP
        
        // 3. Mettre à jour les wait flags (DTCM+0x3FF8 = offset -8)
        "ldr r3, [r2, #-8]",       // Lire flags actuels
        "orr r3, r3, r1",          // Ajouter nouvelles IRQ
        "str r3, [r2, #-8]",       // Réécrire
        
        // 4. Acquitter IF
        "str r1, [r0]",            // Write 1 to acknowledge
        
        // Retour (le irq_handler fera pop)
        "bx lr",
    )
}

/// Init: Installe le default handler si le jeu n'en fournit pas
/// À appeler depuis reset_handler après init_stacks
#[link_section = ".text.init_default_irq"]
#[no_mangle]
pub unsafe extern "C" fn init_default_irq() {
    use core::arch::asm;
    
    asm!(
        // Lire DTCM_TOP
        "mrc p15, #0, r0, c9, c1, #0",
        "lsr r0, r0, #12",
        "lsl r0, r0, #12",
        "add r0, r0, #0x1000",
        
        // Check si le jeu a déjà mis un handler
        "ldr r1, [r0, #-4]",       // DTCM+0x3FFC
        "cmp r1, #0",
        "bxne lr",                 // Si déjà défini, ne rien faire
        
        // Installer notre default handler
        "ldr r1, =default_irq_handler",
        "str r1, [r0, #-4]",
        
        "bx lr",
        
        out("r0") _,
        out("r1") _,
        options(nostack)
    );
}

// USAGE dans reset_handler:
// 
// reset_handler:
//     bl init_stacks
//     bl init_default_irq  ← AJOUTER ICI
//     ldr r12, =0x027FFE24
//     ldr pc, [r12]