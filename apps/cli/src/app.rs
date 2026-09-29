use usage::{Cli, Subcommands};

use crate::commands::new::New;

#[derive(Cli)]
#[usage(bin = "drydrop", version = "0.0.1")]
pub struct Cli {
    #[usage(subcommand)]
    pub command: Commands,
}

#[derive(Subcommands)]
#[usage(run)]
pub enum Commands {
    New(New),
}
