use commands::execute_command;

mod cli;
mod commands;
mod config;
mod error;
mod generated;
mod lints;

fn main() {
    if let Err(error) = execute_command() {
        eprintln!("ezclippy error: {error}");
    }
}
