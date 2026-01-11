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

if ($arch -eq "x64") {
    winget install Microsoft.VCRedist.2015+.x64 --silent --accept-package-agreements --accept-source-agreements
} else {
    winget install Microsoft.VCRedist.2015+.arm64 --silent --accept-package-agreements --accept-source-agreements
}

# ---- GitHub API ----
$repo = "kk-spartans/debloat.rs"
$headers = @{
    "Accept" = "application/vnd.github+json"
    "User-Agent" = "powershell"
}

$run = Invoke-RestMethod `
    "https://api.github.com/repos/$repo/actions/runs?per_page=1" `
    -Headers $headers

$runId = $run.workflow_runs[0].id

$artifacts = Invoke-RestMethod `
    "https://api.github.com/repos/$repo/actions/runs/$runId/artifacts" `
    -Headers $headers

$artifact = $artifacts.artifacts | Where-Object { $_.name -eq $artifactName }

if (-not $artifact) {
    throw "Artifact '$artifactName' not found. Someone broke CI."
}

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

