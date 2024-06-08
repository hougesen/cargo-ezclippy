use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: EzClippyCommand,
}

#[derive(Debug, Subcommand)]
pub enum EzClippyCommand {
    Init,

    Schema,
}
