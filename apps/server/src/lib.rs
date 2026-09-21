use snafu::ResultExt;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    application::state::AppState,
    infrastructure::config::Config,
    presentation::http::v1::{log::init_logger, routes::create_routers},
};

pub mod application;
pub mod infrastructure;
pub mod presentation;

pub async fn run() -> Result<(), snafu::Whatever> {
    let (file_writer, _file_guard) = init_logger();
    let file_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_writer);
    let console_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_writer(std::io::stdout);
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();
    info!("Initialized app logger successfully");

    let config = Config::load()?;
    let app_config = config.clone();
    info!("Initialized config successfully");

    let server_address = format!("{}:{}", config.server.host, config.server.port);

    let app_state = AppState::new(app_config)
        .await
        .with_whatever_context(|e| format!("Failed to initialize app state: {e}"))?;
    info!("Initialized app state successfully");

    let app = create_routers(app_state);
    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .with_whatever_context(|_| format!("Failed to bind to {}", server_address))?;
    info!("Server starting on http://{}", server_address);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .with_whatever_context(|_| "Could not run server")?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install terminate signal handler")
            .recv()
            .await
            .expect("Failed to receive terminate signal");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Shutdown signal received, stopping server...");
        },
        _ = terminate => {
            info!("Shutdown signal received, stopping server...");
        },
    }
}
