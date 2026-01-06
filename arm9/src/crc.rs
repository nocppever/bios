// arm9/src/crc.rs
/* 
// On utilise le nom exact du symbole assembleur pour que le linker fasse le lien
#[no_mangle]
#[link_section = ".text.crc"] // Optionnel, aide à organiser
pub unsafe extern "C" fn SVC_GetCRC16(start_crc: u16, data: *const u8, len: u32) -> u16 {
    let mut crc = start_crc;
    let mut ptr = data;
    let mut count = len;

    // Table CRC16 standard du BIOS DS (Poly 0xA001 inversé)
    // On la met en const pour qu'elle soit peut-être inlinée ou en rodata
    const CRC_TABLE: [u16; 16] = [
        0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401,
        0xA001, 0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400
    ];

    while count > 0 {
        let byte = *ptr;
        
        // Logique "Nibble-based" du BIOS original (optimisée pour la taille)
        // Le BIOS traite par demi-octets (nibbles) pour économiser la taille de la table (32 octets vs 512)
        
        // Low nibble
        let mut idx = (crc ^ (byte as u16)) & 0xF;
        crc = (crc >> 4) ^ CRC_TABLE[idx as usize];

        // High nibble
        idx = (crc ^ ((byte >> 4) as u16)) & 0xF;
        crc = (crc >> 4) ^ CRC_TABLE[idx as usize];

        ptr = ptr.add(1);
        count -= 1;
    }

    crc
} */

//! CRC Functions - Implementation of SWI 0x0E
//! Refactoring: Utilisation de const fn pour générer la table à la compilation.

/// Génère la table de CRC16 (poly 0xA001) à la compilation.
/// Cela évite d'avoir des "magic numbers" dans le code source.
const fn generate_nibble_table() -> [u16; 16] {
    let mut table = [0u16; 16];
    let poly = 0xA001;
    let mut i = 0;
    
    while i < 16 {
        let mut crc = 0;
        let mut c = i as u16;
        let mut j = 0;
        
        // On simule le passage de 4 bits (un nibble)
        while j < 4 {
            if ((crc ^ c) & 1) != 0 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
            c >>= 1;
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

// La table est stockée dans la section .rodata du binaire final
static CRC_TABLE: [u16; 16] = generate_nibble_table();

#[no_mangle]
#[link_section = ".text.crc"]
pub unsafe extern "C" fn SVC_GetCRC16(start_crc: u16, data: *const u8, len: u32) -> u16 {
    let mut crc = start_crc;
    let mut ptr = data;
    
    // COMPORTEMENT BIOS: Le BIOS aligne la longueur sur 2 octets inférieurs.
    // Si len est impair (ex: 3), il ne traite que 2 octets.
    // (lsrs r2, r2, #1 puis lsls r2, r2, #1 dans bios9.s)
    let aligned_len = len & !1; 
    let mut i = 0;

    while i < aligned_len {
        let byte = *ptr;
        
        // Traitement du demi-octet faible (Low Nibble)
        let idx_lo = (crc ^ (byte as u16)) & 0xF;
        crc = (crc >> 4) ^ CRC_TABLE[idx_lo as usize];

        // Traitement du demi-octet fort (High Nibble)
        let idx_hi = (crc ^ ((byte >> 4) as u16)) & 0xF;
        crc = (crc >> 4) ^ CRC_TABLE[idx_hi as usize];

        ptr = ptr.add(1);
        i += 1;
    }
    
    crc
}