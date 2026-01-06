// arm9/src/data.rs

// Le BIOS DS contient une table de sinus à l'adresse relative +0 in data section?
// En réalité, ces tables sont souvent en fin de BIOS.

// On force cette table à être gardée par le linker
#[used]
#[link_section = ".rodata.sine_table"]
#[no_mangle]
pub static SINE_TABLE: [i16; 1024] = {
    let mut table = [0; 1024];
    // Remplissage simulé pour prendre de la place
    // (Dans un vrai émulateur, on copierait les valeurs hexadécimales exactes)
    let mut i = 0;
    while i < 1024 {
        table[i] = (i as i16).wrapping_mul(10); 
        i += 1;
    }
    table
};

#[used]
#[link_section = ".rodata.pitch_table"]
#[no_mangle]
pub static PITCH_TABLE: [u16; 768] = [0; 768]; // Table de hauteur de son (inutilisée mais présente)