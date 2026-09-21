pub mod app;

pub fn run() -> Result<(), snafu::Whatever> {
    let args = app::Args::parse();
    args.process();
    Ok(())
}
