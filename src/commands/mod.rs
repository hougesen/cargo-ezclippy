use clap::Parser;

use crate::{
    cli::{Cli, EzClippyCommand},
    error::EzClippyError,
};

mod init;
mod schema;

#[inline]
pub fn execute_command() -> Result<(), EzClippyError> {
    match Cli::parse().command {
        EzClippyCommand::Init => init::run(),
        EzClippyCommand::Schema => schema::run(),
    }
}
