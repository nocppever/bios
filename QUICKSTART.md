# 🚀 Guide de démarrage rapide

## Installation

```bash
# 1. Copier ce dossier dans votre projet
mv bios_workspace /chemin/vers/avxds_emulator/bios

# 2. Installer OpenNitro ARM7
cd /chemin/vers/avxds_emulator/bios
./setup_arm7.sh

# 3. Installer les outils Rust
rustup toolchain install nightly
rustup target add armv4t-none-eabi armv5te-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview

# 4. Compiler les BIOS
make all

# 5. Installer dans l'émulateur
make install
```

## Résultat

Vous devriez avoir :
```
bios/output/bios7.bin  (16 KB)
bios/output/bios9.bin  (4 KB)
```

Et dans l'émulateur :
```
core/bios/bios7.bin
core/bios/bios9.bin
```

## Prochaines étapes

Suivez `INTEGRATION.md` pour intégrer les BIOS dans le code de l'émulateur.

## Problèmes courants

### "rust-objcopy not found"
```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### "target 'armv5te-none-eabi' not found"
```bash
rustup target add armv5te-none-eabi --toolchain nightly
```

### Erreur de compilation ARM7
Vérifiez que le dépôt est bien cloné :
```bash
ls arm7/src/
```

## Support

Consultez `README.md` pour plus de détails.
