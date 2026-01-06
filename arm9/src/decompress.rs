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

#[no_mangle]
pub unsafe extern "C" fn SVC_LZ77UnCompWRAM(src: *const u8, dst: *mut u8) {
    let header = *(src as *const u32);
    // La taille est sur les 24 bits de poids fort du header
    let size = (header >> 8) as usize;

    if size == 0 { return; }

    let mut src_ptr = src.add(4); // Saut du header
    let mut dst_ptr = dst;
    let mut written = 0;

    while written < size {
        // Lire un octet de drapeaux (8 blocs)
        let flags = *src_ptr;
        src_ptr = src_ptr.add(1);

        // Traiter 8 blocs (ou jusqu'à la fin)
        for i in (0..8).rev() {
            if written >= size { break; }

            // Vérifier le bit i (de 7 à 0)
            if (flags & (1 << i)) == 0 {
                // Bit = 0 : Octet brut
                *dst_ptr = *src_ptr;
                dst_ptr = dst_ptr.add(1);
                src_ptr = src_ptr.add(1);
                written += 1;
            } else {
                // Bit = 1 : Bloc compressé (référence arrière)
                // Lire 2 octets :
                // Byte 1 : (Length - 3) sur 4 bits hauts, (Disp High) sur 4 bits bas
                // Byte 2 : (Disp Low)
                let b1 = *src_ptr;
                let b2 = *src_ptr.add(1);
                src_ptr = src_ptr.add(2);

                // Calcul de la longueur (3..18) et du déplacement
                let length = ((b1 >> 4) + 3) as usize;
                let disp = (((b1 & 0x0F) as usize) << 8) | (b2 as usize);

                // Copier depuis le passé (dst - disp - 1)
                // Attention : src et dst peuvent se chevaucher si disp est petit (ex: RLE simulé)
                // On copie donc octet par octet.
                let mut copy_src = dst_ptr.sub(disp + 1);

                for _ in 0..length {
                    if written >= size { break; }
                    *dst_ptr = *copy_src;
                    dst_ptr = dst_ptr.add(1);
                    copy_src = copy_src.add(1);
                    written += 1;
                }
            }
        }
    }
}

/// SWI 0x10: BitUnPack
/// Décompression de bits (Source -> Dest 32-bits).
/// Utilisé pour convertir des données 1/2/4 bits en valeurs 32 bits.
/// r0: Source, r1: Dest, r2: Pointeur vers struct UnpackInfo
#[no_mangle]
pub unsafe extern "C" fn SVC_BitUnPack(src: *const u8, dst: *mut u32, params: *const UnpackInfo) {
    // Lecture des paramètres depuis la structure pointée par r2
    let info = *params;
    let src_len = info.src_len as usize; // Nombre d'unités à lire
    let src_width = info.src_width as u32; // Largeur en bits (ex: 1, 2, 4, 8)
    let dest_width = info.dest_width as u32; // Largeur destination (step dans le mot 32 bits)
    
    // Le bit 31 de l'offset est un drapeau "Zero Data Flag"
    // Si Bit 31 = 1 : On n'ajoute pas l'offset si la donnée source est 0
    let zero_flag = (info.offset & 0x80000000) != 0;
    let offset_val = info.offset & 0x7FFFFFFF;

    let mut current_src = src;
    let mut current_dst = dst;
    
    // État de lecture bit-à-bit
    let mut bit_buffer: u8 = *current_src;
    current_src = current_src.add(1);
    let mut bits_available: u32 = 8;

    // Accumulateur pour l'écriture 32 bits
    let mut dest_acc: u32 = 0;
    let mut dest_bits_filled: u32 = 0;

    for _ in 0..src_len {
        // 1. Extraire 'src_width' bits de la source
        let mut val: u32 = 0;
        let mut bits_needed = src_width;

        while bits_needed > 0 {
            if bits_available == 0 {
                bit_buffer = *current_src;
                current_src = current_src.add(1);
                bits_available = 8;
            }

            let bits_to_take = if bits_available >= bits_needed { bits_needed } else { bits_available };
            
            // On prend les bits de poids fort du buffer restant
            let shift = bits_available - bits_to_take;
            let mask = (1 << bits_to_take) - 1;
            let chunk = (bit_buffer >> shift) & mask as u8;

            val = (val << bits_to_take) | (chunk as u32);
            
            bits_available -= bits_to_take;
            bits_needed -= bits_to_take;
        }

        // 2. Ajouter l'offset (Gestion du Zero Flag)
        if zero_flag && val == 0 {
            // Ne rien ajouter
        } else {
            val += offset_val;
        }

        // 3. Placer dans le mot de destination 32 bits
        dest_acc |= val << dest_bits_filled;
        dest_bits_filled += dest_width;

        // 4. Si le mot est plein (32 bits), écrire en mémoire
        if dest_bits_filled >= 32 {
            *current_dst = dest_acc;
            current_dst = current_dst.add(1);
            dest_acc = 0;
            dest_bits_filled = 0;
        }
    }
}

// Structure des paramètres passée dans r2
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnpackInfo {
    pub src_len: u16,    // Nombre d'unités sources
    pub src_width: u8,   // Largeur bits source (1..8)
    pub dest_width: u8,  // Largeur bits dest (1..32)
    pub offset: u32,     // Offset + Flag bit 31
}