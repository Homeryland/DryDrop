use usage::Cli;

#[derive(Cli)]
#[usage(bin = "d", version = "0.0.1")]
pub struct Args {}

impl Args {
    pub fn process(&self) {}
}
