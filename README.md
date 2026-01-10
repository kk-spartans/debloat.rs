# debloat.rs

Opinionated tool to debloat windows.

> [!WARNING]
> This project was entirely vibe coded, and is based off of personal preference and some debloating steps/scripts that [talon](https://github.com/ravendevteam/talon) uses. Use at your own risk, or just don't.

## Using it

- Disable windows defender fully, especially realtime protection. Then add all of `C:\` as an exclusion.
- Run:

```
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser; &([ScriptBlock]::Create((https://raw.githubusercontent.com/kk-spartans/debloat.rs/main/scripts/install.ps1))) -smallExecutable -dotfiles
```

- remove `-dotfiles` to not set up chezmoi
- remove -smallExecutable to use the `release` executable instead of `optimized` (compiled with `opt-level = "z"`).

## Why rust?

- It has access to low-level windows APIs to change things like auto-hide taskbar without touching the registry manually and restarting explorer. It's almost as seamless as the settings dialogue.
- I don't know C++, even though I've heard it's more stable on windows. I barely even know rust.
