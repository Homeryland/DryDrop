use crate::app::Cli;
use usage::Run;

pub mod app;
pub mod commands;

pub fn run() -> Result<(), snafu::Whatever> {
    Cli::parse().command.run();
    Ok(())
}
