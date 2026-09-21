#[tokio::main]
async fn main() -> Result<(), snafu::Whatever> {
    server::run().await
}
