//! CRC Functions - Implementation of SWI 0x0E

#[no_mangle]
pub unsafe extern "C" fn swi_crc16_impl(mut crc: u16, data: *const u8, mut len: u32) -> u16 {
    // Le polynôme utilisé est 0x8005.
    // Comme on traite les bits de poids faible en premier (LSB first),
    // on utilise la représentation inversée : 0xA001.
    const POLY: u16 = 0xA001;
    
    let mut ptr = data;
    
    while len > 0 {
        // On combine le byte courant avec le CRC
        crc ^= *ptr as u16;
        
        // On traite les 8 bits du byte
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ POLY;
            } else {
                crc >>= 1;
            }
        }
        
        ptr = ptr.add(1);
        len -= 1;
    }
    
    crc
}