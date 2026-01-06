//! Decompression Functions
//! 
//! LZ77 decompression implementation compatible with BIOS SWI 0x11.

#[no_mangle]
pub unsafe extern "C" fn swi_lz77_wram(source: *const u8, dest: *mut u8) {
    // Lecture de l'en-tête (32 bits)
    // Bits 0-3: Réservé
    // Bits 4-7: Type de compression (doit être 1 pour LZ77)
    // Bits 8-31: Taille décompressée
    let header = *(source as *const u32);
    let decompressed_size = header >> 8;
    
    let mut src_offset = 4; // On commence après l'en-tête
    let mut dst_offset = 0;
    
    while dst_offset < decompressed_size {
        // Lecture du byte de flags (8 indicateurs pour les 8 prochains blocs)
        let flags = *source.add(src_offset);
        src_offset += 1;
        
        for i in (0..8).rev() {
            if dst_offset >= decompressed_size {
                break;
            }
            
            // Si le bit est 0 : Copie brute (Literal)
            // Si le bit est 1 : Donnée compressée (LZ77 tuple)
            if (flags & (1 << i)) == 0 {
                *dest.add(dst_offset as usize) = *source.add(src_offset);
                dst_offset += 1;
                src_offset += 1;
            } else {
                // Lecture de 2 octets pour la longueur et la distance
                let byte1 = *source.add(src_offset) as u32;
                let byte2 = *source.add(src_offset + 1) as u32;
                src_offset += 2;
                
                // Format:
                // byte1 (high 4): Length - 3 (MSB)
                // byte1 (low 4): Disp MSB
                // byte2: Disp LSB
                
                let length = (byte1 >> 4) + 3;
                let disp = ((byte1 & 0x0F) << 8) | byte2;
                
                // Copie depuis le buffer de sortie (fenêtre glissante)
                let copy_src = dst_offset - disp - 1;
                
                for k in 0..length {
                    if dst_offset >= decompressed_size {
                        break;
                    }
                    let val = *dest.add((copy_src + k) as usize);
                    *dest.add(dst_offset as usize) = val;
                    dst_offset += 1;
                }
            }
        }
    }
}