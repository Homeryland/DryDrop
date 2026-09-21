use snafu::ResultExt;

use crate::app::App;

pub mod app;

pub async fn run() -> Result<(), snafu::Whatever> {
    ratatui::run(|terminal| App::default().run(terminal))
        .with_whatever_context(|_| "Could not run tui")?;
    Ok(())
}
