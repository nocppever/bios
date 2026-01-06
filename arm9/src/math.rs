// arm9/src/math.rs
/* use core::arch::naked_asm;

/// Division signée (SWI 0x09)
/// r0: numérateur, r1: dénominateur -> r0: quotient, r1: reste, r3: abs(num)
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn SVC_Div() {
    naked_asm!(
        "ands r3, r1, #128, #8",
        "rsbmi r1, r1, #0",
        "eors r12, r3, r0, asr #32",
        "rsbhs r0, r0, #0",
        "movs r2, r1",
        "1:",
        "cmp r2, r0, lsr #1",
        "lslls r2, r2, #1",
        "blo 1b",
        "2:",
        "cmp r0, r2",
        "adc r3, r3, r3",
        "subhs r0, r0, r2",
        "teq r2, r1",
        "lsrne r2, r2, #1",
        "bne 2b",
        "mov r1, r0",
        "mov r0, r3",
        "lsls r12, r12, #1",
        "rsbhs r0, r0, #0",
        "rsbmi r1, r1, #0",
        "bx lr",
    )
}

/// Racine carrée (SWI 0x0D)
/// r0: val -> r0: sqrt(val)
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn SVC_Sqrt() {
    naked_asm!(
        "push {{r4, r5}}",
        "mov r12, r0",
        "mov r1, #1",
        "1:",
        "cmp r0, r1",
        "lsrhi r0, r0, #1",
        "lslhi r1, r1, #1",
        "bhi 1b",
        "2:",
        "mov r0, r12",
        "mov r4, r1",
        "mov r3, #0",
        "mov r2, r1",
        "3:",
        "cmp r2, r0, lsr #1",
        "lslls r2, r2, #1",
        "blo 3b",
        "4:",
        "cmp r0, r2",
        "adc r3, r3, r3",
        "subhs r0, r0, r2",
        "teq r2, r1",
        "lsrne r2, r2, #1",
        "bne 4b",
        "add r1, r1, r3",
        "lsrs r1, r1, #1",
        "cmp r1, r4",
        "blo 2b",
        "mov r0, r4",
        "pop {{r4, r5}}",
        "bx lr",
    )
} */

// arm9/src/math.rs
// Refactoring : Implémentation pure Rust des algorithmes mathématiques.

/// Racine carrée entière (SWI 0x0D)
/// Algorithme "Digit-by-digit" (base 2)
/// Entrée : r0 (val) -> Sortie : r0 (sqrt)
#[no_mangle]
pub extern "C" fn SVC_Sqrt(val: u32) -> u32 {
    let mut op = val;
    let mut res = 0;
    let mut one = 1u32 << 30; // Le second bit le plus fort

    // "one" commence à la puissance de 4 la plus haute <= argument
    while one > op {
        one >>= 2;
    }

    while one != 0 {
        if op >= res + one {
            op -= res + one;
            res = (res >> 1) + one;
        } else {
            res >>= 1;
        }
        one >>= 2;
    }
    res
}

/// Division signée (SWI 0x09)
/// Entrée : r0 (num), r1 (den)
/// Sortie : r0 (quotient), r1 (reste) -> simulé par un retour i64
/// Note : Le BIOS original met aussi abs(num) dans r3, ce que Rust ignore ici (généralement sans danger).
#[no_mangle]
pub extern "C" fn SVC_Div(numerator: i32, denominator: i32) -> i64 {
    // Gestion de la division par zéro (Le BIOS a un comportement indéfini, on renvoie 0)
    if denominator == 0 {
        return 0;
    }

    // Gestion des signes
    let n_neg = numerator < 0;
    let d_neg = denominator < 0;

    // Valeurs absolues (On évite .abs() pour ne pas dépendre de core/std inutilement)
    let abs_n = if n_neg { 0u32.wrapping_sub(numerator as u32) } else { numerator as u32 };
    let abs_d = if d_neg { 0u32.wrapping_sub(denominator as u32) } else { denominator as u32 };

    // Algorithme de division euclidienne bit-à-bit (Long Division)
    // On le fait manuellement pour éviter que le compilateur n'appelle __aeabi_uidiv
    let mut quotient: u32 = 0;
    let mut remainder: u32 = 0;

    // On parcourt les 32 bits
    for i in (0..32).rev() {
        remainder <<= 1;
        // On descend le bit i du numérateur
        remainder |= (abs_n >> i) & 1;

        if remainder >= abs_d {
            remainder -= abs_d;
            quotient |= 1 << i;
        }
    }

    // Application des signes
    // Quotient est négatif si les signes diffèrent
    let final_q = if n_neg ^ d_neg { 
        0i32.wrapping_sub(quotient as i32) 
    } else { 
        quotient as i32 
    };
    
    // Le reste a le signe du numérateur (convention C et Rust)
    let final_r = if n_neg { 
        0i32.wrapping_sub(remainder as i32) 
    } else { 
        remainder as i32 
    };

    // MAGIE ABI ARM :
    // Une valeur 64 bits est retournée dans r0 (bits 0-31) et r1 (bits 32-63).
    // On place le quotient dans r0 et le reste dans r1.
    // CORRECTION : Cast explicite en i64 pour satisfaire la signature
    (((final_r as u32 as u64) << 32) | (final_q as u32 as u64)) as i64
}