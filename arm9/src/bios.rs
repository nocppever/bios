// arm9/src/bios_asm.rs
// VERSION CORRIGÉE : GetCRC16 supprimé, _076C réparé.

use core::arch::global_asm;

global_asm!(r#"
    .cpu arm946e-s
    .arch armv5te
    .fpu softvfp
    .syntax unified

    // --- CONSTANTS ---
    .set HW_BIOS_EXCP_STACK_MAIN_END, 0x027FFD9C

    // --- SECTIONS ---
    .section .text._start, "ax"
    .global _start
    .global __reset_handler
    
_start:
    b __reset_handler    @ Reset
    b __reserved_vector  @ Undefined instruction
    b __svc_handler      @ SWI
    b __reserved_vector  @ Prefetch Abort
    b __reserved_vector  @ Data Abort
    b __reserved_vector  @ Reserved
    b __irq_handler      @ IRQ
__reserved_vector:
    b __fiq_handler      @ FIQ

    // --- HEADER / CHECKSUM AREA ---
_0020:
    .byte 0x24, 0xFF, 0xAE, 0x51, 0x69, 0x9A, 0xA2, 0x21, 0x3D, 0x84, 0x82, 0x0A, 0x84, 0xE4, 0x09, 0xAD
    .byte 0x11, 0x24, 0x8B, 0x98, 0xC0, 0x81, 0x7F, 0x21, 0xA3, 0x52, 0xBE, 0x19, 0x93, 0x09, 0xCE, 0x20
    .byte 0x10, 0x46, 0x4A, 0x4A, 0xF8, 0x27, 0x31, 0xEC, 0x58, 0xC7, 0xE8, 0x33, 0x82, 0xE3, 0xCE, 0xBF
    .byte 0x85, 0xF4, 0xDF, 0x94, 0xCE, 0x4B, 0x09, 0xC1, 0x94, 0x56, 0x8A, 0xC0, 0x13, 0x72, 0xA7, 0xFC
    .byte 0x9F, 0x84, 0x4D, 0x73, 0xA3, 0xCA, 0x9A, 0x61, 0x58, 0x97, 0xA3, 0x27, 0xFC, 0x03, 0x98, 0x76
    .byte 0x23, 0x1D, 0xC7, 0x61, 0x03, 0x04, 0xAE, 0x56, 0xBF, 0x38, 0x84, 0x00, 0x40, 0xA7, 0x0E, 0xFD
    .byte 0xFF, 0x52, 0xFE, 0x03, 0x6F, 0x95, 0x30, 0xF1, 0x97, 0xFB, 0xC0, 0x85, 0x60, 0xD6, 0x80, 0x25
    .byte 0xA9, 0x63, 0xBE, 0x03, 0x01, 0x4E, 0x38, 0xE2, 0xF9, 0xA2, 0x34, 0xFF, 0xBB, 0x3E, 0x03, 0x44
    .byte 0x78, 0x00, 0x90, 0xCB, 0x88, 0x11, 0x3A, 0x94, 0x65, 0xC0, 0x7C, 0x63, 0x87, 0xF0, 0x3C, 0xAF
    .byte 0xD6, 0x25, 0xE4, 0x8B, 0x38, 0x0A, 0xAC, 0x72, 0x21, 0xD4, 0xF8, 0x07, 0x56, 0xCF, 0x00, 0x00

    // --- CODE ---

    .arm
    .align 2
    .global __fiq_handler
    .type __fiq_handler, %function
__fiq_handler:
    mrs sp, apsr
    orr sp, sp, #0xc0
    msr cpsr_fsxc, sp
    ldr sp, _02F8 @ =HW_BIOS_EXCP_STACK_MAIN_END
    add sp, sp, #1
__fiq_internal:
    push {{r12, lr}}
    mrs lr, spsr
    mrc p15, #0, r12, c1, c0, #0
    push {{r12, lr}}
    bic r12, r12, #1
    mcr p15, #0, r12, c1, c0, #0
    bic r12, sp, #1
    ldr r12, [r12, #0x10]
    cmp r12, #0
    blxne r12
    pop {{r12, lr}}
    mcr p15, #0, r12, c1, c0, #0
    msr spsr_fsxc, lr
    pop {{r12, lr}}
    subs pc, lr, #4

    .arm
    .align 2
    .global __reset_handler
    .type __reset_handler, %function
__reset_handler:
    cmp lr, #0
    moveq lr, #4
    mov r12, #64, #12
    ldrb r12, [r12, #0x300]
    teq r12, #1
    mrseq r12, apsr
    orreq r12, r12, #0xc0
    msreq cpsr_fsxc, r12
    ldreq sp, _02F8 @ =HW_BIOS_EXCP_STACK_MAIN_END
    beq __fiq_internal
    mov r4, #64, #12
    mov r0, #0
    mov r1, #0x80
    str r0, [r4, #0x1a4]
    str r1, [r4, #0x1a1]
    mov r0, #0xdf
    msr cpsr_fsxc, r0
    mov r0, #128, #26
    blx SVC_WaitByLoop
    ldr r4, _037C @ =0x04000204
    mov r1, #128, #26
    strh r1, [r4]
    mov r0, #128, #26
    blx SVC_WaitByLoop
    ldr r3, _0380 @ =0x027FFFFE
    ldr r1, _0384 @ =0x0000FFDF
    ldr r2, _0388 @ =0x0000E732
    ldr r12, _038C @ =0x027E57FE
    ldrh r0, [r3]
    strh r0, [r3]
    strh r0, [r3]
    strh r1, [r3]
    strh r2, [r3]
    ldrh r0, [r12]
    ldr r4, _037C @ =0x04000204
    mov r1, #96, #24
    strh r1, [r4]
    bl sub_0790
    mov r4, #64, #12
    strb r4, [r4, #0x208]
    bl sub_021C
    ldr r0, _0390 @ =sub_03B6+1
    blx r0
    bl sub_01FC
    mov r12, #160, #14
    ldr lr, [r12, #-0x1dc]
    sub r12, r12, #128, #28
    ldrh r0, [r12, #0x2c]
    cmp r0, #0
_01D4:
    bne _01D4
    ldr r12, [r12, #0x20]
    cmp r12, #0
_01E0:
	beq _01E0
	bx r12

    .arm
    .align 2
    .global SVC_SoftReset
SVC_SoftReset: @ 0xFFFF01E8
    bl sub_01FC
    mov r12, #160, #14
    ldr lr, [r12, #-0x1dc]
    mov r12, #0
    bx lr

    .arm
    .align 2
sub_01FC: @ 0xFFFF01FC
    mov r12, lr
    mov r0, #0x1f
    msr cpsr_fsxc, r0
    ldr r0, _0394 @ =0x00012078
    bl sub_0778
    bl sub_021C
    ldmdb r4, {{r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11}}
    bx r12

    .arm
    .align 2
sub_021C: @ 0xFFFF021C
    mov r0, #0xd3
    msr cpsr_fsxc, r0
    ldr sp, _02F4 @ =0x00803FC0
    mov lr, #0
    msr spsr_fsxc, lr
    mov r0, #0xd2
    msr cpsr_fsxc, r0
    ldr sp, _02F0 @ =0x00803FA0
    mov lr, #0
    msr spsr_fsxc, lr
    mov r0, #0x5f
    msr cpsr_fsxc, r0
    ldr sp, _02EC @ =0x00803EC0
    @ Clear DTCM
    // MACRO EXPANSION LD_DTCM_SYSRV_TOP r4
    mrc p15, #0, r4, c9, c1, #0
    lsr r4, r4, #12
    lsl r4, r4, #12
    add r4, r4, #64, #24
    // END MACRO
    adr r0, _0268+1
    bx r0
    .thumb
_0268:
    movs r0, #0
    ldr r1, _0398 @ =-0x200
_026C:
    str r0, [r4, r1]
    adds r1, #4
    blt _026C
    bx lr

    .arm
    .align 2
    .global __irq_handler
__irq_handler: @ 0xFFFF0274
    push {{r0, r1, r2, r3, r12, lr}}
    @ Call the interrupt vector from DTCM
    // MACRO EXPANSION LD_DTCM_SYSRV_TOP r0
    mrc p15, #0, r0, c9, c1, #0
    lsr r0, r0, #12
    lsl r0, r0, #12
    add r0, r0, #64, #24
    // END MACRO
    adr lr, _0290
    ldr pc, [r0, #-4]
_0290:
    pop {{r0, r1, r2, r3, r12, lr}}
    subs pc, lr, #4

    // --- __svc_handler et SVCTable SUPPRIMÉS ICI ---

    .thumb
    .thumb_func
SVC_CustomPost: @ 0xFFFF02E4
    ldr r2, _039C @ =0x04000300
    str r0, [r2]
	bx lr
    
    .align 2
_02EC: .4byte 0x00803EC0
_02F0: .4byte 0x00803FA0
_02F4: .4byte 0x00803FC0
_02F8: .4byte HW_BIOS_EXCP_STACK_MAIN_END

    // SVCTable SUPPRIMÉE

_037C: .4byte 0x04000204
_0380: .4byte 0x027FFFFE
_0384: .4byte 0x0000FFDF
_0388: .4byte 0x0000E732
_038C: .4byte 0x027E57FE
_0390: .4byte sub_03B6+1
_0394: .4byte 0x00012078
_0398: .4byte -0x200
_039C: .4byte 0x04000300

    .thumb
    .thumb_func
sub_03A0: @ 0xFFFF03A0
    ldr r1, _0408 @ =0x04000180
    lsls r0, r0, #8
    strh r0, [r1]
    bx lr

    .thumb
    .thumb_func
sub_03A8: @ 0xFFFF03A8
    ldr r2, _0408 @ =0x04000180
_03AA:
    ldrh r1, [r2]
    lsls r1, r1, #0x1c
    lsrs r1, r1, #0x1c
    cmp r1, r0
    bne _03AA
    bx lr

    .thumb
    .align 1  @ non-word-aligned
sub_03B6: @ 0xFFFF03B6
    push {{r4, lr}}
    ldr r4, _0408 @ =0x04000180
    movs r0, #0
    strh r0, [r4]
    bl sub_03A8
    lsrs r0, r4, #0x12
    strh r0, [r4]
    movs r0, #1
    bl sub_03A8
    bl sub_0410
    bl sub_042A
    ldr r0, _0408 @ =0x04000180
    ldr r1, _040C @ =0x0000E880
    adds r0, #0x80
    strh r1, [r0, #4]
    lsls r1, r0, #0x14
    str r1, [r4, #0x24]
    lsrs r1, r0, #0x11
    strh r1, [r4]
_03E4:
    ldrh r1, [r4]
    lsls r1, r1, #0x1c
    lsrs r1, r1, #0x1c
    cmp r1, #3
    bne _03E4
    movs r1, #0
    str r1, [r0, #8]
    str r1, [r0, #0x10]
    mvns r1, r1
    str r1, [r0, #0x14]
    movs r0, #3
    bl sub_03A8
    movs r0, #3
    lsls r0, r0, #8
    strh r0, [r4]
    pop {{r4, pc}}

    .align 2
_0408: .4byte 0x04000180
_040C: .4byte 0x0000E880

    .thumb
    .thumb_func
sub_0410: @ 0xFFFF0410
    ldr r1, _0758 @ =0x04000200
    movs r0, #0
    str r0, [r1, #8]
    str r0, [r1, #0x10]
    mvns r2, r0
    str r2, [r1, #0x14]
    ldr r2, _0758 @ =0x04000200
    movs r1, #3
    adds r2, #0x40
    strb r1, [r2, #7]
    ldr r1, _075C @ =0x04000300
    strh r0, [r1, #4]
    bx lr

    .thumb
    .align 1
sub_042A: @ 0xFFFF042A
    movs r0, #0
    push {{r3, lr}}
    str r0, [sp]
    movs r1, #1
    lsls r1, r1, #0x17
    mov r0, sp
    ldr r2, _0760 @ =0x01000F80
    blx SVC_CpuFastSet
    movs r0, #0
    str r0, [sp]
    mov r0, sp
    ldr r2, _0764 @ =0x01002000
    ldr r1, _0768 @ =0x027F8000
    blx SVC_CpuFastSet
    pop {{r3, pc}}

    // --- SVC_GetCRC16 SUPPRIMÉ ICI ---

    .thumb
    .align 1
SVC_IsDebugger: @ 0xFFFF048A
    push {{r4, r5, r6, lr}}
    ldr r5, _076C @ =DebuggerIdent+8
    ldr r4, _0770 @ =0x023FFFE0
    movs r1, #0
    movs r0, #0
    ldr r3, _0774 @ =0x027FFFE0
    subs r5, #8
_0498:
    lsls r2, r0, #1
    ldrh r2, [r5, r2]
    strh r2, [r3, #0x18]
    ldrh r6, [r4, #0x18]
    cmp r6, r2
    beq _04A6
    adds r1, #1
_04A6:
    adds r0, #1
    cmp r0, #4
    blt _0498
    movs r0, #0
    strh r0, [r3, #0x18]
    movs r0, #1
    cmp r1, #3
    bhs _04B8
    movs r0, #0
_04B8:
    pop {{r4, r5, r6, pc}}

    .thumb
    .align 1
SVC_HuffUnComp: @ 0xFFFF04BA
    push {{r0, r1, r2, r3, r4, r5, r6, r7, lr}}
    sub sp, #0x1c
    ldr r1, [sp, #0x28]
    add r2, sp, #0x1c
    ldr r3, [r1]
    movs r7, #0
    movs r6, #0
    ldm r2, {{r0, r1, r2}}
    blx r3
    lsls r2, r0, #0x1c
    lsrs r2, r2, #0x1c
    lsls r1, r2, #0x1d
    lsrs r1, r1, #0x1d
    adds r1, #4
    str r1, [sp, #4]
    movs r1, #0x20
    subs r3, r1, r2
    asrs r5, r0, #8
    str r5, [sp, #0xc]
    cmp r0, #0
    str r3, [sp]
    str r2, [sp, #8]
    bge _04EC
    str r0, [sp, #0xc]
    b _0590
_04EC:
    ldr r0, [sp, #0x1c]
    ldr r1, [sp, #0x28]
    adds r0, #4
    str r0, [sp, #0x1c]
    ldr r1, [r1, #8]
    blx r1
    ldr r1, [sp, #0x24]
    strb r0, [r1]
    lsls r0, r0, #1
    adds r0, #2
    str r0, [sp, #0x14]
    movs r4, #1
    b _0518
_0506:
    ldr r0, [sp, #0x1c]
    ldr r1, [sp, #0x28]
    adds r0, #1
    str r0, [sp, #0x1c]
    ldr r1, [r1, #8]
    blx r1
    ldr r1, [sp, #0x24]
    strb r0, [r1, r4]
    adds r4, #1
_0518:
    ldr r0, [sp, #0x14]
    cmp r4, r0
    blt _0506
    ldr r0, [sp, #0x1c]
    ldr r4, [sp, #0x24]
    adds r0, #1
    adds r4, #1
    str r4, [sp, #0x18]
    str r0, [sp, #0x1c]
    b _058C
_052C:
    movs r1, #0x20
    str r1, [sp, #0x10]
    ldr r1, [sp, #0x28]
    ldr r1, [r1, #0x10]
    ldr r0, [sp, #0x1c]
    blx r1
    ldr r1, [sp, #0x1c]
    adds r1, #4
    str r1, [sp, #0x1c]
    b _0584
_0540:
    ldrb r2, [r4]
    lsrs r4, r4, #1
    lsrs r1, r0, #0x1f
    adds r3, r2, #0
    lsls r2, r2, #0x1a
    lsrs r2, r2, #0x1a
    adds r2, r4, r2
    adds r2, #1
    lsls r2, r2, #1
    adds r4, r2, r1
    lsls r3, r1
    lsls r1, r3, #0x18
    bpl _057E
    ldr r2, [sp, #8]
    adds r6, #1
    lsrs r7, r2
    ldrb r2, [r4]
    ldr r3, [sp]
    adds r1, r7, #0
    lsls r2, r3
    orrs r2, r1
    ldr r1, [sp, #4]
    adds r7, r2, #0
    cmp r6, r1
    ldr r4, [sp, #0x18]
    bne _057E
    ldr r1, [sp, #0x20]
    subs r5, #4
    stm r1!, {{r7}}
    str r1, [sp, #0x20]
    movs r6, #0
_057E:
    cmp r5, #0
    ble _0590
    lsls r0, r0, #1
_0584:
    ldr r1, [sp, #0x10]
    subs r1, #1
    str r1, [sp, #0x10]
    bpl _0540
_058C:
    cmp r5, #0
    bgt _052C
_0590:
    ldr r1, [sp, #0x28]
    ldr r1, [r1, #4]
    cmp r1, #0
    beq _05A0
    ldr r0, [sp, #0x1c]
    blx r1
    cmp r0, #0
    blt _05A2
_05A0:
    ldr r0, [sp, #0xc]
_05A2:
    add sp, #0x2c
    pop {{r4, r5, r6, r7, pc}}

    .thumb
    .align 1
SVC_LZ77UnCompVRAM: @ 0xFFFF05A6
    push {{r0, r1, r2, r3, r4, r5, r6, r7, lr}}
    sub sp, #0x1c
    adds r6, r1, #0
    ldr r1, [sp, #0x28]
    adds r7, r0, #0
    movs r5, #0
    movs r4, #0
    ldr r3, [r1]
    adds r1, r6, #0
    blx r3
    asrs r1, r0, #8
    str r1, [sp, #8]
    str r1, [sp, #4]
    cmp r0, #0
    bge _05C8
    str r0, [sp, #8]
    b _068C
_05C8:
    adds r7, #3
    b _0686
_05CC:
    ldr r1, [sp, #0x28]
    adds r0, r7, #1
    ldr r1, [r1, #8]
    adds r7, r0, #0
    blx r1
    str r0, [sp, #0x18]
    movs r0, #8
    str r0, [sp, #0xc]
    b _067E
_05DE:
    ldr r0, [sp, #0x18]
    lsls r0, r0, #0x18
    bmi _0606
    ldr r1, [sp, #0x28]
    adds r0, r7, #1
    ldr r1, [r1, #8]
    adds r7, r0, #0
    blx r1
    lsls r0, r5
    ldr r1, [sp, #4]
    orrs r4, r0
    subs r1, #1
    movs r2, #8
    eors r5, r2
    str r1, [sp, #4]
    bne _0672
    strh r4, [r6]
    adds r6, #2
    movs r4, #0
    b _0672
_0606:
    ldr r1, [sp, #0x28]
    adds r0, r7, #1
    ldr r1, [r1, #8]
    adds r7, r0, #0
    blx r1
    lsrs r3, r0, #4
    lsls r0, r0, #0x1c
    lsrs r1, r0, #0x14
    str r1, [sp, #0x14]
    ldr r1, [sp, #0x28]
    adds r3, #3
    str r3, [sp, #0x10]
    ldr r1, [r1, #8]
    adds r0, r7, #1
    adds r7, r0, #0
    blx r1
    ldr r1, [sp, #0x14]
    movs r2, #8
    orrs r0, r1
    adds r3, r0, #0
    adds r3, #1
    lsls r1, r3, #0x1f
    lsrs r1, r1, #0x1c
    subs r0, r2, r5
    eors r0, r1
    mov r12, r3
    ldr r3, [sp, #0x10]
    ldr r1, [sp, #4]
    subs r1, r1, r3
    str r1, [sp, #4]
    b _066A
_0644:
    subs r1, r2, r5
    lsrs r1, r1, #3
    add r1, r12
    lsrs r1, r1, #1
    lsls r1, r1, #1
    subs r1, r6, r1
    ldrh r1, [r1]
    eors r0, r2
    movs r3, #0xff
    lsls r3, r0
    ands r1, r3
    lsrs r1, r0
    lsls r1, r5
    orrs r4, r1
    eors r5, r2
    bne _066A
    strh r4, [r6]
    adds r6, #2
    movs r4, #0
_066A:
    ldr r1, [sp, #0x10]
    subs r1, #1
    str r1, [sp, #0x10]
    bpl _0644
_0672:
    ldr r1, [sp, #4]
    cmp r1, #0
    ble _068C
    ldr r0, [sp, #0x18]
    lsls r0, r0, #1
    str r0, [sp, #0x18]
_067E:
    ldr r0, [sp, #0xc]
    subs r0, #1
    str r0, [sp, #0xc]
    bpl _05DE
_0686:
    ldr r1, [sp, #4]
    cmp r1, #0
    bgt _05CC
_068C:
    ldr r1, [sp, #0x28]
    ldr r1, [r1, #4]
    cmp r1, #0
    beq _069C
    adds r0, r7, #1
    blx r1
    cmp r0, #0
    blt _069E
_069C:
    ldr r0, [sp, #8]
_069E:
    add sp, #0x2c
    pop {{r4, r5, r6, r7, pc}}

    .thumb
    .align 1
SVC_RLUnCompVRAM: @ 0xFFFF06A2
    push {{r0, r1, r2, r3, r4, r5, r6, r7, lr}}
    sub sp, #0xc
    adds r7, r1, #0
    ldr r1, [sp, #0x18]
    adds r6, r0, #0
    movs r5, #0
    movs r4, #0
    ldr r3, [r1]
    adds r1, r7, #0
    blx r3
    asrs r1, r0, #8
    str r1, [sp, #4]
    str r1, [sp]
    cmp r0, #0
    bge _06C4
    str r0, [sp, #4]
    b _0742
_06C4:
    adds r6, #3
    b _073C
_06C8:
    ldr r1, [sp, #0x18]
    adds r0, r6, #1
    ldr r1, [r1, #8]
    adds r6, r0, #0
    blx r1
    lsls r1, r0, #0x19
    lsrs r1, r1, #0x19
    lsls r0, r0, #0x18
    bmi _070C
    adds r0, r1, #1
    str r0, [sp, #8]
    ldr r1, [sp, #8]
    ldr r0, [sp]
    subs r0, r0, r1
    str r0, [sp]
    b _0702
_06E8:
    ldr r1, [sp, #0x18]
    adds r0, r6, #1
    ldr r1, [r1, #8]
    adds r6, r0, #0
    blx r1
    lsls r0, r5
    orrs r4, r0
    movs r0, #8
    eors r5, r0
    bne _0702
    strh r4, [r7]
    adds r7, #2
    movs r4, #0
_0702:
    ldr r0, [sp, #8]
    subs r0, #1
    str r0, [sp, #8]
    bpl _06E8
    b _073C
_070C:
    adds r1, #3
    ldr r0, [sp]
    str r1, [sp, #8]
    subs r0, r0, r1
    str r0, [sp]
    ldr r1, [sp, #0x18]
    adds r0, r6, #1
    ldr r1, [r1, #8]
    adds r6, r0, #0
    blx r1
    movs r2, #8
    b _0734
_0724:
    adds r1, r0, #0
    lsls r1, r5
    orrs r4, r1
    eors r5, r2
    bne _0734
    strh r4, [r7]
    adds r7, #2
    movs r4, #0
_0734:
    ldr r1, [sp, #8]
    subs r1, #1
    str r1, [sp, #8]
    bpl _0724
_073C:
    ldr r0, [sp]
    cmp r0, #0
    bgt _06C8
_0742:
    ldr r1, [sp, #0x18]
    ldr r1, [r1, #4]
    cmp r1, #0
    beq _0752
    adds r0, r6, #1
    blx r1
    cmp r0, #0
    blt _0754
_0752:
    ldr r0, [sp, #4]
_0754:
    add sp, #0x1c
    pop {{r4, r5, r6, r7, pc}}

    .align 2
_0758: .4byte 0x04000200
_075C: .4byte 0x04000300
_0760: .4byte 0x01000F80
_0764: .4byte 0x01002000
_0768: .4byte 0x027F8000
    // CORRECTION ICI : _076C pointe maintenant vers DebuggerIdent+8
    // Anciennement : .4byte CRC16Table
_076C: .4byte DebuggerIdent + 8
_0770: .4byte 0x023FFFE0
_0774: .4byte 0x027FFFE0

    .arm
    .align 2
sub_0778: @ 0xFFFF0778
    mcr p15, #0, r0, c1, c0, #0
    mov r0, #0
    mcr p15, #0, r0, c7, c5, #0
    mcr p15, #0, r0, c7, c6, #0
    mcr p15, #0, r0, c7, c10, #4
    bx lr

    .arm
    .align 2
sub_0790: @ 0xFFFF0790
    mov r12, lr
    ldr r0, _07B4 @ =0x00002078
    bl sub_0778
    ldr r0, _07B8 @ =0x0080000A
    mcr p15, #0, r0, c9, c1, #0
    mrc p15, #0, r0, c1, c0, #0
    ldr r0, _07BC @ =0x00012078
    mcr p15, #0, r0, c1, c0, #0
    bx r12

    .align 2
_07B4: .4byte 0x00002078
_07B8: .4byte 0x0080000A
_07BC: .4byte 0x00012078

    .arm
    .align 2
    .global SVC_Halt
SVC_Halt: @ 0xFFFF07C0
    mov r0, #0
    mcr p15, #0, r0, c7, c0, #4
    bx lr

    .thumb
    .thumb_func
SVC_WaitByLoop: @ 0xFFFF07CC
    subs r0, #1
    bgt SVC_WaitByLoop
    bx lr
    
    .align 2

    .arm
    .align 2
SVC_VBlankIntrWait: @ 0xFFFF07D4
    mov r0, #1
    mov r1, #1
    
    .arm
    .align 2
SVC_IntrWait: @ 0xFFFF07DC
    push {{r4, lr}}
    cmp r0, #0
    blne sub_0800
_07E8:
    mov lr, #0
    mcr p15, #0, lr, c7, c0, #4
    bl sub_0800
    beq _07E8
    pop {{r4, lr}}
    bx lr

    .arm
    .align 2
sub_0800: @ 0xFFFF0800
    @ Disable IME
    mov r12, #64, #12
    str r12, [r12, #0x208]
    @ Cancel requested interrupt mask
    // MACRO EXPANSION LD_DTCM_SYSRV_TOP r3
    mrc p15, #0, r3, c9, c1, #0
    lsr r3, r3, #12
    lsl r3, r3, #12
    add r3, r3, #64, #24
    // END MACRO
    ldr r2, [r3, #-8]
    ands r0, r1, r2
    eorne r2, r2, r0
    strne r2, [r3, #-8]
    @ Enable IME
    mov r0, #1
    str r0, [r12, #0x208]
    bx lr

   


    .arm
    .align 2
SVC_BitUnPack: @ 0xFFFF09C0
	push {{r4, r5, r6, r7, r8, r9, r10, r11, r12, lr}}
	sub sp, sp, #8
	ldrh r7, [r2]
	ldrb r6, [r2, #2]
	rsb r10, r6, #8
	mov lr, #0
	ldr r11, [r2, #4]
	lsr r8, r11, #0x1f
	ldr r11, [r2, #4]
	lsl r11, r11, #1
	lsr r11, r11, #1
	str r11, [sp]
	ldrb r2, [r2, #3]
	mov r3, #0
_09F8:
	subs r7, r7, #1
	blt _0A58
	mov r11, #0xff
	asr r5, r11, r10
	ldrb r9, [r0], #1
	mov r4, #0
_0A10:
	cmp r4, #8
	bge _09F8
	and r11, r9, r5
	lsrs r12, r11, r4
	cmpeq r8, #0
	beq _0A30
	ldr r11, [sp]
	add r12, r12, r11
_0A30:
	orr lr, lr, r12, lsl r3
	add r3, r3, r2
	cmp r3, #0x20
	blt _0A4C
	str lr, [r1], #4
	mov lr, #0
	mov r3, #0
_0A4C:
	lsl r5, r5, r6
	add r4, r4, r6
	b _0A10
_0A58:
	add sp, sp, #8
	pop {{r4, r5, r6, r7, r8, r9, r10, r11, r12, lr}}
	bx lr

    .arm
    .align 2
SVC_LZ77UnCompWRAM: @ 0xFFFF0A64
	push {{r4, r5, r6, lr}}
	ldr r5, [r0], #4
	lsr r2, r5, #8
_0A70:
	cmp r2, #0
	ble _0AF0
	ldrb lr, [r0], #1
	mov r4, #8
_0A80:
	subs r4, r4, #1
	blt _0A70
	tst lr, #0x80
	bne _0AA4
	ldrb r6, [r0], #1
	swpb r6, r6, [r1]
	add r1, r1, #1
	sub r2, r2, #1
	b _0AE0
_0AA4:
	ldrb r5, [r0]
	mov r6, #3
	add r3, r6, r5, asr #4
	ldrb r6, [r0], #1
	and r5, r6, #0xf
	lsl r12, r5, #8
	ldrb r6, [r0], #1
	orr r5, r6, r12
	add r12, r5, #1
	sub r2, r2, r3
_0ACC:
	ldrb r5, [r1, -r12]
	swpb r5, r5, [r1]
	add r1, r1, #1
	subs r3, r3, #1
	bgt _0ACC
_0AE0:
	cmp r2, #0
	lslgt lr, lr, #1
	bgt _0A80
	b _0A70
_0AF0:
	pop {{r4, r5, r6, lr}}
	bx lr

    .thumb
    .align 1
sub_0B86: @ 0xFFFF0B86
	bx r3

    .thumb
    .thumb_func
sub_0B88: @ 0xFFFF0B88
	bx r2

    .thumb
    .align 1
sub_0B8A: @ 0xFFFF0B8A
	bx r1
	@ 0xFFFF0B8C

	.section .rodata
DebuggerIdent: @ 0xFFFF0B8C
	.2byte 0x56A9
	.2byte 0x695A
	.2byte 0xA695
	.2byte 0x96A5

    // CRC16Table SUPPRIMÉE ICI

"#);