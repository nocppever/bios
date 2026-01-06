// arm9/src/misc.rs
// Implémentation des SWI divers (0x03, 0x06, 0x0F, 0x1F)

use core::arch::asm;

/// SWI 0x06: Halt
/// Met le CPU en mode "Wait For Interrupt" (Low Power)
/// Utilise le coprocesseur p15 (c7, c0, 4)
#[no_mangle]
pub unsafe extern "C" fn SVC_Halt() {
    // mcr p15, 0, <Rd>, c7, c0, 4
    // Le registre source importe peu (ici on utilise 0), c'est l'opcode qui déclenche la pause.
    asm!("mcr p15, 0, {0}, c7, c0, 4", in(reg) 0);
}

/// SWI 0x03: WaitByLoop
/// Attente active pendant 'count' itérations.
/// r0: count
#[no_mangle]
pub unsafe extern "C" fn SVC_WaitByLoop(mut count: i32) {
    // En ASM : subs r0, #1; bgt ...
    // En Rust, on utilise black_box pour empêcher le compilateur de supprimer la boucle vide.
    while count > 0 {
        core::hint::black_box(());
        count -= 1;
    }
}

/// SWI 0x1F: CustomPost
/// Écrit une valeur de debug dans le registre POST (0x04000300).
/// r0: valeur
#[no_mangle]
pub unsafe extern "C" fn SVC_CustomPost(val: u32) {
    let post_reg = 0x04000300 as *mut u32;
    post_reg.write_volatile(val);
}

/// SWI 0x0F: IsDebugger
/// Vérifie si un debugger matériel est connecté en comparant des signatures mémoire.
/// Retourne 1 si détecté, 0 sinon.
#[no_mangle]
pub unsafe extern "C" fn SVC_IsDebugger() -> u32 {
    // Signature magique (présente en .rodata dans le BIOS original)
    const DEBUGGER_IDENT: [u16; 4] = [0x56A9, 0x695A, 0xA695, 0x96A5];
    
    // Adresses mémoire spécifiques définies par le hardware DS
    let check_addr = 0x023FFFE0 as *const u16; // Zone miroir système ?
    let result_addr = 0x027FFFE0 as *mut u16;  // Zone DTCM haute

    let mut match_count = 0;

    for i in 0..4 {
        let sig = DEBUGGER_IDENT[i];
        
        // Le BIOS écrit la signature dans la zone de résultat
        // (offset 0x18 converti en index u16 -> 0xC)
        // Note: L'ASM fait `strh r2, [r3, #0x18]`
        result_addr.add(0xC + i).write_volatile(sig);

        // Compare avec la valeur présente à l'adresse de check
        let val = check_addr.add(0xC + i).read_volatile();
        
        if val == sig {
            match_count += 1;
        }
    }

    // Effacer la signature après test
    result_addr.add(0xC).write_volatile(0);

    if match_count == 4 { 1 } else { 0 }
}