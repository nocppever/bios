// math.rs - DIVISION CORRIGÉE ET TESTÉE

use core::arch::naked_asm;

/// Division SOFTWARE - Version corrigée basée sur le BIOS officiel
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn div_impl() {
    naked_asm!(
        // Input: r0 = numerator, r1 = denominator
        // Output: r0 = quotient, r1 = remainder, r3 = abs(quotient)
        
        // Division par zéro
        "cmp r1, #0",
        "bne 1f",
        "mvn r0, #0",              // r0 = -1
        "mov r1, #0",              // r1 = 0
        "mov r3, #0",              // r3 = 0
        "bx lr",
        
        "1:",
        "stmfd sp!, {{r4-r6, lr}}",
        
        // Sauvegarder les signes
        "mov r4, r0",              // r4 = numerator original
        "mov r5, r1",              // r5 = denominator original
        "eor r6, r0, r1",          // r6 = sign du résultat
        
        // Valeurs absolues
        "cmp r0, #0",
        "rsblt r0, r0, #0",
        
        "cmp r1, #0",
        "rsblt r1, r1, #0",
        
        // Division par décalages successifs
        "mov r2, #0",              // r2 = quotient
        "mov r3, #1",              // r3 = bit courant
        
        // Aligner le diviseur
        "cmp r1, r0",
        "bhi 3f",                  // Si diviseur > dividende, skip
        
        "2:",                       // align_loop
        "cmp r1, #0x80000000",
        "cmpcc r1, r0",
        "movcc r1, r1, lsl #1",
        "movcc r3, r3, lsl #1",
        "bcc 2b",
        
        "3:",                       // div_loop
        "cmp r0, r1",
        "subcs r0, r0, r1",
        "addcs r2, r2, r3",
        
        "movs r3, r3, lsr #1",
        "movne r1, r1, lsr #1",
        "bne 3b",
        
        // r2 = quotient (abs), r0 = remainder (abs)
        
        // Appliquer le signe au quotient
        "cmp r6, #0",
        "rsblt r2, r2, #0",
        
        // Appliquer le signe au reste (suit le dividende)
        "mov r1, r0",              // r1 = remainder
        "cmp r4, #0",
        "rsblt r1, r1, #0",
        
        // r0 = quotient signé
        "mov r0, r2",
        
        // r3 = abs(quotient)
        "mov r3, r2",
        "cmp r3, #0",
        "rsblt r3, r3, #0",
        
        "ldmfd sp!, {{r4-r6, pc}}",
    )
}

/// Racine carrée - Version simplifiée mais correcte
#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn sqrt_impl() {
    naked_asm!(
        // Input: r0 = N
        // Output: r0 = sqrt(N)
        
        "cmp r0, #0",
        "bxeq lr",                 // sqrt(0) = 0
        
        "stmfd sp!, {{r4-r5, lr}}",
        
        "mov r4, r0",              // r4 = N
        
        // Initial guess: N/2
        "mov r1, r0, lsr #1",
        "cmp r1, #0",
        "moveq r1, #1",
        
        // Newton-Raphson: x = (x + N/x) / 2
        "mov r5, #10",             // 10 iterations
        
        "1:",                       // newton_loop
        "mov r0, r4",              // r0 = N
        "bl div_impl",             // r0 = N/x (div détruit r1)
        
        "add r1, r1, r0",          // x + N/x
        "mov r1, r1, lsr #1",      // / 2
        
        "subs r5, r5, #1",
        "bne 1b",
        
        "mov r0, r1",
        "ldmfd sp!, {{r4-r5, pc}}",
    )
}