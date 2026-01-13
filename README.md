# debloat.rs

Opinionated tool to debloat windows.

> [!WARNING]
> This project was entirely vibe coded, and is based off of personal preference and some debloating steps/scripts that [talon](https://github.com/ravendevteam/talon) uses. Use at your own risk, or just don't.

## Using it

- Disable windows defender fully, especially realtime protection. Then add all of `C:\` as an exclusion.
- Run this in a powershell:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser; &([ScriptBlock]::Create((irm https://raw.githubusercontent.com/kk-spartans/debloat.rs/main/scripts/install.ps1))) -dotfiles -smallExecutable -debug
```


## Why rust?

- It has access to low-level windows APIs to change things like auto-hide taskbar without touching the registry manually and restarting explorer. It's almost as seamless as the settings dialogue.
- I don't know C++, even though I've heard it's more stable on windows. I barely even know rust.

## Credits

Thanks to:

- [talon](https://github.com/ravendevteam/talon)
- [Chris Titus Tech's WinUtil](https://github.com/ChrisTitusTech/winutil)
- [Raphire's Win11Debloat](https://github.com/Raphire/Win11Debloat)

for helping me understand what parts of windows need  debloating.