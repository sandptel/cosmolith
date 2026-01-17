use clap::{Parser, ValueHint, builder::PossibleValuesParser};

use crate::debug;

pub const DEFAULT_NAMESPACES: &[&str] = &[
    "com.system76.CosmicAppletAudio",
    "com.system76.CosmicAppletTime",
    "com.system76.CosmicAppLibrary",
    "com.system76.CosmicAppList",
    "com.system76.CosmicAudio",
    "com.system76.CosmicBackground",
    "com.system76.CosmicComp",
    "com.system76.CosmicEdit",
    "com.system76.CosmicFiles",
    "com.system76.CosmicIdle",
    "com.system76.CosmicNotifications",
    "com.system76.CosmicPanel",
    "com.system76.CosmicPanel.Dock",
    "com.system76.CosmicPanel.Panel",
    "com.system76.CosmicPanelButton",
    "com.system76.CosmicPlayer",
    "com.system76.CosmicPortal",
    "com.system76.CosmicSettings",
    "com.system76.CosmicSettings.Shortcuts",
    "com.system76.CosmicSettings.Wallpaper",
    "com.system76.CosmicSettings.WindowRules",
    "com.system76.CosmicSettingsDaemon",
    "com.system76.CosmicTerm",
    "com.system76.CosmicTheme.Dark",
    "com.system76.CosmicTheme.Dark.Builder",
    "com.system76.CosmicTheme.Light",
    "com.system76.CosmicTheme.Light.Builder",
    "com.system76.CosmicTheme.Mode",
    "com.system76.CosmicTk",
    "com.system76.CosmicWorkspaces",
];

/// Command-line interface for cosmolith.
#[derive(Debug, Parser)]
#[command(
    name = "cosmolith",
    version,
    about = "Watch Cosmic configuration changes"
)]
pub struct Cli {
    /// Enable verbose output for configuration changes
    #[arg(short, long)]
    pub debug: bool,

    /// Optional list of namespaces to watch (defaults to all known namespaces)
    #[arg(
		value_name = "NAMESPACE",
		value_parser = PossibleValuesParser::new(DEFAULT_NAMESPACES),
		value_hint = ValueHint::Other,
		num_args = 0..,
	)]
    pub namespaces: Vec<String>,
}

impl Cli {
    /// Parse arguments and execute the selected command.
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let args = Cli::parse();
        debug::run_watcher(args.debug, &args.namespaces)
    }
}
