// arm9/src/memory.rs
// Implémentation des SWI 0x0B (CpuSet) et 0x0C (CpuFastSet)

/// SWI 0x0B: CpuSet
/// Copie ou remplit la mémoire par blocs de 16 ou 32 bits.
/// r0: Source, r1: Dest, r2: Control
/// Control:
/// - Bit 0-20: Compte (nombre d'unités à transférer)
/// - Bit 24:   Mode Source (0 = Incrément, 1 = Fixe)
/// - Bit 26:   Taille (0 = 16-bit, 1 = 32-bit)
#[no_mangle]
pub unsafe extern "C" fn SVC_CpuSet(src: *const u8, dst: *mut u8, control: u32) {
    let count = (control & 0x1FFFFF) as usize;
    let fixed_source = (control & (1 << 24)) != 0;
    let size_32 = (control & (1 << 26)) != 0;

    if size_32 {
        let s = src as *const u32;
        let d = dst as *mut u32;
        if fixed_source {
            // Mode "Fill" (Source fixe 32-bit)
            let val = *s;
            for i in 0..count {
                *d.add(i) = val;
            }
        } else {
            // Mode "Copy" (32-bit)
            // On utilise une boucle simple pour éviter les dépendances externes lourdes
            for i in 0..count {
                *d.add(i) = *s.add(i);
            }
        }
    } else {
        let s = src as *const u16;
        let d = dst as *mut u16;
        if fixed_source {
            // Mode "Fill" (Source fixe 16-bit)
            let val = *s;
            for i in 0..count {
                *d.add(i) = val;
            }
        } else {
            // Mode "Copy" (16-bit)
            for i in 0..count {
                *d.add(i) = *s.add(i);
            }
        }
    }
}

/// SWI 0x0C: CpuFastSet
/// Copie ou remplit la mémoire par blocs de 32 bits (toujours x32).
/// Optimisé pour les gros transferts (doit être aligné sur 4 octets).
/// r0: Source, r1: Dest, r2: Control
#[no_mangle]
pub unsafe extern "C" fn SVC_CpuFastSet(src: *const u32, dst: *mut u32, control: u32) {
    let count = (control & 0x1FFFFF) as usize;
    let fixed_source = (control & (1 << 24)) != 0;

    if fixed_source {
        // Mode "Fill" rapide
        let val = *src;
        for i in 0..count {
            *dst.add(i) = val;
        }
    } else {
        // Mode "Copy" rapide
        // Le BIOS original utilise des instructions LDM/STM (blocs de 32 octets)
        // Ici, une boucle simple suffit car le compilateur peut la vectoriser/optimiser
        for i in 0..count {
            *dst.add(i) = *src.add(i);
        }
    }
}

