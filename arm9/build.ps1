# build-FINAL-V2.ps1 - Correction des chemins absolus pour .NET

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

# 2. Extraire l'archive
$archive = "..\target\armv5te-none-eabi\release\libbios9.a"

if (-not (Test-Path $archive)) {
    Write-Host "ERREUR: Archive introuvable!" -ForegroundColor Red
    exit 1
}

Write-Host "2. Archive trouvee" -ForegroundColor Yellow
Write-Host "   $archive" -ForegroundColor Gray
Write-Host ""

# 3. Outils
$sysroot = rustc +nightly --print sysroot
$binDir = "$sysroot\lib\rustlib\x86_64-pc-windows-msvc\bin"
$ar = "$binDir\llvm-ar.exe"
$linker = "$binDir\rust-lld.exe"
$objcopy = "$binDir\llvm-objcopy.exe"

# 4. Extraction
Write-Host "3. Extraction de l'archive..." -ForegroundColor Yellow

$tmpDir = "temp_bios"
if (Test-Path $tmpDir) {
    Remove-Item -Recurse -Force $tmpDir
}
New-Item -ItemType Directory -Path $tmpDir | Out-Null

Copy-Item $archive "$tmpDir\libbios9.a"
Push-Location $tmpDir
& $ar x libbios9.a
Pop-Location

$objects = Get-ChildItem $tmpDir -Filter "*.o"
Write-Host "   $($objects.Count) objets extraits" -ForegroundColor Green
Write-Host ""

# 5. Linkage
Write-Host "4. Linkage..." -ForegroundColor Yellow

$outDirRel = "..\output"
if (-not (Test-Path $outDirRel)) {
    New-Item -ItemType Directory -Path $outDirRel | Out-Null
}
# --- CORRECTION CRITIQUE : Conversion en chemin absolu ---
$outDir = (Resolve-Path $outDirRel).Path 

$elf = "$outDir\bios9.elf"

$objectList = @()
foreach ($obj in $objects) {
    $objectList += "$tmpDir\$($obj.Name)"
}

$ldArgs = @(
    "-flavor", "gnu",
    "-T", "arm9bios.ld",
    "--gc-sections",
    "-o", $elf
) + $objectList

& $linker $ldArgs

if ($LASTEXITCODE -ne 0) {
    Write-Host "   ERREUR linkage!" -ForegroundColor Red
    Remove-Item -Recurse -Force $tmpDir
    exit 1
}

Write-Host "   OK ELF cree" -ForegroundColor Green

Remove-Item -Recurse -Force $tmpDir
Write-Host ""

# 6. Extraction binaire
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

# 7. Padding
Write-Host "6. Padding..." -ForegroundColor Yellow

# Maintenant $tmp est un chemin absolu, donc .NET le trouvera
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

# 8. Verification
$header = [System.IO.File]::ReadAllBytes($final) | Select-Object -First 16
$hex = ($header | ForEach-Object { $_.ToString("X2") }) -join " "
Write-Host "   Header: $hex" -ForegroundColor Gray
Write-Host ""

# 9. Installation
Write-Host "7. Installation..." -ForegroundColor Yellow

$biosDir = "..\core\bios"
if (-not (Test-Path $biosDir)) {
    New-Item -ItemType Directory -Path $biosDir -Force | Out-Null
}

Copy-Item $final "$biosDir\bios9.bin" -Force

Write-Host "   OK" -ForegroundColor Green
Write-Host ""
Write-Host "=== SUCCES ===" -ForegroundColor Green