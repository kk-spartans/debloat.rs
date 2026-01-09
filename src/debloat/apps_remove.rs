use std::process::{Command, Stdio};

pub fn remove_built_in_apps() {
    println!("[*] Removing built-in apps...");

    let apps = vec![
        "Microsoft.Clipchamp",
        "MicrosoftTeams",
        "Microsoft.Todo",
        "Microsoft.Getstarted",
    ];

    for app in apps {
        println!("Removing {app}...");
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &format!(
                    "Get-AppxPackage {app} | Remove-AppxPackage -ErrorAction SilentlyContinue"
                ),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();
    }

    println!("[+] Built-in apps removed");
}
