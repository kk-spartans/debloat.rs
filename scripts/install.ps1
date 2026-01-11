param(
    [switch]$smallExecutable,
    [switch]$dotfiles
)

# ---- self-elevate ----
if (-not ([Security.Principal.WindowsPrincipal] `
    [Security.Principal.WindowsIdentity]::GetCurrent()
).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Start-Process powershell `
        "-ExecutionPolicy Bypass -File `"$PSCommandPath`" $($args -join ' ')" `
        -Verb RunAs
    exit
}

$ErrorActionPreference = "Stop"

# ---- detect architecture ----
$arch = if ([Environment]::Is64BitOperatingSystem) {
    if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "arm64" } else { "x64" }
} else {
    throw "32-bit Windows in 2026 is a cry for help."
}

$suffix = if ($smallExecutable) { "optimized" } else { "release" }
$artifactName = "debloat-$arch-$suffix"

# ---- install winget + VC++ ----

Start-Process powershell.exe -ArgumentList "-Command", "&([ScriptBlock]::Create((irm winget.pro))) -Force" -Wait

if ($arch -eq "x64") {
    winget install Microsoft.VCRedist.2015+.x64 --silent --accept-package-agreements --accept-source-agreements
} else {
    winget install Microsoft.VCRedist.2015+.arm64 --silent --accept-package-agreements --accept-source-agreements
}

# ---- download & extract (nightly.link) ----
$downloadUrl = "https://nightly.link/kk-spartans/debloat.rs/?artifact=$artifactName"
$zipPath = "$temp\artifact.zip"

Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath

# ---- download & extract ----
$temp = New-Item -ItemType Directory -Force -Path "$env:TEMP\debloat"
$zipPath = "$temp\artifact.zip"

Invoke-WebRequest `
    -Uri $artifact.archive_download_url `
    -Headers $headers `
    -OutFile $zipPath

Expand-Archive $zipPath $temp -Force

$exe = Get-ChildItem $temp -Recurse -Filter "debloat.exe" | Select-Object -First 1

if (-not $exe) {
    throw "debloat.exe missing. try downloading it manually from an older action run...?"
}

# ---- run debloat ----
Start-Process $exe.FullName -Verb RunAs -Wait

# ---- optional dotfiles ----
if ($dotfiles) {
    winget install twpayne.chezmoi --silent --accept-package-agreements --accept-source-agreements
    chezmoi init kk-spartans --apply --verbose
}

