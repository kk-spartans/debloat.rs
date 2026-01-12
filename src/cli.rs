use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "debloat")]
#[command(about = "Opinionated Windows debloating tool", long_about = None)]
pub struct Cli {
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(long = "no-wallpaper", help = "Skip wallpaper download and setting")]
    pub no_wallpaper: bool,

    #[arg(long = "no-dark-mode", help = "Skip enabling dark mode")]
    pub no_dark_mode: bool,

    #[arg(long = "no-taskbar-autohide", help = "Skip setting taskbar to autohide")]
    pub no_taskbar_autohide: bool,

    #[arg(long = "no-edge-removal", help = "Skip Microsoft Edge removal")]
    pub no_edge_removal: bool,

    #[arg(
        long = "no-outlook-onedrive",
        help = "Skip Outlook and OneDrive uninstallation"
    )]
    pub no_outlook_onedrive: bool,

    #[arg(long = "no-builtin-apps", help = "Skip built-in apps removal")]
    pub no_builtin_apps: bool,

    #[arg(long = "no-registry-tweaks", help = "Skip registry tweaks")]
    pub no_registry_tweaks: bool,

    #[arg(long = "no-privacy-tweaks", help = "Skip privacy and system tweaks")]
    pub no_privacy_tweaks: bool,

    #[arg(long = "no-debloat-tweaks", help = "Skip debloat tweaks")]
    pub no_debloat_tweaks: bool,
}

impl Cli {
    pub fn log_level(&self) -> tracing::Level {
        match self.verbose {
            0 => tracing::Level::WARN,
            1 => tracing::Level::INFO,
            2 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        }
    }
}
