// src/main.rs
use cosmolith::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::run()
}