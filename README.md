# debloat.rs

Opinionated tool to debloat windows.

> [!WARNING]
> This project was entirely vibe coded, and is based off of personal preference and some debloating steps/scripts that [talon](https://github.com/ravendevteam/talon) uses. Use at your own risk, or just don't.

## Using it

- Disable windows defender fully, especially realtime protection. Then add all of `C:\` as an exclusion.
- Run this in a powershell:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser; &([ScriptBlock]::Create((irm https://raw.githubusercontent.com/kk-spartans/debloat.rs/main/scripts/install.ps1))) -dotfiles -smallExecutable
```

### Install Script Options

The install script supports the following options:

- `-dotfiles` - Install and configure chezmoi dotfiles after debloating
- `-smallExecutable` - Use the 'optimized' build instead of 'release' (compiled with `opt-level = "z"`)
- `-v`, `-vv`, `-vvv` - Control verbosity (INFO, DEBUG, or TRACE level logging)
- `-noWallpaper` - Skip wallpaper download and setting
- `-noDarkMode` - Skip dark mode and transparency
- `-noTaskbarAutohide` - Skip taskbar autohide
- `-noEdgeRemoval` - Skip Microsoft Edge removal
- `-noOutlookOnedrive` - Skip Outlook/OneDrive uninstallation
- `-noBuiltinApps` - Skip built-in apps removal
- `-noRegistryTweaks` - Skip registry tweaks
- `-noWinutilTweaks` - Skip WinUtil tweaks
- `-noDebloatTweaks` - Skip debloat tweaks

#### Examples

```powershell
# Standard install with verbose logging, skip wallpaper
&([ScriptBlock]::Create((irm https://raw.githubusercontent.com/kk-spartans/debloat.rs/main/scripts/install.ps1))) -v -noWallpaper

# Optimized build with maximum verbosity, skip Edge and Office removal
&([ScriptBlock]::Create((irm https://raw.githubusercontent.com/kk-spartans/debloat.rs/main/scripts/install.ps1))) -smallExecutable -vvv -noEdgeRemoval -noOutlookOnedrive

# Just UI tweaks with debug logging
&([ScriptBlock]::Create((irm https://raw.githubusercontent.com/kk-spartans/debloat.rs/main/scripts/install.ps1))) -vv -noEdgeRemoval -noOutlookOnedrive -noBuiltinApps -noRegistryTweaks -noWinutilTweaks -noDebloatTweaks
```

### Advanced Usage

The executable supports various command-line flags to control logging verbosity and selectively disable features:

```powershell
# Run with increased verbosity (info level)
debloat.exe -v

# Run with maximum verbosity (trace level)
debloat.exe -vvv

# Skip specific features
debloat.exe --no-wallpaper --no-dark-mode

# Combine verbosity and feature flags
debloat.exe -vv --no-edge-removal --no-outlook-onedrive
```

For a complete list of available flags and usage examples, see [USAGE.md](./USAGE.md).

## Why rust?

- It has access to low-level windows APIs to change things like auto-hide taskbar without touching the registry manually and restarting explorer. It's almost as seamless as the settings dialogue.
- I don't know C++, even though I've heard it's more stable on windows. I barely even know rust.
