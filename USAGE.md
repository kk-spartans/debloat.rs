# Usage Guide

## Command Line Flags

### Verbosity Levels

Control logging output with the `-v` flag:

- **Default (no flag)**: `WARN` level - only warnings and errors
- **`-v`**: `INFO` level - informational messages about progress
- **`-vv`**: `DEBUG` level - detailed debug information
- **`-vvv`** or more: `TRACE` level - maximum verbosity with all trace information

Examples:
```powershell
# Run with default verbosity (warnings only)
debloat.exe

# Run with info level logging
debloat.exe -v

# Run with debug level logging
debloat.exe -vv

# Run with trace level logging
debloat.exe -vvv
```

### Feature Flags

Selectively disable specific operations using the following flags:

#### UI and Appearance
- `--no-wallpaper` - Skip downloading and setting the wallpaper
- `--no-dark-mode` - Skip enabling dark mode and transparency
- `--no-taskbar-autohide` - Skip setting taskbar to autohide

#### System Modifications
- `--no-edge-removal` - Skip Microsoft Edge removal
- `--no-outlook-onedrive` - Skip Outlook and OneDrive uninstallation
- `--no-builtin-apps` - Skip built-in apps removal
- `--no-registry-tweaks` - Skip registry tweaks
- `--no-privacy-tweaks` - Skip privacy and system tweaks
- `--no-debloat-tweaks` - Skip debloat tweaks

### Examples

```powershell
# Run with info logging, but skip wallpaper and dark mode
debloat.exe -v --no-wallpaper --no-dark-mode

# Run with debug logging and skip Edge removal
debloat.exe -vv --no-edge-removal

# Run with default logging, only apply UI tweaks
debloat.exe --no-edge-removal --no-outlook-onedrive --no-builtin-apps --no-registry-tweaks --no-privacy-tweaks --no-debloat-tweaks

# Maximum verbosity with custom feature selection
debloat.exe -vvv --no-wallpaper --no-taskbar-autohide --no-outlook-onedrive
```

### Getting Help

To see all available options:
```powershell
debloat.exe --help
```
