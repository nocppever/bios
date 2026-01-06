# analyze-size.ps1 - Analyser pourquoi c'est trop gros

Write-Host ""
Write-Host "=== Analyse Taille BIOS ===" -ForegroundColor Cyan
Write-Host ""

# Compiler
Write-Host "Compilation..." -ForegroundColor Yellow
cargo +nightly build --release -Z build-std=core 2>&1 | Out-Null

$archive = "..\target\armv5te-none-eabi\release\libbios9.a"

if (-not (Test-Path $archive)) {
    Write-Host "Archive introuvable!" -ForegroundColor Red
    exit 1
}

# Extraire
$tmpDir = "temp_analyze"
if (Test-Path $tmpDir) {
    Remove-Item -Recurse -Force $tmpDir
}
New-Item -ItemType Directory -Path $tmpDir | Out-Null

$sysroot = rustc +nightly --print sysroot
$ar = "$sysroot\lib\rustlib\x86_64-pc-windows-msvc\bin\llvm-ar.exe"
$size = "$sysroot\lib\rustlib\x86_64-pc-windows-msvc\bin\llvm-size.exe"

Copy-Item $archive "$tmpDir\libbios9.a"
Push-Location $tmpDir
& $ar x libbios9.a
Pop-Location

# Analyser chaque objet
Write-Host "Taille des objets:" -ForegroundColor Cyan
Write-Host ""

$objects = Get-ChildItem $tmpDir -Filter "*.o"
$totalSize = 0

foreach ($obj in $objects | Sort-Object Length -Descending) {
    $objSize = $obj.Length
    $totalSize += $objSize
    
    $color = "Gray"
    if ($objSize -gt 10000) {
        $color = "Red"
    } elseif ($objSize -gt 5000) {
        $color = "Yellow"
    }
    
    Write-Host ("{0,8} bytes - {1}" -f $objSize, $obj.Name) -ForegroundColor $color
    
    # Afficher les symboles pour les gros fichiers
    if ($objSize -gt 5000) {
        Write-Host "  Symboles:" -ForegroundColor Gray
        & $size $obj.FullName 2>&1 | Select-Object -Skip 1 | ForEach-Object {
            Write-Host "    $_" -ForegroundColor DarkGray
        }
    }
}

Write-Host ""
Write-Host "Total: $totalSize bytes" -ForegroundColor Cyan
Write-Host "Cible: 4096 bytes" -ForegroundColor Yellow
Write-Host "Excès: $($totalSize - 4096) bytes" -ForegroundColor Red
Write-Host ""

# Nettoyer
Remove-Item -Recurse -Force $tmpDir

# Suggestions
Write-Host "=== Suggestions ===" -ForegroundColor Yellow
Write-Host ""

if ($totalSize -gt 50000) {
    Write-Host "PROBLEME MAJEUR: Code beaucoup trop gros!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Solutions:" -ForegroundColor Cyan
    Write-Host "  1. Retirer decompress.rs (LZ77 = ~5-10KB)" -ForegroundColor Gray
    Write-Host "  2. Retirer crc.rs si non utilise" -ForegroundColor Gray
    Write-Host "  3. Simplifier math.rs (division plus simple)" -ForegroundColor Gray
    Write-Host "  4. Retirer data.rs (tables)" -ForegroundColor Gray
    Write-Host "  5. Utiliser BIOS officiel ou FreeBIOS" -ForegroundColor Gray
} elseif ($totalSize -gt 10000) {
    Write-Host "Code trop gros mais optimisable" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Solutions:" -ForegroundColor Cyan
    Write-Host "  1. Activer --gc-sections dans rustflags" -ForegroundColor Gray
    Write-Host "  2. Utiliser opt-level='z'" -ForegroundColor Gray
    Write-Host "  3. Simplifier les fonctions" -ForegroundColor Gray
}

Write-Host ""