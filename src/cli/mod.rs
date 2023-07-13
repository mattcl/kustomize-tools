use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod replace_tag;

/// A collection of tools for manipulating kustomization files.
#[derive(Debug, Clone, Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Self::parse();

        cli.command.run()
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    ReplaceTag(replace_tag::ReplaceTag),
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::ReplaceTag(cmd) => cmd.run(),
        }
    }
}
