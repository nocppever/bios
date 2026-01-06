// arm9/src/decompress.rs
// Implémentation des algorithmes de décompression (SWI 0x14, 0x16, 0x18)

/// SWI 0x16: Diff8UnFilter
/// Applique un filtre différentiel 8-bits (chaque octet est la somme du précédent et de la valeur lue).
/// r0: Source (Header inclus), r1: Destination
#[no_mangle]
pub unsafe extern "C" fn SVC_Diff8UnFilter(src: *const u8, dst: *mut u8) {
    // Lecture du header (32 bits)
    // Le header contient la taille sur 24 bits (>> 8)
    let header = *(src as *const u32);
    let count = (header >> 8) as usize;

    let mut current_src = src.add(4); // On saute le header
    let mut current_dst = dst;
    
    // Le BIOS lit d'abord la valeur initiale
    let mut accumulator: u8 = *current_src;
    current_src = current_src.add(1);
    
    *current_dst = accumulator;
    current_dst = current_dst.add(1);

    // Boucle principale
    // Note: count est le nombre total d'octets, on en a déjà traité 1
    for _ in 0..(count - 1) {
        let val = *current_src;
        current_src = current_src.add(1);

        // L'addition doit wrapper (déborder silencieusement)
        accumulator = accumulator.wrapping_add(val);
        
        *current_dst = accumulator;
        current_dst = current_dst.add(1);
    }
}

/// SWI 0x18: Diff16UnFilter
/// Applique un filtre différentiel 16-bits.
/// r0: Source (Header inclus), r1: Destination
#[no_mangle]
pub unsafe extern "C" fn SVC_Diff16UnFilter(src: *const u8, dst: *mut u8) {
    let header = *(src as *const u32);
    let count = (header >> 8) as usize; // Nombre d'octets total

    let mut current_src = src.add(4) as *const u16;
    let mut current_dst = dst as *mut u16;
    
    // On travaille par mots de 16 bits, donc on divise count par 2
    let count_u16 = count / 2;

    let mut accumulator: u16 = *current_src;
    current_src = current_src.add(1);
    
    *current_dst = accumulator;
    current_dst = current_dst.add(1);

    for _ in 0..(count_u16 - 1) {
        let val = *current_src;
        current_src = current_src.add(1);

        accumulator = accumulator.wrapping_add(val);
        
        *current_dst = accumulator;
        current_dst = current_dst.add(1);
    }
}

/// SWI 0x14: RLUnCompWRAM
/// Décompression Run-Length (RLE) vers la WRAM (écriture 8-bits autorisée).
/// r0: Source, r1: Destination
#[no_mangle]
pub unsafe extern "C" fn SVC_RLUnCompWRAM(src: *const u8, dst: *mut u8) {
    let header = *(src as *const u32);
    // Taille décompressée sur 24 bits
    let mut remaining_len = (header >> 8) as usize;

    let mut current_src = src.add(4);
    let mut current_dst = dst;

    if remaining_len == 0 {
        return;
    }

    while remaining_len > 0 {
        // Lire le byte de drapeau
        let flag_byte = *current_src;
        current_src = current_src.add(1);

        // Bit 7 : Type (0 = Non compressé, 1 = Compressé)
        let compressed = (flag_byte & 0x80) != 0;
        // Bits 0-6 : Compte (nombre d'octets à traiter - voir spec)
        // La spec dit : count = (flag & 0x7F) + N
        // Pour copy (raw) : N=1, donc count = flag + 1
        // Pour compressed (rle) : N=3, donc count = flag + 3
        let count_bits = (flag_byte & 0x7F) as usize;

        if compressed {
            // RLE: Lire 1 octet et le répéter (count + 3) fois
            let run_len = count_bits + 3;
            let val = *current_src;
            current_src = current_src.add(1);

            for _ in 0..run_len {
                *current_dst = val;
                current_dst = current_dst.add(1);
            }
            remaining_len -= run_len;
        } else {
            // RAW: Copier (count + 1) octets tels quels
            let copy_len = count_bits + 1;
            
            for _ in 0..copy_len {
                *current_dst = *current_src;
                current_src = current_src.add(1);
                current_dst = current_dst.add(1);
            }
            remaining_len -= copy_len;
        }
    }
}