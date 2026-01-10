# debloat.rs

Opinionated tool to debloat windows.

> [!WARNING]
> This project was entirely vibe coded using opencode, and is based off of personal preference and some debloating steps/scripts that [talon](https://github.com/ravendevteam/talon) uses. Use at your own risk, or just don't.

## Using it

- Use `irm winget.pro | iex` in an admin powershell just to make sure winget is installed.
- Run `winget install Microsoft.VCRedist.2015+.x64` (or `winget install Microsoft.VCRedist.2015+.arm64` on Snapdragon machines.)
- Run the executable from the github build artifacts.

## Why rust?

- It has access to low-level windows APIs to change things like auto-hide taskbar without touching the registry manually and restarting explorer. It's almost as seamless as the settings dialogue.
- I don't know C++, even though I've heard it's more stable on windows. I barely even know rust.
