use clap::Parser;

use crate::debug;

/// Command-line interface for cosmolith.
#[derive(Debug, Parser)]
#[command(name = "cosmolith", version, about = "Watch Cosmic configuration changes")]
pub struct Cli {
	/// Enable verbose output for configuration changes
	#[arg(short, long)]
	pub debug: bool,
}

impl Cli {
	/// Parse arguments and execute the selected command.
	pub fn run() -> Result<(), Box<dyn std::error::Error>> {
		let args = Cli::parse();
		debug::run_watcher(args.debug)
	}
}
