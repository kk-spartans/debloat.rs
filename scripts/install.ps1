param(
    [switch]$smallExecutable,
    [switch]$dotfiles
)

$ErrorActionPreference = "Stop"

# ---- detect architecture ----
$arch = if ([Environment]::Is64BitOperatingSystem) {
    if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "arm64" } else { "x64" }
} else {
    throw "32-bit Windows in 2026 is a cry for help."
}

$suffix = if ($smallExecutable) { "optimized" } else { "release" }
$exeName = "debloat-$arch-$suffix.exe"

# ---- install winget + VC++ ----

Start-Process powershell.exe -ArgumentList "-Command", "&([ScriptBlock]::Create((irm winget.pro))) -Force" -Verb RunAs -Wait

if ($arch -eq "x64") {
    winget install Microsoft.VCRedist.2015+.x64 --silent --accept-package-agreements --accept-source-agreements
} else {
    winget install Microsoft.VCRedist.2015+.arm64 --silent --accept-package-agreements --accept-source-agreements
}

# ---- download exe from latest release ----
$temp = New-Item -ItemType Directory -Force -Path "$env:TEMP\debloat"
$exePath = "$temp\$exeName"

try {
    $apiUrl = "https://api.github.com/repos/kk-spartans/debloat.rs/releases?per_page=10"
    $releases = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "PowerShell" }
    $latestRelease = $releases | Select-Object -First 1

    if (-not $latestRelease) {
        throw "No release found. Report an issue."
    }

    $asset = $latestRelease.assets | Where-Object { $_.name -eq $exeName } | Select-Object -First 1

    if (-not $asset) {
        throw "Asset $exeName not found in latest release. What arch are you using?"
    }

    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $exePath

    if (-not (Test-Path $exePath)) {
        throw "Failed to download $exeName"
    }
} catch {
    throw "Failed to fetch release from GitHub API: $($_.Exception.Message)"
}

# ---- run debloat ----
Start-Process $exePath -Verb RunAs -Wait

# ---- optional dotfiles ----
if ($dotfiles) {
    winget install twpayne.chezmoi --silent --accept-package-agreements --accept-source-agreements --scope user
    chezmoi init kk-spartans --apply --verbose
}

