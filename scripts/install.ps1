<#
.SYNOPSIS
    Downloads and runs the debloat.rs Windows debloating tool.

.DESCRIPTION
    This script downloads the latest release of debloat.exe from GitHub and runs it with 
    elevated privileges. It supports various flags to control the tool's behavior, including
    verbosity levels and selective feature disabling.

.PARAMETER smallExecutable
    Use the 'optimized' build (compiled with opt-level="z") instead of the standard 'release' build.

.PARAMETER dotfiles
    Install and configure chezmoi dotfiles after running debloat.

.PARAMETER debug
    Pause before exit to see output (passes --debug to debloat.exe).

.PARAMETER v
    Enable INFO level logging in debloat.exe (shows progress messages).

.PARAMETER vv
    Enable DEBUG level logging in debloat.exe (shows detailed debug information).

.PARAMETER vvv
    Enable TRACE level logging in debloat.exe (shows maximum verbosity).

.PARAMETER noWallpaper
    Skip wallpaper download and setting.

.PARAMETER noDarkMode
    Skip enabling dark mode and transparency.

.PARAMETER noTaskbarAutohide
    Skip setting taskbar to autohide.

.PARAMETER noEdgeRemoval
    Skip Microsoft Edge removal.

.PARAMETER noOutlookOnedrive
    Skip Outlook and OneDrive uninstallation.

.PARAMETER noBuiltinApps
    Skip built-in apps removal.

.PARAMETER noRegistryTweaks
    Skip registry tweaks.

.PARAMETER noPrivacyTweaks
    Skip privacy and system tweaks.

.PARAMETER noDebloatTweaks
    Skip debloat tweaks.

.EXAMPLE
    .\install.ps1
    Run with default settings (standard build, WARN level logging, all features enabled).

.EXAMPLE
    .\install.ps1 -smallExecutable -dotfiles
    Use the optimized build and install dotfiles.

.EXAMPLE
    .\install.ps1 -v -noWallpaper -noDarkMode
    Run with INFO logging, skipping wallpaper and dark mode.

.EXAMPLE
    .\install.ps1 -vvv -noEdgeRemoval -noOutlookOnedrive
    Run with maximum logging verbosity, skipping Edge and Outlook/OneDrive removal.

.NOTES
    This script requires administrator privileges to run debloat.exe.
    Internet connection is required to download the executable.
#>

param(
    [switch]$smallExecutable,
    [switch]$dotfiles,
    [switch]$debug,
    # Verbosity flags
    [switch]$v,
    [switch]$vv,
    [switch]$vvv,
    # Feature disable flags
    [switch]$noWallpaper,
    [switch]$noDarkMode,
    [switch]$noTaskbarAutohide,
    [switch]$noEdgeRemoval,
    [switch]$noOutlookOnedrive,
    [switch]$noBuiltinApps,
    [switch]$noRegistryTweaks,
    [switch]$noPrivacyTweaks,
    [switch]$noDebloatTweaks
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
    winget install Microsoft.VCRedist.2015+.x64 --silent --accept-package-agreements --accept-source-agreements --source winget
} else {
    winget install Microsoft.VCRedist.2015+.arm64 --silent --accept-package-agreements --accept-source-agreements --source winget
}

# ---- download exe from latest release ----
$temp = New-Item -ItemType Directory -Force -Path "$env:TEMP\debloat"
$exePath = "$env:TEMP\$exeName"

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
# Build argument list for debloat.exe
$debloatArgs = @()

# Add debug flag
if ($debug) { $debloatArgs += "--debug" }

# Add verbosity flags
if ($vvv) {
    $debloatArgs += "-vvv"
} elseif ($vv) {
    $debloatArgs += "-vv"
} elseif ($v) {
    $debloatArgs += "-v"
}

# Add feature disable flags
if ($noWallpaper) { $debloatArgs += "--no-wallpaper" }
if ($noDarkMode) { $debloatArgs += "--no-dark-mode" }
if ($noTaskbarAutohide) { $debloatArgs += "--no-taskbar-autohide" }
if ($noEdgeRemoval) { $debloatArgs += "--no-edge-removal" }
if ($noOutlookOnedrive) { $debloatArgs += "--no-outlook-onedrive" }
if ($noBuiltinApps) { $debloatArgs += "--no-builtin-apps" }
if ($noRegistryTweaks) { $debloatArgs += "--no-registry-tweaks" }
if ($noPrivacyTweaks) { $debloatArgs += "--no-privacy-tweaks" }
if ($noDebloatTweaks) { $debloatArgs += "--no-debloat-tweaks" }

# Start debloat.exe directly with elevated privileges and wait for completion
if ($debloatArgs.Count -gt 0) {
    Start-Process $exePath -ArgumentList $debloatArgs -Verb RunAs -Wait
} else {
    Start-Process $exePath -Verb RunAs -Wait
}

# ---- optional dotfiles ----
if ($dotfiles) {
    winget install twpayne.chezmoi --silent --accept-package-agreements --accept-source-agreements --scope user --source winget
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    chezmoi init kk-spartans --apply --verbose
}

