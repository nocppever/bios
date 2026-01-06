# Makefile pour compiler les BIOS ARM7 et ARM9
# Usage: make all

.PHONY: all clean arm7 arm9 install

# Outils
OBJCOPY := rust-objcopy
CARGO := cargo

# Sorties
OUTPUT_DIR := output
BIOS7_BIN := $(OUTPUT_DIR)/bios7.bin
BIOS9_BIN := $(OUTPUT_DIR)/bios9.bin

# Targets ARM
ARM7_TARGET := armv4t-none-eabi
ARM9_TARGET := armv5te-none-eabi

all: arm7 arm9
	@echo "✓ Tous les BIOS compilés avec succès!"
	@ls -lh $(OUTPUT_DIR)/*.bin

# Créer le dossier de sortie
$(OUTPUT_DIR):
	@mkdir -p $(OUTPUT_DIR)

# Compiler ARM7
arm7: $(OUTPUT_DIR)
	@echo "=== Compilation ARM7 BIOS ==="
	cd arm7 && $(CARGO) build --release --target $(ARM7_TARGET)
	@echo "Extraction du binaire ARM7..."
	$(OBJCOPY) -O binary arm7/target/$(ARM7_TARGET)/release/libbios_arm7.a $(BIOS7_BIN).tmp || \
	$(OBJCOPY) -O binary arm7/target/$(ARM7_TARGET)/release/bios_arm7 $(BIOS7_BIN).tmp
	@echo "Padding à 16KB..."
	dd if=/dev/zero of=$(BIOS7_BIN) bs=16384 count=1 2>/dev/null
	dd if=$(BIOS7_BIN).tmp of=$(BIOS7_BIN) conv=notrunc 2>/dev/null
	@rm -f $(BIOS7_BIN).tmp
	@SIZE=$$(stat -f%z $(BIOS7_BIN) 2>/dev/null || stat -c%s $(BIOS7_BIN)); \
	echo "✓ ARM7 BIOS: $$SIZE bytes (16KB)"

# Compiler ARM9  
arm9: $(OUTPUT_DIR)
	@echo "=== Compilation ARM9 BIOS ==="
	cd arm9 && $(CARGO) build --release --target $(ARM9_TARGET)
	@echo "Extraction du binaire ARM9..."
	$(OBJCOPY) -O binary arm9/target/$(ARM9_TARGET)/release/libbios_arm9.a $(BIOS9_BIN).tmp || \
	$(OBJCOPY) -O binary arm9/target/$(ARM9_TARGET)/release/bios_arm9 $(BIOS9_BIN).tmp
	@echo "Padding à 4KB..."
	dd if=/dev/zero of=$(BIOS9_BIN) bs=4096 count=1 2>/dev/null
	dd if=$(BIOS9_BIN).tmp of=$(BIOS9_BIN) conv=notrunc 2>/dev/null
	@rm -f $(BIOS9_BIN).tmp
	@SIZE=$$(stat -f%z $(BIOS9_BIN) 2>/dev/null || stat -c%s $(BIOS9_BIN)); \
	echo "✓ ARM9 BIOS: $$SIZE bytes (4KB)"

# Nettoyer
clean:
	@echo "Nettoyage..."
	cd arm7 && $(CARGO) clean
	cd arm9 && $(CARGO) clean
	rm -rf $(OUTPUT_DIR)
	@echo "✓ Nettoyé"

# Installer dans l'émulateur
install: all
	@echo "Installation dans ../core/bios/..."
	@mkdir -p ../core/bios
	cp $(BIOS7_BIN) ../core/bios/
	cp $(BIOS9_BIN) ../core/bios/
	@echo "✓ BIOS installés!"

# Vérifier les checksums
verify:
	@echo "Checksums:"
	@sha256sum $(OUTPUT_DIR)/*.bin

# Aide
help:
	@echo "Makefile BIOS AvxDS"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Compiler ARM7 et ARM9"
	@echo "  arm7     - Compiler seulement ARM7"
	@echo "  arm9     - Compiler seulement ARM9"
	@echo "  clean    - Nettoyer les builds"
	@echo "  install  - Installer dans ../core/bios/"
	@echo "  verify   - Vérifier les checksums"
	@echo "  help     - Afficher cette aide"
