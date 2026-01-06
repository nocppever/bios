# build-FINAL.ps1 - Solution qui marche vraiment

Write-Host ""
Write-Host "=== Build BIOS ARM9 ===" -ForegroundColor Cyan
Write-Host ""

# 1. Compiler normalement
Write-Host "1. Compilation..." -ForegroundColor Yellow
cargo +nightly build --release -Z build-std=core

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERREUR!" -ForegroundColor Red
    exit 1
}
Write-Host "   OK" -ForegroundColor Green
Write-Host ""

# 2. Extraire l'archive et linker manuellement
$archive = "..\target\armv5te-none-eabi\release\libbios9.a"

if (-not (Test-Path $archive)) {
    Write-Host "ERREUR: Archive introuvable!" -ForegroundColor Red
    exit 1
}

Write-Host "2. Archive trouvee" -ForegroundColor Yellow
Write-Host "   $archive" -ForegroundColor Gray
Write-Host ""

# 3. Trouver les outils
$sysroot = rustc +nightly --print sysroot
$binDir = "$sysroot\lib\rustlib\x86_64-pc-windows-msvc\bin"
$ar = "$binDir\llvm-ar.exe"
$linker = "$binDir\rust-lld.exe"
$objcopy = "$binDir\llvm-objcopy.exe"

# 4. Extraire l'archive
Write-Host "3. Extraction de l'archive..." -ForegroundColor Yellow

$tmpDir = "temp_bios"
if (Test-Path $tmpDir) {
    Remove-Item -Recurse -Force $tmpDir
}
New-Item -ItemType Directory -Path $tmpDir | Out-Null

# Copier l'archive dans temp et extraire
Copy-Item $archive "$tmpDir\libbios9.a"
Push-Location $tmpDir
& $ar x libbios9.a
Pop-Location

$objects = Get-ChildItem $tmpDir -Filter "*.o"
Write-Host "   $($objects.Count) objets extraits" -ForegroundColor Green
Write-Host ""

# 5. Linker tous les objets
Write-Host "4. Linkage..." -ForegroundColor Yellow

$outDir = "..\output"
if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir | Out-Null
}

$elf = "$outDir\bios9.elf"

# Créer la liste des objets
$objectList = @()
foreach ($obj in $objects) {
    $objectList += "$tmpDir\$($obj.Name)"
}

# Linker avec le script
$ldArgs = @(
    "-flavor", "gnu",
    "-T", "arm9bios.ld",
    "-o", $elf
) + $objectList

& $linker $ldArgs

if ($LASTEXITCODE -ne 0) {
    Write-Host "   ERREUR linkage!" -ForegroundColor Red
    Remove-Item -Recurse -Force $tmpDir
    exit 1
}

Write-Host "   OK ELF cree" -ForegroundColor Green

# Nettoyer temp
Remove-Item -Recurse -Force $tmpDir
Write-Host ""

# 6. Extraire le binaire
Write-Host "5. Extraction binaire..." -ForegroundColor Yellow

$tmp = "$outDir\bios9.tmp"
& $objcopy -O binary $elf $tmp

if ($LASTEXITCODE -ne 0) {
    Write-Host "   ERREUR!" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path $tmp)) {
    Write-Host "   ERREUR: Fichier non cree!" -ForegroundColor Red
    exit 1
}

Write-Host "   OK" -ForegroundColor Green
Write-Host ""

# 7. Padder
Write-Host "6. Padding..." -ForegroundColor Yellow

$content = [System.IO.File]::ReadAllBytes($tmp)
$size = $content.Length

Write-Host "   Taille brute: $size bytes" -ForegroundColor Cyan

if ($size -eq 0) {
    Write-Host "   ERREUR: Vide!" -ForegroundColor Red
    exit 1
}

if ($size -gt 4096) {
    Write-Host "   ERREUR: Trop grand! ($size > 4096)" -ForegroundColor Red
    exit 1
}

$padded = $content + (,0x00 * (4096 - $size))
$final = "$outDir\bios9.bin"
[System.IO.File]::WriteAllBytes($final, $padded)

Remove-Item $tmp
Remove-Item $elf

Write-Host "   OK 4096 bytes" -ForegroundColor Green
Write-Host "   Code: $size bytes, Libre: $(4096 - $size) bytes" -ForegroundColor Gray
Write-Host ""

# 8. Header
$header = [System.IO.File]::ReadAllBytes($final) | Select-Object -First 16
$hex = ($header | ForEach-Object { $_.ToString("X2") }) -join " "
Write-Host "   Header: $hex" -ForegroundColor Gray
Write-Host ""

# 9. Installer
Write-Host "7. Installation..." -ForegroundColor Yellow

$biosDir = "..\core\bios"
if (-not (Test-Path $biosDir)) {
    New-Item -ItemType Directory -Path $biosDir -Force | Out-Null
}

Copy-Item $final "$biosDir\bios9.bin" -Force

Write-Host "   OK" -ForegroundColor Green
Write-Host ""

Write-Host "=== BUILD TERMINE ===" -ForegroundColor Green
Write-Host ""
Write-Host "Fichier cree: $biosDir\bios9.bin" -ForegroundColor Cyan
Write-Host "Taille: 4096 bytes (Code: $size, Libre: $(4096-$size))" -ForegroundColor Gray
Write-Host ""