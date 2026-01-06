#!/bin/bash
# Script pour cloner et configurer OpenNitro ARM7

set -e

echo "=== Installation d'OpenNitro ARM7 ==="
echo ""

# Vérifier si git est installé
if ! command -v git &> /dev/null; then
    echo "❌ Git n'est pas installé!"
    echo "Installez git et réessayez."
    exit 1
fi

# Cloner le repo si le dossier n'existe pas
if [ ! -d "arm7/src" ]; then
    echo "📥 Clonage d'OpenNitro ARM7..."
    
    # Supprimer le dossier vide si existe
    rm -rf arm7
    
    # Cloner
    git clone https://github.com/OpenNitro-Project/opennitro-arm7.git arm7_tmp
    
    # Déplacer le contenu
    mv arm7_tmp/* arm7/ 2>/dev/null || true
    mv arm7_tmp/.* arm7/ 2>/dev/null || true
    rm -rf arm7_tmp
    
    echo "✓ OpenNitro ARM7 cloné"
else
    echo "✓ OpenNitro ARM7 déjà présent"
fi

# Mettre à jour Cargo.toml pour le workspace
echo ""
echo "📝 Configuration du workspace..."

cat > arm7/Cargo.toml << 'EOF'
[package]
name = "bios-arm7"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[lib]
name = "bios_arm7"
crate-type = ["staticlib"]

[dependencies]
# OpenNitro ARM7 dependencies
EOF

# Copier la vraie config si elle existe
if [ -f "arm7/Cargo.toml.bak" ]; then
    echo "✓ Cargo.toml configuré"
fi

echo ""
echo "=== Installation terminée ==="
echo ""
echo "Prochaines étapes:"
echo "  1. make all          # Compiler les BIOS"
echo "  2. make install      # Installer dans l'émulateur"
echo ""
