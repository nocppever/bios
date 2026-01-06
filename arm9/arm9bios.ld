/* * ARM9 BIOS Linker Script - CORRIGÉ (avec KEEP pour éviter le GC)
 * Nintendo DS ARM9 BIOS is located at 0xFFFF0000 and is 4KB (0x1000 bytes)
 */

ENTRY(_start)

MEMORY
{
    /* ARM9 BIOS ROM - 4KB at high memory */
    BIOS (rx) : ORIGIN = 0xFFFF0000, LENGTH = 4K
}

SECTIONS
{
    .text 0xFFFF0000 : {
        /* --- 1. VECTEURS D'EXCEPTION (Doivent être au début) --- */
        /* On utilise KEEP() pour forcer le linker à les garder même s'il pense qu'ils sont inutiles */
        KEEP(*(.vectors))
        KEEP(*(.vectors.*))
        
        /* --- 2. CODE DE DÉMARRAGE --- */
        KEEP(*(.text._start))
        KEEP(*(.text.reset_handler))
        
        /* --- 3. GESTIONNAIRES (Handlers) --- */
        /* Crucial : Si on ne met pas KEEP, le linker supprime swi_handler, 
           et donc supprime aussi swi_dispatch, lz77, etc. */
        KEEP(*(.text.swi_handler))
        KEEP(*(.text.swi_*))      /* Garde swi_dispatch et les wrappers */
        
        KEEP(*(.text.irq_handler))
        
        /* --- 4. TOUT LE RESTE DU CODE --- */
        *(.text)
        *(.text.*)
        
        /* --- 5. DONNÉES EN LECTURE SEULE --- */
        *(.rodata)
        *(.rodata.*)
        
        /* Alignement pour être sûr */
        . = ALIGN(4);
    } > BIOS

    /* Data tables (sine, pitch, volume) */
    .data : {
        *(.data)
        *(.data.*)
        . = ALIGN(4);
    } > BIOS

    /* BSS is not needed in ROM BIOS (tout est en ROM ou registres) */
    /DISCARD/ : {
        *(.bss)
        *(.bss.*)
        *(.ARM.exidx)   /* Pas d'unwind tables */
        *(.ARM.exidx.*)
        *(.ARM.extab)
        *(.ARM.extab.*)
        *(.note.gnu.build-id) /* Retire l'ID de build qui prend de la place */
    }
}

/* Vérification finale de la taille (max 4KB) */
ASSERT(SIZEOF(.text) + SIZEOF(.data) <= 0x1000, "BIOS exceeds 4KB!")