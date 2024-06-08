use commands::execute_command;

mod cli;
mod commands;
mod config;
mod lints;

fn main() {
    println!("Hello, world!");

    execute_command();
}
