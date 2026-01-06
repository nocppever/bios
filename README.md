# BIOS Nintendo DS pour AvxDS

Ce dossier contient les implémentations des BIOS ARM7 et ARM9 pour l'émulateur AvxDS.

## 📁 Structure

```
bios/
├── Cargo.toml          # Workspace Rust
├── Makefile            # Build automation
├── arm7/               # ARM7 BIOS (OpenNitro)
│   ├── Cargo.toml
│   ├── src/
│   └── arm7bios.ld
├── arm9/               # ARM9 BIOS (notre implémentation)
│   ├── Cargo.toml
│   ├── src/
│   └── arm9bios.ld
└── output/             # Binaires compilés
    ├── bios7.bin       # 16KB
    └── bios9.bin       # 4KB
```

## 🛠️ Compilation

### Prérequis

```bash
# Installer Rust nightly
rustup toolchain install nightly

# Ajouter les targets ARM
rustup target add armv4t-none-eabi armv5te-none-eabi

# Installer rust-objcopy
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### Build

```bash
# Tout compiler
make all

# Seulement ARM7
make arm7

# Seulement ARM9
make arm9

# Installer dans l'émulateur
make install
```

## 📊 Tailles des BIOS

- **ARM7**: 16 KB (16,384 bytes) à 0x00000000
- **ARM9**: 4 KB (4,096 bytes) à 0xFFFF0000

## ✅ Fonctionnalités

### ARM7 BIOS (OpenNitro)
- Exception handlers
- SWI functions
- Sound, wireless, touch screen
- Power management

### ARM9 BIOS (AvxDS)
- ✅ Reset handler avec CP15 init
- ✅ SWI dispatcher complet
- ✅ IRQ handler
- ✅ Math: Div, Sqrt
- ✅ Memory: CpuSet, CpuFastSet
- ✅ Wait: IntrWait, VBlankIntrWait
- ⏳ Decompression (TODO)
- ⏳ Audio tables (TODO)

## 🔧 Intégration dans l'émulateur

### 1. Charger les BIOS

Dans `core/src/nds.rs`:

```rust
pub struct NDS {
    pub bios7: Vec<u8>,
    pub bios9: Vec<u8>,
    // ...
}

impl NDS {
    pub fn new() -> Self {
        let bios7 = std::fs::read("bios/output/bios7.bin")
            .expect("bios7.bin not found");
        let bios9 = std::fs::read("bios/output/bios9.bin")
            .expect("bios9.bin not found");
        
        assert_eq!(bios7.len(), 16384);
        assert_eq!(bios9.len(), 4096);
        
        // ...
    }
}
```

### 2. Mapper en mémoire

**ARM7 BIOS** à `0x00000000 - 0x00003FFF`:

```rust
// Dans arm7_bus.rs
0x00 => {
    if addr < 0x4000 {
        return self.bios7[addr as usize];
    }
    // ...
}
```

**ARM9 BIOS** à `0xFFFF0000 - 0xFFFF0FFF`:

```rust
// Dans arm9_bus.rs
0xFF => {
    if addr >= 0xFFFF0000 && addr < 0xFFFF1000 {
        let offset = (addr - 0xFFFF0000) as usize;
        return self.bios9[offset];
    }
    // ...
}
```

### 3. Démarrer depuis le BIOS

```rust
pub fn load_rom(&mut self, path: &Path) -> Result<(), String> {
    // Charger la ROM normalement
    // ...
    
    // NE PAS appeler setup_post_bios() !
    // À la place, démarrer au reset vector
    self.arm9.regs[15] = 0xFFFF0000;  // ARM9 reset
    self.arm7.regs[15] = 0x00000000;  // ARM7 reset
    
    Ok(())
}
```

## 🐛 Debugging

### Logs utiles

```rust
// Logger l'exécution du BIOS
if pc >= 0xFFFF0000 && pc < 0xFFFF1000 {
    println!("[ARM9 BIOS] PC=0x{:08X}", pc);
}

if pc < 0x4000 {
    println!("[ARM7 BIOS] PC=0x{:08X}", pc);
}
```

### Vérifier les SWI

```rust
fn handle_swi(&mut self, num: u8) {
    println!("[SWI 0x{:02X}] called", num);
    // ...
}
```

## 📚 Ressources

- [OpenNitro ARM7](https://github.com/OpenNitro-Project/opennitro-arm7)
- [GBATEK](https://problemkaputt.de/gbatek.htm)
- [libnds](https://github.com/devkitPro/libnds)

## 🤝 Développement

### Ajouter une fonction SWI

1. Éditer `arm9/src/swi.rs`
2. Ajouter le case dans `swi_dispatch()`
3. Implémenter la fonction
4. Recompiler: `make arm9`

### Tests

```bash
# Compiler en mode debug
cd arm9 && cargo build --target armv5te-none-eabi

# Vérifier la taille
ls -lh target/armv5te-none-eabi/debug/
```

## 📝 TODO

- [ ] Implémenter décompression LZ77
- [ ] Implémenter décompression Huffman
- [ ] Implémenter décompression RLE
- [ ] Ajouter tables audio (sine, pitch, volume)
- [ ] Implémenter CRC16
- [ ] Tests unitaires
- [ ] Documentation complète des SWI

---

**AvxDS Team** - Émulateur Nintendo DS open-source
