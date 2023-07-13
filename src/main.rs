use anyhow::Result;

pub mod cli;

fn main() -> Result<()> {
    cli::Cli::run()
}
