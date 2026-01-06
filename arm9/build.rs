// build.rs - Compatible avec build-std

fn main() {
    // Indiquer le chemin du linker script
    println!("cargo:rustc-link-arg=-Tarm9bios.ld");
    
    // Recompiler si le linker script change
    println!("cargo:rerun-if-changed=arm9bios.ld");
}