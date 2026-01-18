use clap::{Parser, ValueHint, builder::PossibleValuesParser};

use crate::{namespaces::DEFAULT_NAMESPACES, watcher};

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
        watcher::run_watcher(args.debug, &args.namespaces)
    }
}
