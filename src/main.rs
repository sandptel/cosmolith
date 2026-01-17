
// src/main.rs
mod cli;
mod debug;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::Cli::run()
}