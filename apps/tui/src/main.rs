#[tokio::main]
async fn main() -> Result<(), snafu::Whatever> {
    tui::run().await
}
