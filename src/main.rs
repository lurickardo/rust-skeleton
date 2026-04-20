use std::io::Write;

use rust_skeleton::app::build_router;
use rust_skeleton::config::load_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let env = load_env();

    let _ = std::io::stdout().write_all(b"\x1B[2J\n\x1b[32mStarting server...\x1b[0m\n");

    let router = build_router(&env);

    let addr = format!("[::]:{}", env.app.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
