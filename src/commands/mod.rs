use clap::Parser;

use crate::cli::{Cli, EzClippyCommand};

mod init;
mod schema;

pub fn execute_command() {
    match Cli::parse().command {
        EzClippyCommand::Init => init::run(),
        EzClippyCommand::Schema => schema::run(),
    }
}
